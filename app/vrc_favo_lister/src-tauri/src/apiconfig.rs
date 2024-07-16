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
  pub basic_credentials: Option<String>,
  pub username: Option<String>,
  pub password: Option<String>,
}

impl APIConfig {
  
  pub fn new() -> Self {
    APIConfig::load_config_file().expect("REASON")
  }

  fn load_config_file() -> Result<Self> {  

    let api_config_default = Self {
      base_url: "https://api.vrchat.cloud/api/1/".to_string(),
      api_key: "JlE5Jldo5Jibnk5O5hTx6XVqsJu4WJ26".to_string(),
      user_agent: "vrc_fabo_lister/0.1".to_string(),
      basic_credentials: None,
      auth_cookie: None,
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

    println!("config_path: {:?}", config_path);

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
            fc
          },
          Err(err) => panic!("Cannot created file: {:?}", err),
        }
      } 
      Err(err) => panic!("Cannot open file: {:?}", err),
    };
    
    let mut config_str = String::new();
    f.read_to_string(&mut config_str)
        // ファイルの読み込み中に問題がありました
        .expect("something went wrong reading the file");
    
    println!("config_str: {}", config_str);

    // ファイルの中身をConfig構造体に合わせて展開する
    // let config: Self = serde_json::from_str(&config_str).unwrap();
    let prec = serde_json::from_str(&config_str);
    println!("prec: {:?}", prec);
    let config: Self = prec.unwrap();
    
    return Ok(config);
  }

}
