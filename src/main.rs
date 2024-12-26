use std::sync::Arc;

use essay_writer::{
    agent::{Agent, PlanAgent, ResearchAgent},
    state::AgentState,
};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let state = Arc::new(RwLock::new(AgentState::new(
        "What is the impact of global warming?".to_string(),
        3,
    )));
    let planner = PlanAgent::init(state.clone());
    let plan = planner.execute().await;
    let researcher = ResearchAgent::init(state.clone());
    let research = researcher.execute().await;
    println!("Response:");
    println!();
    println!("{}", plan);

    println!("Research:");
    println!();
    println!("{}", research);
    dbg!(state);
}
