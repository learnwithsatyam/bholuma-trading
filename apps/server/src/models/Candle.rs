pub struct Candle{
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

pub struct CandleResponse{
    pub status: String,
    pub data: CandlePayload
}

pub struct CandlePayload{
    pub candles: Vec<[serde_json::Value; 6]>,
}