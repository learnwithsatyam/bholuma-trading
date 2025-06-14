use std::env::var;
use serde::{Serialize, Deserialize};
use reqwest::{Client};
use time::{OffsetDateTime, UtcOffset};
use time::format_description::well_known::Rfc3339;


pub async fn fetch_100_candles(token: &str, instrument_token: &str) -> Result<Vec<Candle>, reqwest::Error> {
    let ist_offset = UtcOffset::from_hms(5, 30, 0).unwrap();
    let now = OffsetDateTime::now_utc().to_offset(ist_offset);
    let to = now.format(&Rfc3339).unwrap();
    let from = (now - time::Duration::minutes(3 * 100)).format(&Rfc3339).unwrap();

    let url = format!(
        "https://api.kite.trade/instruments/historical/{}/3minute?from={}&to={}",
        instrument_token, from, to
    );

    let res = reqwest::Client::new()
        .get(&url)
        .header("Authorization", format!(
            "token {}:{}",
            var("ZERODHA_API_KEY").unwrap_or_default(),
            token
        ))
        .send()
        .await?
        .json::<HistoricalResponse>()
        .await?;

    Ok(res.data.candles)
}
#[derive(Deserialize, Debug)]
struct HistoricalResponse {
    data: CandleData,
}

#[derive(Deserialize, Debug)]
struct CandleData {
    candles: Vec<Candle>,
}

type Candle = [serde_json::Value; 6]; // Or create a struct if you want to type fields
