use dotenv::dotenv;
use std::env;
use std::fs;
use tracing::Level;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::fmt::writer::MakeWriterExt;

/// Represents the configuration settings for the duck_updater application.
pub struct Config {
    /// The domain for DuckDNS.
    pub domain: String,
    /// The token for DuckDNS.
    pub token: String,
    /// The current IP address.
    pub current_ip: String,
    /// The last recorded IP address.
    pub last_ip: String,
    /// The location of the log files.
    pub log_location: String,
}

impl Config {
    /// Represents the configuration settings for the duck updater.
    /// 
    /// # Fields
    /// 
    /// * `domain` - The domain for DuckDNS.
    /// * `token` - The token for DuckDNS.
    /// * `current_ip` - The current IP address.
    /// * `last_ip` - The last recorded IP address.
    /// * `log_location` - The location of the log files.
    pub async fn new() -> Self {
        dotenv().ok();
        let config = Self {
            domain: env::var("DDNS_DOMAIN").expect("DDNS_DOMAIN must be set"),
            token: env::var("DDNS_TOKEN").expect("DDNS_TOKEN must be set"),
            current_ip: self::Config::get_current_ip().await.expect("Failed to fetch current IP"),
            last_ip: Config::get_last_ip().expect("Failed to read last IP"),
            log_location: env::var("LOG_LOCATION").unwrap_or_else(|_| "logs".to_string()),
        };

        config.setup_logging().expect("Failed to set up logging");
        config
    }

    pub async fn get_current_ip() -> Result<String, reqwest::Error> {
        let response = reqwest::get("https://ifconfig.me/ip").await?;
        let ip = response.text().await?;
        Ok(ip.trim().to_string())
    }

    pub fn get_last_ip() -> Result<String, std::io::Error> {
        match fs::read_to_string("last_ip.txt") {
            Ok(ip) => Ok(ip.trim().to_string()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                Ok(String::new())
            }
            Err(err) => Err(err),
        }
    }

    fn setup_logging(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file_appender = RollingFileAppender::new(Rotation::NEVER, &self.log_location, "duck_updater.log");

        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .with_writer(file_appender.and(std::io::stdout))
            .finish();

        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

        Ok(())
    }

    pub fn update_last_ip(&self) -> Result<(), std::io::Error> {
        fs::write("last_ip.txt", &self.current_ip)
    }
}