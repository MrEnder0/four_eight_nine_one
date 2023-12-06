use std::fs::File;

use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
};
use scorched::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub config_version: u8,
    pub token: String,
    pub reply_list_url: String,
    pub target_count_range: (u8, u8),
}

pub static CONFIG_VERSION: u8 = 1;

pub async fn read_config() -> Config {
    let config_file =
        File::open("config.ron").log_expect(LogImportance::Error, "Config file not found");
    let config: Config = match from_reader(config_file) {
        Ok(x) => x,
        Err(e) => {
            log_this(LogData {
                importance: LogImportance::Error,
                message: format!(
                    "Unable to read config file because of the following error:\n{}",
                    e
                ),
            })
            .await;

            std::process::exit(0);
        }
    };

    config
}

pub async fn gen_config() {
    let data = Config {
        config_version: CONFIG_VERSION,
        token: "token".to_string(),
        reply_list_url: "https://raw.githubusercontent.com/retardicator/4891/main/list".to_string(),
        target_count_range: (25, 200),
    };

    let config = PrettyConfig::new()
        .depth_limit(3)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let config_str = to_string_pretty(&data, config).expect("Serialization failed");
    std::fs::write("config.ron", config_str).unwrap();

    log_this(LogData {
        importance: LogImportance::Info,
        message: "Config file has been generated.".to_string(),
    })
    .await;
}

pub fn non_async_read_config() -> Config {
    let config_file = File::open("config.ron").expect("Config file not found");
    let config: Config = match from_reader(config_file) {
        Ok(x) => x,
        Err(e) => {
            println!(
                "Unable to read config file because of the following error:\n{}",
                e
            );

            std::process::exit(0);
        }
    };

    config
}
