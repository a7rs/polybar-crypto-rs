use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Coin {
    pub name: String,
    pub symbol: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Config {
    pub vs_currency: String,
    pub coins: Vec<Coin>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vs_currency: String::from("usd"),
            coins: vec![
                Coin {
                    name: String::from("bitcoin"),
                    symbol: String::from("BTC"),
                },
                Coin {
                    name: String::from("ethereum"),
                    symbol: String::from("ETH"),
                },
                Coin {
                    name: String::from("solana"),
                    symbol: String::from("SOL"),
                },
                Coin {
                    name: String::from("polkadot"),
                    symbol: String::from("DOT"),
                },
                Coin {
                    name: String::from("binancecoin"),
                    symbol: String::from("BNB"),
                },
            ],
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, io::Error> {
        let config_path = match Self::get_dir_path() {
            Some(dir) => Some(Self::get_file_path(dir)),
            _ => None,
        };

        if config_path.is_none() {
            if let Ok(path) = Self::choose_config_loc() {
                match Self::create_config_file(path) {
                    Ok(_) => (),
                    Err(_) => println!("Error create config file"),
                }
            }
        }

        Self::load_config_file(config_path.unwrap())
    }

    fn load_config_file(path: PathBuf) -> Result<Config, io::Error>{
        let file = File::open(path)?;
        let config: Config = serde_json::from_reader(file)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{:?}", err)))?;
        Ok(config)
    }

    fn choose_config_loc() -> Result<PathBuf, Box<dyn Error>> {
        let dir = match dirs::config_dir() {
            Some(mut dir) => {
                dir.push("polybar");
                fs::create_dir(&dir)?;
                dir
            },
            _ => {
                if let Some(dir) = dirs::home_dir() {
                    dir
                } else {
                    unimplemented!();
                }
            },
        };
        Ok(dir)
    }

    fn create_config_file(path: PathBuf) -> io::Result<()> {
        let path = Self::get_file_path(path);
        serde_json::to_writer(&File::create(path)?, &Config::default())?;
        Ok(())
    }

    fn get_file_path(mut dir: PathBuf) -> PathBuf {
        dir.push("coins");
        dir.set_extension("json");
        dir
    }

    fn get_dir_path() -> Option<PathBuf> {
        let config_dir = match dirs::config_dir() {
            Some(mut dir) => {
                dir.push("polybar");
                if dir.exists() { Some(dir) } else { None }
            },
            _ => None,
        };

        config_dir
    }
}
