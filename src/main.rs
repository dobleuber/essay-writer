use std::sync::Arc;

use essay_writer::{
    agent::{Agent, PlanAgent},
    state::AgentState,
};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let state = Arc::new(RwLock::new(AgentState::new(
        "what is the difference between langchain and langsmith".to_string(),
        3,
    )));
    let agent = PlanAgent::init(state.clone());
    let response = agent.execute().await;
    println!("Response:");
    println!();
    println!("{}", response);
    dbg!(state);
}
