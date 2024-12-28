#[derive(Debug)]
pub struct AgentState {
    pub task: String,
    pub plan: Option<String>,
    pub queries: Option<Vec<String>>,
    pub urls: Option<Vec<String>>,
    pub research: Option<Vec<String>>,
    pub draft: Option<String>,
    pub critique: Option<String>,
    pub content: Option<String>,
    pub revision_number: u32,
    pub max_revisions: u32,
}

impl AgentState {
    pub fn new(task: String, max_revisions: u32) -> Self {
        AgentState {
            task: task,
            plan: None,
            queries: None,
            urls: None,
            research: None,
            draft: None,
            critique: None,
            content: None,
            revision_number: max_revisions,
            max_revisions,
        }
    }
}
