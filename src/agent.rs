use anyhow::Result;
use langchain_rust::{
    chain::{Chain, LLMChain, LLMChainBuilder},
    fmt_message,
    llm::openai::{OpenAI, OpenAIConfig, OpenAIModel},
    message_formatter, prompt_args,
    schemas::messages::Message,
};
use std::{future::Future, sync::Arc};
use tokio::sync::RwLock;

use crate::prompts::{PLAN_PROMPT, RESEARCH_CRITIQUE_PROMPT, RESEARCH_PLAN_PROMPT};

use tavily::Tavily;

use super::state::AgentState;

type State = Arc<RwLock<AgentState>>;

pub trait Agent {
    fn init(state: State) -> Self;
    fn execute(&self) -> impl Future<Output = String> + Send;
}

pub struct PlanAgent {
    chain: LLMChain,
    state: State,
}

struct Query {
    query: String,
    notes: String,
}

impl Agent for PlanAgent {
    fn init(state: Arc<RwLock<AgentState>>) -> Self {
        let llm = OpenAI::default().with_model(OpenAIModel::Gpt4);
        let prompt = message_formatter![fmt_message!(Message::new_system_message(PLAN_PROMPT)),];

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
    state: State,
}

impl Agent for ResearchAgent {
    fn init(state: State) -> Self {
        todo!()
    }

    async fn execute(&self) -> String {
        todo!()
    }
}

impl ResearchAgent {
    fn format_plan(plan: &str) -> Vec<Query> {
        todo!()
    }
}
