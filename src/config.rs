use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
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
                Coin {
                    name: String::from("avalanche"),
                    symbol: String::from("AVAX"),
                },
                Coin {
                    name: String::from("luna"),
                    symbol: String::from("LUNA"),
                },
                Coin {
                    name: String::from("fantom"),
                    symbol: String::from("FTM"),
                },
                Coin {
                    name: String::from("near"),
                    symbol: String::from("NEAR"),
                },
            ],
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, io::Error> {
        // Get config path and append file path if exists
        let config_path = match Self::get_dir_path() {
            Some(dir) => Some(Self::get_file_path(dir)),
            _ => None,
        };

        // Load config file if path exists else create config file
        match config_path {
            Some(path) => match path.exists() {
                true => Self::load_config_file(&path),
                false => match Self::create_config_file(&path) {
                    Ok(_) => Self::load_config_file(&path),
                    Err(e) => Err(e),
                },
            },
            None => match Self::choose_config_loc() {
                Ok(path) => match Self::create_config_file(&Self::get_file_path(path.clone())) {
                    Ok(_) => Self::load_config_file(&path),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
        }
    }

    fn load_config_file(path: &PathBuf) -> Result<Config, io::Error> {
        let file = File::open(path)?;
        let config: Config = serde_json::from_reader(file)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{:?}", err)))?;
        Ok(config)
    }

    // Attempts to create config directory if $XDG_CONFIG_HOME exists
    // Otherwise just returns $HOME
    fn choose_config_loc() -> Result<PathBuf, io::Error> {
        let dir = match dirs::config_dir() {
            Some(mut dir) => {
                dir.push("polybar");
                fs::create_dir(&dir)?;
                dir
            }
            _ => dirs::home_dir()
                .ok_or("Error resolving home directory")
                .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{:?}", err)))?,
        };
        Ok(dir)
    }

    fn create_config_file(path: &PathBuf) -> io::Result<()> {
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
                if dir.exists() {
                    Some(dir)
                } else {
                    None
                }
            }
            _ => None,
        };
        config_dir
    }
}
