use reqwest::{Url, Client, Response, cookie::Jar, header::{self, USER_AGENT, HeaderMap, HeaderValue}};
use std::{sync::Arc, time::Duration};

use crate::structs::app_state::{AppState, ContextTrait};
use crate::structs::favorite_world::FavoriteWorldFromAPI;

use crate::commands::database::{ insert_favorite_world, select_favorite_world };

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
pub async fn load_favorite(app_state: tauri::State<'_, AppState>) -> Result<String, ()> { 

    // APIConfigを取得
    let api_config = app_state.context.lock().unwrap().get_api_config();
    // DBPoolを取得
    let db_pool = app_state.context.lock().unwrap().db_pool.clone();
    
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
    cookie_jar.add_cookie_str(&api_config.auth_cookie.clone().unwrap(), &Url::parse(&api_config.base_url).unwrap());
    cookie_jar.add_cookie_str(&api_config.two_factor_auth.clone().unwrap(), &Url::parse(&api_config.base_url).unwrap());
    
    // let mut params = HashMap::new();
    // params.insert("n", "200");
    // params.insert("offset", "0");

    let client: Client  = reqwest::Client::builder()
        .default_headers(headers.clone())
        .cookie_provider(cookie_jar) 
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    // await中にstateが変更される可能性があるため、base_urlを変数に格納しておく
    let base_url = api_config.base_url.clone();

    // リクエスト発行
    println!("DebugLog:リクエスト発行");
    let response1:Response = client
        .get(base_url + "worlds/favorites")
        .headers(headers.clone())
        .query(&[
            ("n", "200"), 
            ("offset", "0")
        ])
        .send()
        .await
        .unwrap();
    
    // API取得制限用60秒タイマーの起動
    // 次のAPIの呼び出し時に、前回のAPI取得から1分経過しているか確認する
    // APIの取得は1分に1回のみ
    let timer = tokio::time::Instant::now() + Duration::from_secs(60);

    // レスポンスの取得
    let body1 = response1
        .text()
        .await
        .unwrap();
    // println!("Response is : \n{}", body1);
    let body_json1: Vec<FavoriteWorldFromAPI> = serde_json::from_str(&body1)
        .expect("Failed to parse JSON");
    println!("Response is : \n{:?}", body_json1);

    // DBに格納
    for world in body_json1 {
        if world.tags.is_none() {
            println!("DebugLog:タグが存在しないデータをスキップ");
            println!("DebugLog:{:?}", world);
            continue;
        }
        println!("DebugLog:DBに格納");
        println!("DebugLog:{:?}", world);
        insert_favorite_world(&db_pool, world).await.unwrap();
    }

    // 前回のAPI取得から1分経過しているか確認する
    // 1分経過していない場合は、1分経過するまで待機する
    // 1分経過している場合は、APIを取得する
    if timer.elapsed().as_secs() < 60 {
        println!("DebugLog:API取得制限中");
        tokio::time::sleep(Duration::from_secs(60) - timer.elapsed()).await;
    }

    // リクエスト発行
    println!("DebugLog:リクエスト発行");
    let response2:Response = client
        .get(api_config.base_url.clone() + "worlds/favorites")
        .headers(headers.clone())
        .query(&[
            ("n", "200"), 
            ("offset", "200")
        ])
        .send()
        .await
        .unwrap();  
    // レスポンスの取得
    let body2 = response2
        .text()
        .await
        .unwrap();
    // println!("Response is : \n{}", body2);
    let body_json2: Vec<FavoriteWorldFromAPI> = serde_json::from_str(&body2)
        .expect("Failed to parse JSON");
    println!("Response is : \n{:?}", body_json2);

    // DBに格納
    for world in body_json2 {
        if world.tags.is_none() {
            println!("DebugLog:タグが存在しないデータをスキップ");
            println!("DebugLog:{:?}", world);
            continue;
        }
        println!("DebugLog:DBに格納");
        println!("DebugLog:{:?}", world);
        insert_favorite_world(&db_pool, world).await.unwrap();
    }
    
    return Ok("処理の完了".to_string()); 
}

/* read favorite dbから読み込んだデータを返す */
#[tauri::command]
pub async fn read_favorite(app_state: tauri::State<'_, AppState>) -> Result<Vec<FavoriteWorldFromAPI>, ()> { 
    // DBPoolを取得
    let db_pool = app_state.context.lock().unwrap().db_pool.clone();
    let result = select_favorite_world(&db_pool).await.unwrap();
    return Ok(result);
}