use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::{env, fs::File, io::{ErrorKind, Read, Write}};

use tauri::{api::path::{BaseDirectory, resolve_path}, Env};
//use tauri::api::fs::{read_text_file,write_text_file};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct APIConfig {
  pub base_url: String,
  pub api_key: String,
  pub user_agent: String,
  pub auth_cookie: Option<String>,
  pub two_factor_type: Option<String>,
  pub two_factor_auth: Option<String>,
  pub username: Option<String>,
  pub password: Option<String>,
}

impl APIConfig {
  
  pub fn new() -> Self {
    APIConfig::load_config_file().expect("REASON")
  }

  pub fn save_config_file(&self) -> Result<()> {
    // 自身の構造に合わせたjson形式でファイルを保存する
    let context = tauri::generate_context!();
    let config_path = resolve_path(
        context.config(),
        context.package_info(),
        &Env::default(),
        "config.json",
        Some(BaseDirectory::AppLocalData) )
      .unwrap();
    let _ = match File::open(config_path.clone()) {
      Ok(_) => {
        match File::create(config_path.clone()) {
          Ok(mut fc) => fc.write_all(serde_json::to_string(&self).unwrap().as_bytes()),
          Err(err) => panic!("Cannot created file: {:?}", err),
        }
      },
      Err(err) => panic!("Cannot open file: {:?}", err),
    };

    println!("Save config file: {:?}", config_path.clone());
    println!("Save config: {:?}", self);

    Ok(())
  }

  fn load_config_file() -> Result<Self> {  

    let api_config_default = Self {
      base_url: "https://api.vrchat.cloud/api/1/".to_string(),
      api_key: "JlE5Jldo5Jibnk5O5hTx6XVqsJu4WJ26".to_string(),
      user_agent: "vrc_fabo_lister/0.1".to_string(),
      auth_cookie: None,
      two_factor_type: None,
      two_factor_auth: None,
      username: None,
      password: None,
    };

    // contextを取得する
    let context = tauri::generate_context!();

    // Configのパスを取得
    let config_path = resolve_path(
        context.config(),
        context.package_info(),
        &Env::default(),
        "config.json",
        Some(BaseDirectory::AppLocalData) )
      .unwrap();

    // println!("config_path: {:?}", config_path);

    // jsonデータが保存されたConfigファイルを読み込む

    // TODO:ファイルの作成部分は、アプリの初期化処理に移動させる
    // ファイルが見つかりませんでした
    let f = File::open(config_path.clone());
    let mut f = match f {
      Ok(file) => file,
      Err(ref err) if err.kind() == ErrorKind::NotFound => {
        match File::create(config_path.clone()) {
          Ok(mut fc) => {
            println!("Create file: {:?}", config_path.clone());
            let _ = fc.write_all(serde_json::to_string(&api_config_default).unwrap().as_bytes());         
            return Ok(api_config_default);
          },
          Err(err) => panic!("Cannot created file: {:?}", err),
        }
      } 
      Err(err) => panic!("Cannot open file: {:?}", err),
    };
    
    // TODO:ファイル読み込み時のエラーハンドリング
    let mut config_str = String::new();
    match f.read_to_string(&mut config_str) {
        Ok(_) => { println!("Read file: {:?}", config_path.clone()); },
        Err(err) => { panic!("Cannot read file: {:?}", err); }
    }
    
    // println!("config_str: {}", config_str);

    // ファイルの中身をConfig構造体に合わせて展開する
    // let config: Self = serde_json::from_str(&config_str).unwrap();
    // let prec = serde_json::from_str(&config_str);
    // println!("prec: {:?}", prec);
    // let config: Self = prec.unwrap();
    
    let config: Self = serde_json::from_str(&config_str).unwrap();
    return Ok(config);
  }

}
