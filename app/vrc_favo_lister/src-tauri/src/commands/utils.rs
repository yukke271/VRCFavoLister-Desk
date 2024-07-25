// 細かい便利な関数をまとめたファイル

use tauri::{api::path::{BaseDirectory, resolve_path}, Env, generate_context};

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
