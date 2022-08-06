use anyhow::Result;
use dotenvy::dotenv;
use egg_mode::entities::MediaType;
use egg_mode::search;
use egg_mode::search::ResultType;
use std::env;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let token = egg_mode::Token::Bearer(env::var("TW_BEARER_TOKEN").unwrap());

    let search_str = "#アークナイツ";

    // make dir
    let path = Path::new("./images").join(search_str);
    println!("{:#?}", path);
    if !path.exists() {
        fs::create_dir_all(path).expect("create dir error");
    }

    // search
    let query = String::from(search_str) + " filter:images exclude:retweets min_faves:3000";
    println!("query: {}", query);
    let tweets = search::search(query)
        .result_type(ResultType::Recent)
        .count(100)
        .call(&token)
        .await?;

    for tweet in &tweets.statuses {
        if let Some(media) = &tweet.extended_entities {
            for m in &media.media {
                if m.media_type == MediaType::Photo {
                    println!("{}", m.media_url_https);
                }
            }
        }
    }

    // download

    Ok(())
}
