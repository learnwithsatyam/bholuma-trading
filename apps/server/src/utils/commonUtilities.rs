use std::fs::File;
use std::io::Write;

mod TokenResponse;

pub fn save_to_file(data: &TokenResponse, filename: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(data)?; // pretty or just `to_string`
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn read_from_file(filename: &str) -> std::io::Result<TokenResponse> {
    let json = fs::read_to_string(filename)?;
    let user: User = serde_json::from_str(&json)?;
    Ok(user)
}
