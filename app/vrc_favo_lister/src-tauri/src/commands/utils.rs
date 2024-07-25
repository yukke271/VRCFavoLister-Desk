/*
  細かい便利な関数をまとめたファイル
  このファイルには、以下の関数が含まれています。
  - is_debug
  - debug_log
  - get_file_path
  - get_file_path_str
  - create_headers
  - create_cookie_jar
  - create_request_client

*/ 


use std::sync::Arc;
use std::time::Duration;

use reqwest::Url;
use reqwest::Client;
use reqwest::cookie::Jar;
use reqwest::header::HeaderMap;
use reqwest::header::{HeaderValue, USER_AGENT, ACCEPT_ENCODING};

use tauri::api::path::{BaseDirectory, resolve_path};
use tauri::Env;
use tauri::generate_context;

use crate::structs::apiconfig::APIConfig;

// 現在のビルドステータスを取得する関数。
// 開発者ビルドの時にtrueを返す。
pub fn is_debug() -> bool {
  // 開発者ビルドの時にのみ有効になる。
  #[cfg(debug_assertions)]
  {
    true
  }
  // 開発者ビルドではないときに有効になる。
  #[cfg(not(debug_assertions))]
  {
    false
  }
}

// デバッグ用の関数
// どんな型の引数でも受け取れる。
pub fn debug_log<T: std::fmt::Debug>(arg: T) {
  // 開発者ビルドの時のみログを出力する
  if !is_debug() {
    return;
  }
  println!("DebugLog:{:#?}", arg);
}

// ファイルパスをPathBufで返す関数
pub fn get_file_path(file_name: &str) -> std::path::PathBuf {
  let context = generate_context!();
  let app_data_dir = resolve_path(
    context.config(),
    context.package_info(),
    &Env::default(),
    "",
    Some(BaseDirectory::AppLocalData)
  ).unwrap();
  app_data_dir.join(file_name)
}

// ファイルパスを文字列で返す関数
pub fn get_file_path_str(file_name: &str) -> String {
  let context = generate_context!();
  let app_data_dir = resolve_path(
    context.config(),
    context.package_info(),
    &Env::default(),
    "",
    Some(BaseDirectory::AppLocalData)
  ).unwrap();
  // PathbufをStringに変換
  app_data_dir.into_os_string().into_string().unwrap() + file_name
}

// httpリクエストのヘッダーを作成する関数
pub fn create_headers(user_agent: &str) -> HeaderMap {
  debug_log("ヘッダー設定");
  let mut headers: HeaderMap = HeaderMap::new();
  headers.insert(USER_AGENT, HeaderValue::from_str(user_agent).unwrap());
  headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
  headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
  headers
}

// httpリクエストのクッキージャーを作成する関数
pub fn create_cookie_jar(api_config: &APIConfig) -> Arc<Jar> {
  debug_log("クッキージャー作成");
  let cookie_jar = Arc::new(Jar::default());
  cookie_jar.add_cookie_str(&("apiKey=".to_owned() + &api_config.api_key.clone()), &Url::parse(&api_config.base_url).unwrap());
  // authCookieがある場合設定する
  if !api_config.auth_cookie.is_none() {
    debug_log("authCookie設定する");
    cookie_jar.add_cookie_str(&api_config.auth_cookie.clone().unwrap(), &Url::parse(&api_config.base_url).unwrap());
  }
  // two_factor_authがある場合設定する
  if !api_config.two_factor_auth.is_none() {
    debug_log("2段階認証設定");
    cookie_jar.add_cookie_str(&api_config.two_factor_auth.clone().unwrap(), &Url::parse(&api_config.base_url).unwrap());
  }
  cookie_jar
}

// httpリクエストクライアントを作成する関数
pub fn create_request_client(headers: HeaderMap, cookie_jar: Arc<Jar>) -> Client {
  debug_log("リクエストクライアント作成");
  Client::builder()
    .default_headers(headers)
    .cookie_provider(cookie_jar) 
    .timeout(Duration::from_secs(10))
    .build()
    .unwrap()
}