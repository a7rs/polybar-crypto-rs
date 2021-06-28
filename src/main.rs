use std::error::Error;

const DEFAULT: [(&str, &str); 5] = [("bitcoin", "BTC"), ("ethereum", "ETH"), ("polkadot", "DOT"), ("solana", "SOL"), ("binancecoin", "BNB")];
const CMP: &str = "usd";

#[tokio::main]
async fn get_data(coin: &str, cmp: &str) -> Result<(f64, f64), Box<dyn Error>> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&include_24hr_change=true",
        coin,
    );

    let resp = reqwest::get(&url)
        .await?
        .json::<serde_json::Value>()
        .await?;

    let pi: &str = &(cmp.to_string() + "_24h_change");
    let price = resp[coin][cmp].to_string().parse::<f64>()?;
    let performance = resp[coin][pi].to_string().parse::<f64>()?;

    Ok((price, performance))
}

fn main() {
    let mut output: Vec<String> = Vec::with_capacity(DEFAULT.len());
    for (coin, ticker) in DEFAULT {
        let (price, performance) = get_data(coin, CMP).unwrap();
        let performance = if performance > 0f64 {
            format!("%{{F#21cf5f}}+{:.2}%%{{F-}}", performance)
        } else {
            format!("%{{F#ff004b}}{:.2}%%{{F-}}", performance)
        };
        output.push(format!("{}: ${}/{} // ", ticker, price, performance));
    }
    let output = output.into_iter().collect::<String>();
    println!("{}", &output[..output.len() - 4]);
}
