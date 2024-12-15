use reqwest::Error;

pub async fn fetch_utxos(address: &str) -> Result<Vec<String>, Error> {
    let url = format!("https://blockstream.info/api/address/{}/utxo", address);
    let response = reqwest::get(&url).await?.json::<Vec<String>>().await?;
    Ok(response)
}

