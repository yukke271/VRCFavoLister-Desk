// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::{Url, Client, RequestBuilder, Response, cookie::Jar, header::{self, USER_AGENT, HeaderMap, HeaderValue}};
use std::{sync::Arc, time::Duration};
//use reqwest::header::{SET_COOKIE};
//use http::{HeaderMap, HeaderValue, header::{COOKIE}};
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

    TODO:エラーハンドリングを実装する

    非同期コマンドの実装について    
    https://tauri.app/v1/guides/features/command/#async-commands

    エラーコードを返す
    0: 成功
    1: なんらかの処理に失敗した
    2: usernameかpasswordが空
    3: メールボックスの確認が必要な2段階認証 
    4: 認証アプリによる2段階認証
    5: 2段階認証のコードが空
    6: 何らかの理由でログインに失敗した
*/
#[tauri::command]
async fn login(username: &str, password: &str, otp_code: &str) -> Result<u8, ()> { 
    
    // 引数内の文字列を表示
    println!("username: {}", username);
    println!("password: {}", password);
    println!("otp_code: {}", otp_code);

    // APIConfigを取得
    let mut api_config = APIConfig::new();
    
    // usernameかpasswordが空の場合、エラー
    if api_config.username.is_none() {
        if username.is_empty() {
            return Ok(2);
        } else {
            api_config.username = Some(username.to_string());
        }
    }
    if api_config.password.is_none() {
        if password.is_empty() {
            return Ok(2);
        } else {
            api_config.password = Some(password.to_string());
        }
    }

    println!("DebugLog:ヘッダー設定");
    // ヘッダー設定
    let mut headers: HeaderMap  = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str(&api_config.user_agent.clone()).unwrap());
    headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
    headers.insert(header::ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));

    println!("DebugLog:クッキージャー作成");
    // クッキージャーを作成
    let cookie_jar = Arc::new(Jar::default());
    cookie_jar.add_cookie_str(&("apiKey=".to_owned() + &api_config.api_key.clone()), &Url::parse(&api_config.base_url).unwrap());
    
    println!("DebugLog:authCookie設定する");
    // authCookieがある場合設定する
    if !api_config.auth_cookie.is_none() && !otp_code.is_empty() {
        cookie_jar.add_cookie_str(&api_config.auth_cookie.clone().unwrap(), &Url::parse(&api_config.base_url).unwrap());
    }

    println!("DebugLog:2段階認証済設定");
    // two_factor_authがある場合設定する
    if !api_config.two_factor_auth.is_none() {
        cookie_jar.add_cookie_str(&api_config.two_factor_auth.clone().unwrap(), &Url::parse(&api_config.base_url).unwrap());
    }

    println!("DebugLog:リクエストクライアント作成");
    // リクエストクライアントを作成        
    let client: Client  = reqwest::Client::builder()
        .default_headers(headers.clone())
        .cookie_provider(cookie_jar) 
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();
    
    println!("DebugLog:リクエストビルダー作成");
    // リクエスト発行準備
    // エンドポイントの設定
    let mut user_request:RequestBuilder = client
        .get(api_config.base_url.clone() + "auth/user")
        .headers(headers.clone())
        .basic_auth(api_config.username.clone().unwrap(), Some(api_config.password.clone().unwrap()));

    println!("DebugLog:2段階認証コード設定");
    // 2段階認証のコードがある場合、postで認証
    if !otp_code.is_empty() {

        if api_config.two_factor_type.is_none() {
            // 2段階認証の方式が未設定の場合エラー
            println!("DebugLog:2段階認証方式未設定");
            return Ok(1);
        }
        let factor_type: String = api_config
            .two_factor_type
            .clone()
            .expect("two_factor_type is None");

        // 送信用の変数を作成
        // '{ "code": "######" }'
        let send_data = format!("{{ \"code\": \"{}\" }}", otp_code); 

        // 2段階認証の方式によってエンドポイントが異なる
        let verify_endpoint: String;
        
        println!("DebugLog:2段階認証方式判定");
        println!("factor type is : \n{}", factor_type);
        
        if factor_type.contains("emailOtp") {
            // emailOtpの場合
            verify_endpoint = api_config.base_url.clone() + "auth/twofactorauth/emailotp/verify";
        } else if factor_type.contains("totp") {
            // totpの場合
            verify_endpoint = api_config.base_url.clone() + "auth/twofactorauth/totp/verify";
        } else if factor_type.contains("otp") {
            // otpの場合
            verify_endpoint = api_config.base_url.clone() + "auth/twofactorauth/otp/verify";
        } else {
            // それ以外の例外
            println!("DebugLog:2段階認証方式不明");
            println!("factor type is : \n{}", factor_type);
            return Ok(1);
        }
        
        println!("DebugLog:リクエストビルダー再設定");
        user_request = client
            .post(verify_endpoint)
            .headers(headers.clone())
            .body(send_data);
    }
    
    println!("DebugLog:リクエスト発行");
    // エンドポイントと認証に必要な情報以外を設定し、リクエストの発行
    let user_response:Response = user_request
        .send()
        .await
        .unwrap();  
        
    // println!("DebugLog:ステータス確認");
    // // リクエストの成否を確認
    // if !user_response.status().is_success() {
    //     eprintln!("Failed to first request: {}", user_response.status());
    //     return Ok(1);
    // }

    /*
    let cookies = user_response
        .headers()
        .get(SET_COOKIE)
        .expect("failed to get cookie")
        .to_str()
        .unwrap();
    println!("cookies is : \n{}", cookies);
    */
    
    println!("process get header");
    // user_response.headers().get_all(&api_config.base_url).iter().for_each(|v| {
    //     println!("header: {:?}", v.to_str().unwrap());
    // });
    for (header_name, header_value) in user_response.headers() {
        // println!("{}: {:?}", header_name, header_value);
        if header_name == "set-cookie" {
            println!("set-cookie: {:?}", header_value);
            // api_config.two_factor_auth = HeaderValue::from_str(header_value).unwrap();
            api_config.two_factor_auth = Some(header_value.to_str().unwrap_or_default().to_string());
        }
    }
    println!("success get header");
    

    // レスポンスの中身を取得
    // レスポンスはストリームのため一度しか処理できない
    // 複製したい場合は、text()を使い、Stringに変換してから処理する
    // 参考:https://stackoverflow.com/questions/77344789/how-to-deal-with-no-method-named-clone-found-for-third-party-struct#comment136354605_77344789
    
    let body = user_response
        .text()
        .await
        .unwrap();
    println!("Response is : \n{}", body);
    
    // 2段階認証が必要か確認
    if body.contains("requiresTwoFactorAuth") {
    
        /*
        // println!("cookie: {:?}", cookies);
        api_config.auth_cookie = Some(cookies.to_string());        
        // authCookieが取得できなかった場合エラー
        if api_config.auth_cookie.is_none() {
            eprintln!("Failed to get authCookie");
            return Ok(6);
        }
        */

        // bodyにemailOtpが含まれている場合、メールボックスの確認が必要な2段階認証
        if body.contains("emailOtp") {
            api_config.two_factor_type = Some("emailOtp".to_string());
            let _ = api_config.save_config_file();
            return Ok(3);
        }

        // bodyにtotpが含まれている場合、認証アプリによる2段階認証
        if body.contains("totp") {
            api_config.two_factor_type = Some("totp".to_string());
            let _ = api_config.save_config_file();
            return Ok(4);
        }

        // bodyにotpが含まれている場合、認証アプリによる2段階認証
        if body.contains("otp") {
            api_config.two_factor_type = Some("totp".to_string());
            let _ = api_config.save_config_file();
            return Ok(4);
        }

        // 上記の処理で2段階認証が必要か確認できなかった場合、完全な例外なためエラー
        return Ok(1);

    }

    // Configを保存
    let _ = api_config.save_config_file();

    return Ok(0);
}


// // 2段階認証前のauth cookieを取得
// let cookies = match user_response.headers().get(SET_COOKIE) {
//     cookie => {
//         let cookies_str = cookie
//             .expect("failed to get cookie")
//             .to_str()
//             .unwrap();
//         // println!("cookies is : \n{}", cookies_str);
//         cookies_str
//     }
// };

// // authCookieがNoneの場合、ログイン試行
// if api_config.auth_cookie.is_none() {
    
//     // 2段階認証のためのCookieを取得するための初回ログイン処理
//     // TODO:Cookieが存在した場合の処理を考慮する必要がある

//     // リクエストクライアントを作成        
//     let client: Client  = reqwest::Client::builder()
//         .default_headers(headers.clone())
//         .cookie_provider(cookie_jar) 
//         .timeout(Duration::from_secs(10))
//         .build()
//         .unwrap();


//     // ワンタイムヘッダーを作成
//     // let mut onetime_headers: HeaderMap = HeaderMap::new();
//     // onetime_headers.insert(HeaderName::try_from("Custom-Header").unwrap(), HeaderValue::from_static("header value"));

//     // リクエスト発行

//     // let resp = client.get("api")            
//     //     .query(&[("name", "foo")])
//     //     .send()
//     //     .map(|resp|{
//     //         println!("{:?}", resp.status());
//     //         resp
//     //     })
//     //     .map_err(|err|{
//     //         println!("{:?}", err);
//     //         err
//     //     });
        
//     let user_response:Response = client
//         .get(api_config.base_url.clone() + "auth/user")
//         .headers(headers)
//         .basic_auth(api_config.username.clone().unwrap(), Some(api_config.password.clone().unwrap()))
//         .send()
//         .await
//         .unwrap();

//     // リクエストの中身を取得
//     // let user_response_str = match user_response.headers().get(header::TRANSFER_ENCODING) {
        
//     //     Some(v) if v == "chunked" => {
//     //         let mut raw = Vec::new();
//     //         while let Some(item) = user_response.chunk().await.unwrap() {
//     //             item
//     //                 .to_vec()
//     //                 .into_iter()
//     //                 .for_each(|byte| raw.push(byte));
//     //         }
//     //         String::from_utf8(raw).unwrap()
//     //     },
//     //     _ => {
//     //         let res_str = user_response.text().await.unwrap();
//     //         println!("Response is : \n{}", res_str);
//     //         res_str
//     //     }

//     //     // Ok(response) => {
            
//     //     //     Some(v) if v == "chunked" => {
//     //     //         let mut raw = Vec::new();
//     //     //         while let Some(item) = response.chunk().next().await.unwrap() {
//     //     //             item
//     //     //                 .to_vec()
//     //     //                 .into_iter()
//     //     //                 .for_each(|byte| raw.push(byte));
//     //     //         }
//     //     //         String::from_utf8(raw).unwrap()
//     //     //     },
//     //     //     _ => {
//     //     //         let res_str = response.text().await.unwrap()
//     //     //         println!("Response is : \n{}", res_str);
//     //     //         res_str
//     //     //     }

//     //     // }
//     //     // Err(e) => {
//     //     //     eprintln!("Failed to first request: {}", e);
//     //     //     return 3;
//     //     // }
//     // };
    
//     // レスポンスヘッダーの中身を表示
//     // println!("Status: {}", user_response.status());
//     // println!("Content-length: {}", user_response.content_length().unwrap());
//     // for (header_name, header_value) in user_response.headers() {
//     //     println!("{}: {:?}", header_name, header_value);
//     // }

//     // レスポンスヘッダーからauthCookieを取得
//     // api_config.auth_cookie = user_response.headers().get_all(SET_COOKIE).iter().next().map(|v| v.to_str().unwrap().to_string());

//     // リクエストの成否を確認
//     if !user_response.status().is_success() {
//         eprintln!("Failed to first request: {}", user_response.status());
//         return 1;
//     }

//     // 認証用のCookieを取得
//     let cookies = match user_response.headers().get(SET_COOKIE) {
//         cookie => {
//             let cookies_str = cookie
//                 .expect("failed to get cookie")
//                 .to_str()
//                 .unwrap();
//             // println!("cookies is : \n{}", cookies_str);
//             cookies_str
//         }
//     };
//     // println!("cookie: {:?}", cookies);
//     api_config.auth_cookie = Some(cookies.to_string());
    
//     // api_config.auth_cookie = cookie_jar.get_cookie(&Url::parse(&api_config.base_url).unwrap(), "auth").map(|v| v.value().to_string());

//     /* 
//         初回ログイン時はなんらかの2段階認証が必要
    
//         // 2要素認証が有効な場合
//         {"requiresTwoFactorAuth":["totp","otp"]}

//         // 2要素認証を設定していない場合
//         {"requiresTwoFactorAuth":["emailOtp"]}
//     */
//     let body = user_response
//         .text()
//         .await
//         .unwrap();
//     println!("Response is : \n{}", body);

//     // authCookieが取得できなかった場合エラー
//     if api_config.auth_cookie.is_none() {
//         eprintln!("Failed to get authCookie");
//         return 3;
//     }

//     // println!("authCookie: {:?}", api_config.auth_cookie);
    
    
//     // user_responseの中身を表示
//     // let res_str = user_response.text().await.unwrap();
//     // println!("Response is : \n{}", user_response_str);

//     return 0;
// }



/*
$ echo -e "api.vrchat.cloud\tFALSE\t/\tFALSE\t0\tapiKey\tJlE5Jldo5Jibnk5O5hTx6XVqsJu4WJ26" > cookiejar.txt
$ curl -b cookiejar.txt -c cookiejar.txt -A "userAgent" -H "Authorization: Basic ########################" https://api.vrchat.cloud/api/1/auth/user

{"requiresTwoFactorAuth":["emailOtp"]}
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

