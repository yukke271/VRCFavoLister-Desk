use reqwest::{Url, Client, RequestBuilder, Response, cookie::Jar, header::{self, USER_AGENT, HeaderMap, HeaderValue}};
use std::{sync::Arc, time::Duration};

use crate::structs::apiconfig::APIConfig;

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
pub async fn login(username: &str, password: &str, otp_code: &str) -> Result<u8, ()> { 
    
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
            if header_value.to_str().unwrap().contains("authcookie") {
                println!("authcookie: {:?}", header_value);
                api_config.auth_cookie = Some(header_value.to_str().unwrap_or_default().to_string());
            }
            if header_value.to_str().unwrap().contains("twoFactorAuth") {
                println!("twoFactorAuth: {:?}", header_value);
                api_config.two_factor_auth = Some(header_value.to_str().unwrap_or_default().to_string());
            }
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
