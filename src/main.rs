use std::error::Error;

use crate::config::{Coin, Config};
mod config;

struct CoinData {
    coin: Coin,
    price: f64,
    change: f64,
}

#[tokio::main]
async fn get_data(config: Config) -> Result<Vec<CoinData>, Box<dyn Error>> {
    let coin_ids = config.coins
        .iter()
        .map(|c| format!("{}%2C", c.name))
        .collect::<String>();

    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}&include_24hr_change=true",
        config.vs_currency,
        &coin_ids[..coin_ids.len() - 3],
    );

    let resp = reqwest::get(&url)
        .await?
        .json::<serde_json::Value>()
        .await?;

    let mut coin_data: Vec<CoinData> = Vec::with_capacity(config.coins.len());
    for coin in config.coins {
        coin_data.push(
            CoinData {
                coin: coin.clone(),
                price: resp[&coin.name][&config.vs_currency]
                    .to_string()
                    .parse::<f64>()?,
                change: resp[&coin.name][&(format!("{}_24h_change", &config.vs_currency))]
                    .to_string()
                    .parse::<f64>()?,
            }
        );
    }

    Ok(coin_data)
}

fn main() {
    let config: Config = Config::load().unwrap_or_default();
    let mut output: Vec<String> = Vec::with_capacity(config.coins.len());

    let coin_data: Vec<CoinData> = match get_data(config) {
        Ok(data) => data,
        Err(_) => return (),
    };

    for data in coin_data {
        let change = if data.change > 0f64 {
            format!("%{{F#21cf5f}}+{:.2}%%{{F-}}", data.change)
        } else {
            format!("%{{F#ff004b}}{:.2}%%{{F-}}", data.change)
        };
        output.push(format!("{}: ${}/{} // ", data.coin.symbol , data.price, change));
    }
    let output = output.into_iter().collect::<String>();
    println!("{}", &output[..output.len() - 4]);
}
