use config::Config;
use tracing::{error, info};

mod config;

#[tokio::main]
async fn main() {
    let set_config = Config::new().await;

    let current_ip = match reqwest::get("https://ifconfig.me/ip").await {
        Ok(response) => response.text().await.unwrap(),
        Err(err) => panic!("Failed to fetch IP address: {}", err),
    };

    if let Err(err) = update_ddns(set_config, current_ip).await {
        error!("DDNS update failed: {}", err);
    }
}

async fn update_ddns(current_config: Config, current_ip: String) -> Result<(), reqwest::Error> {
    if current_ip == current_config.last_ip {
        info!("No change in IP address: {}", current_ip);
        return Ok(());
    }

    info!("IP address changed: {}", current_ip);
    
    let update_url = format!(
        "https://www.duckdns.org/update?domains={}&token={}&ip={}",
        current_config.domain, current_config.token, current_ip
    );

    let client = reqwest::Client::new();
    let response = client.get(&update_url).send().await?;

    match response.status().as_u16() {
        200 => info!("DDNS update successful!"),
        _ => error!("DDNS update failed: {}", response.status()),
    }

    current_config.update_last_ip().expect("Failed to update last IP");

    Ok(())
}
