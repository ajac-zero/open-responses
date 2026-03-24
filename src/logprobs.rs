use serde::{Deserialize, Serialize};

/// Log probability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogProb {
    pub token: String,
    pub logprob: f64,
    pub bytes: Vec<i64>,
}

/// Top log probability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopLogProb {
    pub token: String,
    pub logprob: f64,
    pub bytes: Vec<i64>,
}
