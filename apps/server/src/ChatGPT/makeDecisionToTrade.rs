use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use std::env;
use crate::models::tradeDecisionResponse::tradeDecisionResponse;
// use std::error::Error;
pub async fn makeDecisionToTrade(candle_values: Vec<Value>) -> Result<tradeDecisionResponse, Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let prompt = format!(
        "Make the decision to buy, sell or hold the stock based on the given 100 candle data for the market right now. Only give me a JSON as output with fields as:\n\n{{\n    \"decision\": \"Buy\" | \"Sell\" | \"Hold\",\n    \"reason\": \"explain your reasoning behind the decision\"\n}}\n\nAnd here is the 100 candle data: {} \n\n And the order or data for each candle is as follows: [timestamp, open, high, low, close, volume].\n\n",
        candle_values.iter()
            .map(|candle| candle.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    let body = json!({
        "model": "gpt-4o",
        "messages": [
            {"role": "user", "content": prompt}
        ],
        "temperature": 0.3
    });

    let res = Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?;

    let text = res.text().await?;

    let json_part = text
    .lines()
    .find(|line| line.trim_start().starts_with('{'))
    .unwrap_or("{}");

    let parsed: tradeDecisionResponse = serde_json::from_str(json_part)?;

    Ok(parsed)
}