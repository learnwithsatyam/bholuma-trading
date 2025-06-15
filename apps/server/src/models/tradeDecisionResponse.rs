use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct tradeDecisionResponse {
    pub decision: String, // "buy", "sell", or "hold"
    pub reason: String,  // Explanation for the decision
}