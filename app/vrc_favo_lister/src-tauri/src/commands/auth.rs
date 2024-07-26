use reqwest::{Client, RequestBuilder, Response, header::HeaderMap};

use crate::structs::app_state::{AppState, ContextTrait};
use crate::commands::utils::{debug_log, create_headers, create_cookie_jar, create_request_client};

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

    auth.rsで使用するエンドポイントは以下の通り
    - ユーザー情報取得: https://api.vrchat.cloud/api/1/auth/user
    - ログアウト: https://api.vrchat.cloud/api/1/auth/logout
    - 2段階認証: https://api.vrchat.cloud/api/1/auth/twofactorauth/emailotp/verify
    - 2段階認証: https://api.vrchat.cloud/api/1/auth/twofactorauth/totp/verify
    - 2段階認証: https://api.vrchat.cloud/api/1/auth/twofactorauth/otp/verify
    - Cookie確認: https://api.vrchat.cloud/api/1/auth
*/
#[tauri::command]
pub async fn login(app_state: tauri::State<'_, AppState>, username: &str, password: &str, otp_code: &str) -> Result<u8, ()> { 
    
    // 引数内の文字列を表示
    debug_log(format!("username: {}", username));
    debug_log(format!("password: {}", password));
    debug_log(format!("otp_code: {}", otp_code));

    // APIConfigを取得
    let mut api_config = app_state.context.lock().unwrap().get_api_config();
    // is_loginを取得
    let is_login = app_state.context.lock().unwrap().get_is_login();
    
    // usernameかpasswordが空の場合、エラー
    if api_config.username.is_none() || api_config.password.is_none() {
        if username.is_empty() || password.is_empty() {
            debug_log("usernameかpasswordが空");
            return Ok(2);
        } else {
            api_config.username = Some(username.to_string());
            api_config.password = Some(password.to_string());
        }
    }

    // ヘッダー設定
    let headers: HeaderMap = create_headers(&api_config.user_agent.clone());
    // クッキージャーを作成
    let cookie_jar = create_cookie_jar(&api_config);
    // リクエストクライアントを作成        
    let client: Client = create_request_client(
        headers.clone(),
        cookie_jar.clone()
    );
    
    // リクエスト発行準備
    // エンドポイントの設定
    debug_log("リクエストビルダー作成");
    let mut user_request:RequestBuilder = client
        .get(api_config.base_url.clone() + "auth/user")
        .headers(headers.clone())
        .basic_auth(api_config.username.clone().unwrap(), Some(api_config.password.clone().unwrap()));

    debug_log("2段階認証コード設定");
    // 2段階認証のコードがある場合、postで認証
    if !otp_code.is_empty() {
        if api_config.two_factor_type.is_none() {
            // 2段階認証の方式が未設定の場合エラー
            debug_log("2段階認証方式未設定");
            return Ok(1);
        }
        let factor_type: String = api_config.two_factor_type.clone().expect("two_factor_type is None");

        // 送信用の変数を作成
        // '{ "code": "######" }'
        let send_data = format!("{{ \"code\": \"{}\" }}", otp_code); 

        // 2段階認証の方式によってエンドポイントが異なる
        let verify_endpoint: String;
        
        debug_log("2段階認証方式判定");
        debug_log(format!("factor type is : \n{}", factor_type));        
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
            debug_log("2段階認証方式不明");
            debug_log(format!("factor type is : \n{}", factor_type));
            return Ok(1);
        }
        
        debug_log("リクエストビルダー再設定");
        user_request = client
            .post(verify_endpoint)
            .headers(headers.clone())
            .body(send_data);
    }

    debug_log("リクエスト発行");
    // エンドポイントと認証に必要な情報以外を設定し、リクエストの発行
    let user_response:Response = user_request
        .send()
        .await
        .unwrap();  
    
    debug_log("ステータス確認");
    // リクエストの成否を確認
    if !user_response.status().is_success() {
        debug_log("Failed to first request");
        return Ok(1);
    }

    debug_log("ヘッダー取得");
    for (header_name, header_value) in user_response.headers() {
        debug_log(format!("{}: {:?}", header_name, header_value));
        if header_name == "set-cookie" {
            debug_log(format!("set-cookie: {:?}", header_value));
            if header_value.to_str().unwrap().contains("authcookie") {
                debug_log(format!("authcookie: {:?}", header_value));
                api_config.auth_cookie = Some(header_value.to_str().unwrap_or_default().to_string());
            }
            if header_value.to_str().unwrap().contains("twoFactorAuth") {
                debug_log(format!("twoFactorAuth: {:?}", header_value));
                api_config.two_factor_auth = Some(header_value.to_str().unwrap_or_default().to_string());
            }
        }
    }
    
    // レスポンスの中身を取得
    // レスポンスはストリームのため一度しか処理できない
    // 複製したい場合は、text()を使い、Stringに変換してから処理する
    // 参考:https://stackoverflow.com/questions/77344789/how-to-deal-with-no-method-named-clone-found-for-third-party-struct#comment136354605_77344789
    
    let body = user_response
        .text()
        .await
        .unwrap();
    debug_log(format!("Response is : \n{}", body));
   
    // 2段階認証が必要か確認
    if body.contains("requiresTwoFactorAuth") {
    
        // bodyにemailOtpが含まれている場合、メールボックスの確認が必要な2段階認証
        if body.contains("emailOtp") {
            api_config.two_factor_type = Some("emailOtp".to_string());
            // Configを保存
            app_state.context.lock().unwrap().set_api_config(api_config);
            debug_log("メールボックスの確認が必要な2段階認証");
            return Ok(3);
        }

        // bodyにtotpが含まれている場合、認証アプリによる2段階認証
        if body.contains("totp") {
            api_config.two_factor_type = Some("totp".to_string());
            // Configを保存
            app_state.context.lock().unwrap().set_api_config(api_config);
            debug_log("認証アプリによる2段階認証");
            return Ok(4);
        }

        // bodyにotpが含まれている場合、認証アプリによる2段階認証
        if body.contains("otp") {
            api_config.two_factor_type = Some("totp".to_string());
            // Configを保存
            app_state.context.lock().unwrap().set_api_config(api_config);
            debug_log("認証アプリによる2段階認証");
            return Ok(4);
        }

        // 上記の処理で2段階認証が必要か確認できなかった場合、完全な例外なためエラー
        debug_log("2段階認証方式不明");
        return Ok(1);

    }

    // Configを保存
    app_state.context.lock().unwrap().set_api_config(api_config);
    app_state.context.lock().unwrap().set_is_login(true);
    debug_log("ログイン成功");
    return Ok(0);
}

// ログアウト処理
#[tauri::command]
pub async fn logout(app_state: tauri::State<'_, AppState>) -> Result<u8, ()> { 
    
    // APIConfigを取得
    let mut api_config = app_state.context.lock().unwrap().get_api_config();

    // ヘッダー設定
    let headers: HeaderMap = create_headers(&api_config.user_agent.clone());
    // クッキージャーを作成
    let cookie_jar = create_cookie_jar(&api_config);
    // リクエストクライアントを作成        
    let client: Client = create_request_client(
        headers.clone(),
        cookie_jar.clone()
    );
    
    // リクエスト発行準備
    // エンドポイントの設定
    let user_request:RequestBuilder = client
        .put(api_config.base_url.clone() + "auth/logout")
        .headers(headers.clone())
        .basic_auth(api_config.username.clone().unwrap(), Some(api_config.password.clone().unwrap()));

    // リクエスト発行
    let user_response:Response = user_request
        .send()
        .await
        .unwrap();  
    
    // リクエストの成否を確認
    if !user_response.status().is_success() {
        debug_log("ログアウト失敗");
        return Ok(1);
    }

    // Configを初期化
    api_config.auth_cookie = None;
    api_config.two_factor_auth = None;
    api_config.two_factor_type = None;
    api_config.username = None;
    api_config.password = None;

    // Configを保存
    app_state.context.lock().unwrap().set_api_config(api_config);
    app_state.context.lock().unwrap().set_is_login(false);

    debug_log("ログアウト成功");
    return Ok(0);
}

// cookieが有効か確認
// 有効な場合はtrueを返す
#[tauri::command]
pub async fn check_cookie(app_state: tauri::State<'_, AppState>) -> Result<bool, ()> { 
    
    // APIConfigを取得
    let api_config = app_state.context.lock().unwrap().get_api_config();
    
    // ヘッダー設定
    let headers: HeaderMap = create_headers(&api_config.user_agent.clone());
    // クッキージャーを作成
    let cookie_jar = create_cookie_jar(&api_config);
    // リクエストクライアントを作成        
    let client: Client = create_request_client(
        headers.clone(),
        cookie_jar.clone()
    );
    
    // リクエスト発行準備
    // エンドポイントの設定
    let user_request:RequestBuilder = client
        .get(api_config.base_url.clone() + "auth")
        .headers(headers.clone())
        .basic_auth(api_config.username.clone().unwrap(), Some(api_config.password.clone().unwrap()));

    // リクエスト発行
    let user_response:Response = user_request
        .send()
        .await
        .unwrap();  
    
    // リクエストの成否を確認
    if !user_response.status().is_success() {
        debug_log("cookieが有効ではありません");
        app_state.context.lock().unwrap().set_is_login(false);
        return Ok(false);
    }

    debug_log("cookieが有効です");
    app_state.context.lock().unwrap().set_is_login(true);
    return Ok(true);
}

// app_stateのis_loginを取得
#[tauri::command]
pub async fn get_is_login(app_state: tauri::State<'_, AppState>) -> Result<bool, ()> { 
    
    // is_loginを取得
    let is_login = app_state.context.lock().unwrap().get_is_login();
    return Ok(is_login);
}