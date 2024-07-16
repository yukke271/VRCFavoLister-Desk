// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::header::{USER_AGENT, SET_COOKIE};
// use base64::{engine::general_purpose, Engine as _};
use crate::apiconfig::APIConfig;
mod apiconfig;


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        greet,
        login_check,
        login,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn login_check() -> bool {
    return true;
}


/*
    エラーコードを返す
    0: 成功
    1: usernameかpasswordが空
    2: usernameとpasswordの組のエンコードに失敗
    3: 初回ログイン処理に失敗
    
*/
#[tauri::command]
async fn login() -> u8 { 
    
    // APIConfigを取得
    let mut config = APIConfig::new();
    
    // usernameかpasswordが空の場合、エラー
    if config.username.is_none() || config.password.is_none() {
        return 1;
    }

    // basic_credentialsがNoneの場合、usernameとpasswordから生成
    // if config.basic_credentials.is_none() {

    //     // usernameとpasswordの組をbase64エンコード
    //     let username_password = config.username.unwrap() + ":" + &config.password.unwrap();
    //     config.basic_credentials = Some(format!("Basic {}", general_purpose::STANDARD.encode(&username_password)));
    // }

    // authCookieがNoneの場合、ログイン試行
    if config.auth_cookie.is_none() {

        // リクエスト発行
        let client = reqwest::Client::new();
        let user_response = client
            .get(config.base_url + "auth/user")
            .header("Content-type", "application/json")
            .header(USER_AGENT, config.user_agent)
            .basic_auth(config.username.unwrap(), Some(config.password.unwrap()))
            .send()
            .await;

        // リクエストに失敗した場合エラー
        let user_response = match user_response {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Failed to first request: {}", e);
                return 3;
            }
        };

        // レスポンスヘッダーからauthCookieを取得
        config.auth_cookie =  user_response.headers().get_all(SET_COOKIE).iter().next().map(|v| v.to_str().unwrap().to_string());

        // authCookieが取得できなかった場合エラー
        if config.auth_cookie.is_none() {
            eprintln!("Failed to get authCookie");
            return 3;
        }

        // user_responseの中身を表示
        let res_str = user_response.text().await.unwrap();
        println!("Response is : \n{}", res_str);
    }
    
    return 0;
}


/*
$ echo -e "api.vrchat.cloud\tFALSE\t/\tFALSE\t0\tapiKey\tJlE5Jldo5Jibnk5O5hTx6XVqsJu4WJ26" > cookiejar.txt
$ curl -b cookiejar.txt -c cookiejar.txt -A "WorldMon" -H "Authorization: Basic ########################" https://api.vrchat.cloud/api/1/auth/user

{"requiresTwoFactorAuth":["totp","otp"]}

$ curl -X 'POST' \
  'https://api.vrchat.cloud/api/1/auth/twofactorauth/totp/verify' \
  -A 'Mozilla/5.0' \ 
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -b cookiejar.txt -c cookiejar.txt  \
  -d '{
  "code": "######"
}'

*/

