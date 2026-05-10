use crate::{
    consts::supported_network_enum::SupportedNetwork,
    dependencies::context::CoreContext,
};
use anyhow::{Result, anyhow, bail};
use reqwest::Client;
use serde::Deserialize;
use tracing::{debug, info, warn};

pub enum SupraNetwork {
    Mainnet,
    Testnet,
}

#[derive(Deserialize, Debug)]
struct SupraInstrument {
    #[serde(rename = "currentPrice")]
    pub current_price: String,
}

#[derive(Deserialize, Debug)]
struct SupraLatestResponse {
    pub instruments: Vec<SupraInstrument>,
}

fn map_network_to_supra_symbol(network: SupportedNetwork) -> &'static str {
    match network {
        SupportedNetwork::BTC => "btc",
        SupportedNetwork::HEDERA => "hbar",
        SupportedNetwork::ETH => "eth",
        SupportedNetwork::SOLANA => "sol",
    }
}

async fn fetch_supra_price(
    client: &Client,
    base_url: &str,
    api_key: &str,
    pair: &str,
) -> Result<f64> {
    let url = format!(
        "{}/latest?trading_pair={}",
        base_url.trim_end_matches('/'),
        pair
    );

    debug!("Fetching price from Supra for pair: {}", pair);

    let response = client
        .get(&url)
        .header("x-api-key", api_key)
        .send()
        .await?
        .json::<SupraLatestResponse>()
        .await?;

    let instrument = response.instruments.first().ok_or_else(|| {
        warn!("No price data found in Supra response for pair {}", pair);
        anyhow!("No price data found for pair {}", pair)
    })?;

    let price: f64 = instrument.current_price.parse().map_err(|_| {
        warn!(
            "Failed to parse currentPrice '{}' as f64 for {}",
            instrument.current_price, pair
        );
        anyhow!("Failed to parse currentPrice as f64 for {}", pair)
    })?;

    if price == 0.0 {
        warn!("Price returned as zero for pair {}", pair);
        bail!("Price returned as zero for pair {}", pair);
    }

    debug!("Successfully fetched price for {}: {}", pair, price);

    Ok(price)
}

pub async fn get_token_price_vs_btc(
    ctx: &CoreContext,
    network: SupportedNetwork,
    env: Option<SupraNetwork>,
) -> Result<f64> {
    let env = env.unwrap_or(SupraNetwork::Testnet);

    let (base_url, api_key) = match env {
        SupraNetwork::Mainnet => {
            (&ctx.cfg.supra.api_url_mainnet, &ctx.cfg.supra.api_key_mainnet)
        },
        SupraNetwork::Testnet => {
            (&ctx.cfg.supra.api_url_testnet, &ctx.cfg.supra.api_key_testnet)
        },
    };

    let client = Client::new();
    let symbol = map_network_to_supra_symbol(network);
    let pair_token = format!("{}_usdt", symbol);
    let pair_btc = "btc_usdt".to_string();

    info!(
        "Starting price fetch for token vs BTC. Pairs: {} and {}",
        pair_token, pair_btc
    );

    let (token_usd, btc_usd) = tokio::try_join!(
        fetch_supra_price(&client, base_url, api_key, &pair_token),
        fetch_supra_price(&client, base_url, api_key, &pair_btc)
    )?;

    let ratio = token_usd / btc_usd;

    Ok(ratio)
}

pub async fn get_token_value_in_sats(
    ctx: &CoreContext,
    network: SupportedNetwork,
    env: Option<SupraNetwork>,
) -> Result<u64> {
    let ratio = get_token_price_vs_btc(ctx, network, env).await?;
    let sats = (ratio * 100_000_000.0).round() as u64;

    info!("Converted token ratio {} to Satoshi value: {} sats", ratio, sats);

    Ok(sats)
}
