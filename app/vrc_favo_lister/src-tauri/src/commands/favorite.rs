use reqwest::{Client, Response, header::HeaderMap};
use std::time::Duration;

use crate::structs::app_state::{AppState, ContextTrait};
use crate::structs::app_struct::AppFavoriteWorldCard;
use crate::structs::favorite_world::FavoriteWorldFromAPI;

use crate::commands::database::{ insert_favorite_world, select_favorite_world };
use crate::commands::utils::{debug_log, create_headers, create_cookie_jar, create_request_client};

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
pub async fn load_favorite_worlds(app_state: tauri::State<'_, AppState>) -> Result<Vec<FavoriteWorldFromAPI>, String> { 

    // APIConfigを取得
    let api_config = app_state.context.lock().unwrap().get_api_config();
    // DBPoolを取得
    let db_pool = app_state.context.lock().unwrap().db_pool.clone();
    
    if api_config.two_factor_auth.is_none() {
        debug_log("2段階認証未設定");
        return Err("2段階認証未設定".to_string());
    }

    // 整形できなかったデータを格納する変数
    let mut failed_data: Vec<FavoriteWorldFromAPI> = Vec::new();

    // ヘッダー設定
    let headers: HeaderMap = create_headers(&api_config.user_agent.clone());
    // クッキージャーを作成
    let cookie_jar = create_cookie_jar(&api_config);
    // リクエストクライアントを作成        
    let client: Client = create_request_client(
        headers.clone(),
        cookie_jar.clone()
    );

    // await中にstateが変更される可能性があるため、base_urlを変数に格納しておく
    let base_url = api_config.base_url.clone();

    // リクエスト発行
    debug_log("リクエスト発行");
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
    // debug_log(format!("Response is : \n{}", body1));
    let body_json1: Vec<FavoriteWorldFromAPI> = serde_json::from_str(&body1)
        .expect("Failed to parse JSON");
    // debug_log(format!("Response is : \n{:?}", body_json1));

    // DBに格納
    for world in body_json1 {
        if world.tags.is_none() {
            debug_log("タグが存在しないデータをスキップ");
            debug_log(format!("{:?}", world));
            failed_data.push(world);
            continue;
        }
        debug_log("DBに格納");
        debug_log(format!("{:?}", world));
        insert_favorite_world(&db_pool, world).await.unwrap();
    }

    // 前回のAPI取得から1分経過しているか確認する
    // 1分経過していない場合は、1分経過するまで待機する
    // 1分経過している場合は、APIを取得する
    if timer.elapsed().as_secs() < 60 {
        debug_log(format!("DebugLog:残り時間 : {:?}", 60 - timer.elapsed().as_secs()));
        tokio::time::sleep(Duration::from_secs(60) - timer.elapsed()).await;
    }

    // リクエスト発行
    debug_log("DebugLog:リクエスト発行");
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
    // debug_log(format!("Response is : \n{}", body2));
    let body_json2: Vec<FavoriteWorldFromAPI> = serde_json::from_str(&body2)
        .expect("Failed to parse JSON");
    // debug_log(format!("Response is : \n{:?}", body_json2));

    // DBに格納
    for world in body_json2 {
        if world.tags.is_none() {
            debug_log("タグが存在しないデータをスキップ");
            debug_log(format!("{:?}", world));
            failed_data.push(world);
            continue;
        }
        debug_log("DBに格納");
        debug_log(format!("{:?}", world));
        insert_favorite_world(&db_pool, world).await.unwrap();
    }
    
    return Ok(failed_data); 
}

/* read favorite dbから読み込んだデータを返す */
#[tauri::command]
pub async fn read_favorite(app_state: tauri::State<'_, AppState>) -> Result<Vec<AppFavoriteWorldCard>, ()> { 
    // DBPoolを取得
    let db_pool = app_state.context.lock().unwrap().db_pool.clone();
    let result = select_favorite_world(&db_pool).await.unwrap();
    return Ok(result);
}