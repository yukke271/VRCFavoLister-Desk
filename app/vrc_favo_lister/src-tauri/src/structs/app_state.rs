use std::{mem, sync::Mutex};
use tauri::async_runtime::block_on;
use sqlx::SqlitePool;
use crate::commands::database::init_db_pool;
use crate::commands::utils::debug_log;
use crate::structs::apiconfig::APIConfig;

#[derive(Debug)]
pub(crate) struct AppState {
  pub(crate) context: Mutex<Context>,
}

impl AppState {
  pub(crate) fn new() -> Self {
    AppState {
      context: Mutex::new(Context::new()),
    }
  }
}

pub(crate) trait ContextTrait {
  fn get_api_config(&self) -> APIConfig;
  fn set_api_config(&mut self, api_config: APIConfig);
}

#[derive(Debug)] 
pub(crate) struct Context {
  pub(crate) db_pool: SqlitePool,
  api_config: APIConfig,
}

impl ContextTrait for Context {
  fn get_api_config(&self) -> APIConfig {
    self.api_config.clone()
  }
  fn set_api_config(&mut self, api_config: APIConfig) {
    let _ = api_config.save_config_file();
    let _ = mem::replace(&mut self.api_config, api_config);
  }
}

impl Context {
  pub(crate) fn new() -> Self {
    check_app_data_dir();
    let api_config = APIConfig::new();
    let db_pool = block_on(init_db_pool()).expect("error while initializing database pool");
    Context {
      db_pool,
      api_config,
    }
  }
}


// 設定ファイルなどを格納するディレクトリが存在しない場合、作成する
fn check_app_data_dir() {
  use tauri::{api::path::{BaseDirectory, resolve_path}, Env};

  let context = tauri::generate_context!();
  let app_data_dir = resolve_path(
    context.config(),
    context.package_info(),
    &Env::default(),
    "",
    Some(BaseDirectory::AppLocalData)
  ).unwrap();
  if !app_data_dir.exists() {
    std::fs::create_dir_all(app_data_dir.clone()).expect("Cannot create app data directory");
  }
  debug_log(format!("App data directory: {:?}", app_data_dir.clone()));
}

