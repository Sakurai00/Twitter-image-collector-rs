use anyhow::Result;
use dotenvy::dotenv;
use egg_mode::entities::MediaType;
use egg_mode::search;
use egg_mode::search::ResultType;
use egg_mode::Token;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{env, fs};
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let token = Token::Bearer(env::var("TW_BEARER_TOKEN").unwrap());
    let config = get_config()?;

    for query in &config.queries {
        set_dir(query)?;
        search_tweets(&token, query, config.minfav).await?;
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    minfav: i64,
    queries: Vec<String>,
}

fn get_config() -> Result<Config> {
    let file = File::open("./config.json").expect("config.json open error");
    let reader = BufReader::new(file);

    let config: Config = serde_json::from_reader(reader).expect("json parse error");
    Ok(config)
}

fn set_dir(search_str: &str) -> Result<()> {
    let current_exe_dir = env::current_exe()?;
    let base_dir = Path::new(&current_exe_dir).parent().unwrap();
    let path = base_dir.join("images").join(search_str);

    if !path.exists() {
        fs::create_dir_all(&path).expect("create dir error");
    }

    env::set_current_dir(&path)?;

    Ok(())
}

async fn search_tweets(token: &Token, search_str: &str, minfav: i64) -> Result<()> {
    let query = String::from(search_str)
        + " filter:images exclude:retweets min_faves:"
        + &minfav.to_string();
    println!("query: {}", query);

    let tweets = search::search(query)
        .result_type(ResultType::Recent)
        .count(100)
        .call(token)
        .await?;

    for tweet in &tweets.statuses {
        if let Some(media) = &tweet.extended_entities {
            for m in &media.media {
                if m.media_type == MediaType::Photo {
                    let url = Url::parse(&m.media_url_https)?;
                    download_image(url).await?;
                }
            }
        }
    }

    Ok(())
}

async fn download_image(url: Url) -> Result<()> {
    let mut path_seg = url.path_segments().expect("Url parse error");
    path_seg.next().expect("Url parse error. /media not found");
    let filename = path_seg.next().expect("Url parse error. image not found");
    let url = url.to_string() + ":orig";

    if Path::new(filename).exists() {
        println!("pass");
    } else {
        println!("url: {}", url);
        let mut file = File::create(filename).expect("Image file create error");
        reqwest::blocking::get(url)
            .expect("Download error")
            .copy_to(&mut file)
            .expect("Image copy error");
    }

    Ok(())
}
