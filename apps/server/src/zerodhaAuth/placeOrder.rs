use reqwest::Client;
use serde_json::json;

pub async fn place_order(
    api_key: &str,
    access_token: &str,
    decision: &str,
    quantity: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate input
    let txn_type_string = decision.to_lowercase();
    let txn_type = txn_type_string.as_str();

    let url = "https://api.kite.trade/orders/regular";

    let payload = json!({
        "exchange": "NSE",
        "tradingsymbol": "ONGC",
        "transaction_type": txn_type,
        "order_type": "MARKET",
        "quantity": quantity,
        "product": "CNC",
        "variety": "regular"
    });

    let client = Client::new();
    let res = client
        .post(url)
        .header("X-Kite-Version", "3")
        .header("Authorization", format!("token {}:{}", api_key, access_token))
        .form(&payload)
        .send()
        .await?;

    let status = res.status();
    let text = res.text().await?;
    if status.is_success() {
        println!("✅ Order placed: {}", text);
    } else {
        eprintln!("❌ Failed to place order: {}", text);
    }

    Ok(())
}
