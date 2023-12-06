use rand::seq::SliceRandom;
use std::path::Path;

use super::config::read_config;

pub async fn download_reply_list() {
    let reply_list_url = read_config().await.reply_list_url;

    let reply_list = reqwest::get(reply_list_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    std::fs::write("list", reply_list).unwrap();
}

pub async fn choose_reply_from_list() -> String {
    if !Path::new("list").exists() {
        download_reply_list().await;
    }

    let reply_list: Vec<String> = std::fs::read_to_string("list")
        .unwrap()
        .split('\n')
        .map(|x| x.to_string())
        .collect();

    reply_list
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}
