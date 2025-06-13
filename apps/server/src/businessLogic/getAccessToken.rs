use reqwest::Client;
mod TokenResponse;

async fn get_access_token(
    api_key: &str,
    request_token: &str,
    api_secret: &str,
) -> Result<TokenResponse, reqwest::Error> {
    let client = Client::new();

    let checksum = generate_checksum(api_key, request_token, api_secret);

    let params = [
        ("api_key", api_key),
        ("request_token", request_token),
        ("checksum", &checksum),
    ];

    let res = client
        .post("https://api.kite.trade/session/token")
        .header("X-Kite-Version", "3")
        .form(&params)
        .send()
        .await?
        .json::<TokenResponse>()
        .await?;

    Ok(res)
}