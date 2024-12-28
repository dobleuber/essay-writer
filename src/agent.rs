use futures::stream::{self, StreamExt};
use langchain_rust::{
    chain::{Chain, LLMChain, LLMChainBuilder},
    fmt_message, fmt_template,
    llm::openai::{OpenAI, OpenAIModel},
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::messages::Message,
    template_fstring,
};
use std::env::var;
use std::{future::Future, sync::Arc};
use tavily::Tavily;
use tokio::sync::RwLock;

use super::state::AgentState;
use crate::prompts::{
    PLAN_PROMPT, REFLECTION_INPUT_PROMPT, REFLECTION_PROMPT, RESEARCH_PLAN_PROMPT,
    WRITER_INPUT_PROMPT, WRITER_PROMPT,
};

type State = Arc<RwLock<AgentState>>;

type TavilyShared = Arc<Tavily>;

pub trait Agent {
    fn init(state: State) -> Self;
    fn execute(&self) -> impl Future<Output = String> + Send;
}

pub struct PlanAgent {
    chain: LLMChain,
    state: State,
}

impl Agent for PlanAgent {
    fn init(state: Arc<RwLock<AgentState>>) -> Self {
        let llm = OpenAI::default().with_model(OpenAIModel::Gpt4oMini);
        let prompt = message_formatter![
            fmt_message!(Message::new_system_message(PLAN_PROMPT)),
            fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
                "{input}", "input"
            ))),
        ];

        let chain = LLMChainBuilder::new()
            .prompt(prompt)
            .llm(llm)
            .build()
            .expect("Failed to create chain");

        Self {
            chain,
            state: state.clone(),
        }
    }

    async fn execute(&self) -> String {
        let response = {
            let state = self.state.read().await;
            let task = state.task.clone();
            let messages = vec![Message::new_human_message(task)];
            let input = prompt_args! {
                "input" => messages
            };

            let response = self.chain.invoke(input).await;

            response.expect("Failed to get response")
        };
        let mut state = self.state.write().await;
        state.plan = Some(response.clone());
        response
    }
}

pub struct ResearchAgent {
    chain: LLMChain,
    tavily: Option<TavilyShared>,
    state: State,
}

impl Agent for ResearchAgent {
    fn init(state: Arc<RwLock<AgentState>>) -> Self {
        let llm = OpenAI::default().with_model(OpenAIModel::Gpt4);
        let tavily_api_key = var("TAVILY_API_KEY").expect("TAVILY_API_KEY not set");
        let tavily = Tavily::builder(&tavily_api_key)
            .build()
            .expect("Failed to build Tavily");
        let prompt = message_formatter![
            fmt_message!(Message::new_system_message(RESEARCH_PLAN_PROMPT)),
            fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
                "{input}", "input"
            ))),
        ];

        let chain = LLMChainBuilder::new()
            .prompt(prompt)
            .llm(llm)
            .build()
            .expect("Failed to create chain");

        Self {
            chain,
            state: state.clone(),
            tavily: Some(Arc::new(tavily)),
        }
    }

    async fn execute(&self) -> String {
        let queries = {
            let state = self.state.read().await;
            let plan = state.plan.clone().unwrap();
            let messages = vec![Message::new_human_message(plan)];
            let input = prompt_args! {
                "input" => messages
            };
            let queries = self.chain.invoke(input).await;
            queries.expect("Failed to get queries")
        };

        let queries: Vec<String> = queries.lines().map(String::from).collect();
        let tavily = self.tavily.as_ref().unwrap().clone();

        let research: Vec<_> = stream::iter(queries.clone())
            .map(|query| {
                let tavily = tavily.clone();
                async move {
                    let result = tavily.search(&query).await;
                    result
                }
            })
            .buffer_unordered(5)
            .collect::<Vec<_>>()
            .await;

        let research = research
            .into_iter()
            .flat_map(|result| {
                result
                    .unwrap()
                    .results
                    .into_iter()
                    .map(|result| (result.content, result.url))
            })
            .collect::<Vec<(String, String)>>();

        let mut state = self.state.write().await;
        state.queries = Some(queries.clone());
        let research_result: Vec<String> = research
            .iter()
            .map(|(content, _)| content)
            .cloned()
            .collect();
        state.research = Some(research_result.clone());

        state.urls = Some(research.iter().map(|(_, url)| url).cloned().collect());

        research_result.join("\n")
    }
}

pub struct WriterAgent {
    chain: LLMChain,
    state: State,
}

impl Agent for WriterAgent {
    fn init(state: Arc<RwLock<AgentState>>) -> Self {
        let llm = OpenAI::default().with_model(OpenAIModel::Gpt4);
        let prompt = message_formatter![
            fmt_message!(Message::new_system_message(WRITER_PROMPT)),
            fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
                WRITER_INPUT_PROMPT,
                "topic",
                "plan",
                "research",
                "critique",
                "draft",
                "references"
            ))),
        ];

        let chain = LLMChainBuilder::new()
            .prompt(prompt)
            .llm(llm)
            .build()
            .expect("Failed to create chain");

        Self {
            chain,
            state: state.clone(),
        }
    }

    async fn execute(&self) -> String {
        let draft = {
            let state = self.state.read().await;
            let input_variables = prompt_args! {
                "topic" => state.task.clone(),
                "plan" => state.plan.clone().expect("Plan not found"),
                "research" => state.research.clone().expect("Research not found"),
                "critique" => state.critique.clone(),
                "draft" => state.draft.clone(),
                "references" => state.urls.clone()
            };
            let queries = self.chain.invoke(input_variables).await;
            queries.expect("Failed to generate draft")
        };

        let mut state = self.state.write().await;
        state.draft = Some(draft.clone());
        draft
    }
}

pub struct CritiqueAgent {
    chain: LLMChain,
    state: State,
}

impl Agent for CritiqueAgent {
    fn init(state: Arc<RwLock<AgentState>>) -> Self {
        let llm = OpenAI::default().with_model(OpenAIModel::Gpt4);
        let prompt = message_formatter![
            fmt_message!(Message::new_system_message(REFLECTION_PROMPT)),
            fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
                REFLECTION_INPUT_PROMPT,
                "topic",
                "plan",
                "draft",
            ))),
        ];

        let chain = LLMChainBuilder::new()
            .prompt(prompt)
            .llm(llm)
            .build()
            .expect("Failed to create chain");

        Self {
            chain,
            state: state.clone(),
        }
    }

    async fn execute(&self) -> String {
        let critique = {
            let state = self.state.read().await;
            let input_variables = prompt_args! {
                "topic" => state.task.clone(),
                "plan" => state.plan.clone().expect("Plan not found"),
                "draft" => state.draft.clone(),
            };
            let critique = self.chain.invoke(input_variables).await;
            critique.expect("Failed to get critique")
        };
        let mut state = self.state.write().await;
        state.critique = Some(critique.clone());
        critique
    }
}
