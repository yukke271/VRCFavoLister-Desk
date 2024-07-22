//use reqwest::{Url, Client, Response, cookie::Jar, header::{self, USER_AGENT, HeaderMap, HeaderValue}};
use reqwest::{Url, Client, cookie::Jar, header::{self, USER_AGENT, HeaderMap, HeaderValue}};
use std::{sync::Arc, time::Duration};

use crate::structs::apiconfig::APIConfig;

/*

    TODO:エラーハンドリングを実装する

    読み込んだ複数データの内処理に失敗したデータを返し、
    そもそもAPIの呼び出しができなかった場合はエラーコードを返す。

    最低要件:
    ログインしたアカウントがFavoriteしているワールドの一覧を取得・整形し、
    整形できなかったデータはそのまま抜き取り、別の変数に保管し、のちに返却する。
    整形できたデータはDBに格納後し、データの被りがある場合は新しくデータを更新する。

*/
#[tauri::command]
pub async fn load_favorite(_db_pool: tauri::State<'_, sqlx::SqlitePool>) -> Result<String, ()> { 

    // APIConfigを取得
    let api_config = APIConfig::new();    
    
    if api_config.two_factor_auth.is_none() {
        println!("DebugLog:2段階認証未設定");
        return Ok("2段階認証が未設定です".to_string()); 
    }

    let mut headers: HeaderMap  = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str(&api_config.user_agent.clone()).unwrap());
    headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
    headers.insert(header::ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));

    let cookie_jar = Arc::new(Jar::default());
    cookie_jar.add_cookie_str(&("apiKey=".to_owned() + &api_config.api_key.clone()), &Url::parse(&api_config.base_url).unwrap());
    // cookie_jar.add_cookie_str(&api_config.auth_cookie.clone().unwrap(), &Url::parse(&api_config.base_url).unwrap());
    cookie_jar.add_cookie_str(&api_config.two_factor_auth.clone().unwrap(), &Url::parse(&api_config.base_url).unwrap());
    
    // let mut params = HashMap::new();
    // params.insert("n", "200");
    // params.insert("offset", "0");

    let _client: Client  = reqwest::Client::builder()
        .default_headers(headers.clone())
        .cookie_provider(cookie_jar) 
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    // // リクエスト発行
    // println!("DebugLog:リクエスト発行");
    // let response1:Response = client
    //     .get(api_config.base_url.clone() + "worlds/favorites")
    //     .headers(headers.clone())
    //     .query(&[
    //         ("n", "200"), 
    //         ("offset", "0")
    //     ])
    //     .send()
    //     .await
    //     .unwrap();
    // let body = response1
    //     .text()
    //     .await
    //     .unwrap();
    // println!("Response is : \n{}", body);
    
    // // リクエスト発行
    // println!("DebugLog:リクエスト発行");
    // let response2:Response = client
    //     .get(api_config.base_url.clone() + "worlds/favorites")
    //     .headers(headers.clone())
    //     .query(&[
    //         ("n", "200"), 
    //         ("offset", "200")
    //     ])
    //     .send()
    //     .await
    //     .unwrap();  
    // let body = response2
    //     .text()
    //     .await
    //     .unwrap();
    // println!("Response is : \n{}", body);
    
    return Ok("処理の完了".to_string()); 
}

/* read favorite dbから読み込んだデータを返す */