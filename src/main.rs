use std::sync::Arc;

use essay_writer::{
    agent::{Agent, CritiqueAgent, PlanAgent, ResearchAgent, WriterAgent},
    io_utils::get_user_input,
    state::AgentState,
};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    println!("Essay Writer");
    println!();
    println!("Enter the topic of your essay:");
    let topic = get_user_input();
    println!();

    let state = Arc::new(RwLock::new(AgentState::new(topic, 2)));

    let revisions = { state.read().await.revision_number };

    let planner = PlanAgent::init(state.clone());
    let _plan = planner.execute().await;

    let researcher = ResearchAgent::init(state.clone());
    let _research = researcher.execute().await;

    let writer = WriterAgent::init(state.clone());
    let critique_agent = CritiqueAgent::init(state.clone());

    let draft = writer.execute().await;
    println!("Draft #1:");
    println!();
    println!("{}", draft);

    for i in 0..revisions {
        let critique = critique_agent.execute().await;
        println!("Critique #{}: ", i + 1);
        println!();
        println!("{}", critique);

        let draft = writer.execute().await;
        println!("Draft #{}:", i + 2);
        println!();
        println!("{}", draft);
    }
}
