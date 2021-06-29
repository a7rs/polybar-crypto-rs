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
    // Formats array of coins for API request
    let coin_ids = config
        .coins
        .iter()
        .map(|c| format!("{}%2C", c.name))
        .collect::<String>();

    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}&include_24hr_change=true",
        &coin_ids[..coin_ids.len() - 3],
        config.vs_currency,
    );

    let resp = reqwest::get(&url)
        .await?
        .json::<serde_json::Value>()
        .await?;

    let mut coin_data: Vec<CoinData> = Vec::with_capacity(config.coins.len());
    for coin in config.coins {
        coin_data.push(CoinData {
            price: resp[&coin.name][&config.vs_currency]
                .to_string()
                .parse::<f64>()?,
            change: resp[&coin.name][&(format!("{}_24h_change", &config.vs_currency))]
                .to_string()
                .parse::<f64>()?,
            coin,
        });
    }

    Ok(coin_data)
}

fn main() {
    let config: Config = Config::load().unwrap_or_default();
    let mut output: Vec<String> = Vec::with_capacity(config.coins.len());

    let coin_data: Vec<CoinData> = match get_data(config) {
        Ok(data) => data,
        Err(_) => {
            println!("Failed to get data");
            return ();
        }
    };

    for data in coin_data {
        let change = if data.change > 0f64 {
            // Format % change green if positive
            format!("%{{F#21cf5f}}+{:.2}%%{{F-}}", data.change)
        } else {
            // Format % change to red if negative
            format!("%{{F#ff004b}}{:.2}%%{{F-}}", data.change)
        };
        output.push(format!(
            "{}: ${}/{} // ",
            data.coin.symbol, data.price, change
        ));
    }
    let output = output.into_iter().collect::<String>();
    println!("{}", &output[..output.len() - 4]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_data() {
        let config = Config::default();
        let data: Vec<CoinData> = match get_data(Config::default()) {
            Ok(data) => data,
            Err(_) => panic!(),
        };

        for i in 0..data.len() {
            assert_eq!(config.coins[i].name, data[i].coin.name);
        }
    }
}
