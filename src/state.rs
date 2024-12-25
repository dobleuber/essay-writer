#[derive(Debug)]
pub struct AgentState {
    pub task: String,
    pub plan: Option<String>,
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
            draft: None,
            critique: None,
            content: None,
            revision_number: 0,
            max_revisions,
        }
    }
}
