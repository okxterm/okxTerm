use std::time::Duration;

use anyhow::{Ok, anyhow};
use once_cell::sync::OnceCell;
use reqwest::{Client, ClientBuilder};

static HTTP_CLINET: OnceCell<reqwest::Client> = OnceCell::new();

/// 初始化 http 客户端
pub async fn init() -> anyhow::Result<()> {
    let clinet = ClientBuilder::new()
        .pool_idle_timeout(Some(Duration::from_secs(30)))
        .pool_max_idle_per_host(29)
        .connection_verbose(false)
        .build()?;
    self::HTTP_CLINET
        .set(clinet)
        .map_err(|_| anyhow!("Failed to set http client"))?;
    Ok(())
}

/// 获取 http client 客户端
pub async fn client() -> anyhow::Result<Client> {
    let client = self::HTTP_CLINET
        .get()
        .ok_or_else(|| anyhow!("Failed to get http client"))?;
    Ok(client.clone())
}
