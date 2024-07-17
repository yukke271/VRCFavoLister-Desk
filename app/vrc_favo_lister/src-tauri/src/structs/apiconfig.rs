use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::{env, fs::File, io::{ErrorKind, Read, Write}};
use tauri::{api::path::{BaseDirectory, resolve_path}, Env};

/*
  app/vrc_favo_lister/src-tauri/src/apiconfig.rs
  APIConfig構造体の定義と実装
  APIConfigはAPI呼び出しに関する設定情報を保持する構造体
  
  APIConfig::new()
  - Configファイルを読み込み、APIConfigを返す

  APIConfig::save_config_file()
  - ConfigファイルにAPIConfigを保存する

*/

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
    APIConfig::load_config_file().expect("Cannot load config file")
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
    // println!("Save config: {:?}", self);

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
    // ファイルが見つからない場合はデフォルト値でファイルを作成し、APIConfigを返す
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
    
    // 正常にファイルを読み込めた場合ここに到達する
    let mut config_str = String::new();
    match f.read_to_string(&mut config_str) {
        Ok(_) => { println!("Read file: {:?}", config_path.clone()); },
        Err(err) => { panic!("Cannot read file: {:?}", err); }
    }
    // println!("config_str: {}", config_str);

    // ファイルの中身をConfig構造体に合わせて展開する
    let config: Self = serde_json::from_str(&config_str).unwrap();
    // println!("config: {:?}", config);

    return Ok(config);
  }

}
