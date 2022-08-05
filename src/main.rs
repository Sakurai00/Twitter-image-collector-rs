use anyhow::Result;
use dotenvy::dotenv;
use std::env;
use std::fs;
use std::path::Path;
use twitter_v2::query::MediaField;
use twitter_v2::query::TweetField;
use twitter_v2::{authorization::BearerToken, TwitterApi};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let token = env::var("TW_BEARER_TOKEN").unwrap();
    let auth = BearerToken::new(token);
    let api = TwitterApi::new(auth);

    let search_str = "#アークナイツ";

    // make dir
    let path = Path::new("./images").join(search_str);
    println!("{:#?}", path);
    if !path.exists() {
        fs::create_dir_all(path).expect("create dir error");
    }

    // search
    // Twitter-v2ではminFavの指定が出来ないらしい；；
    // https://developer.twitter.com/en/docs/twitter-api/tweets/search/migrate
    //TODO eggmodeで書くしかなさそう．v1は仕様そんなに重くないから行ける気がする
    let query = String::from(search_str) + " has:images -is:retweet";
    println!("query: {}", query);
    let tweets = api
        .get_tweets_search_recent(query)
        .max_results(20)
        .tweet_fields([TweetField::Entities, TweetField::Text])
        .media_fields([MediaField::Url, MediaField::Type])
        .send()
        .await?;
    let tweets = tweets.into_data();
    println!("{:#?}", tweets);

    //? Optionのネスト深すぎんか
    // if let Some(ts) = tweets {
    //     for t in ts {
    //         for url in t.entities.expect("a").urls.expect("msg") {
    //             for i in url.images.unwrap() {
    //                 println!("{}", i.url);
    //             }
    //         }
    //     }
    // }

    // download

    Ok(())
}
