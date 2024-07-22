use tauri::{api::path::{BaseDirectory, resolve_path}, Env};

use std::str::FromStr;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn init_db_pool() -> Result<SqlitePool> {
    // DBファイルのパスを取得
    let context = tauri::generate_context!();
    let database_path = resolve_path(
        context.config(),
        context.package_info(),
        &Env::default(),
        "db.sqlite",
        Some(BaseDirectory::AppLocalData))
      .unwrap();

    // PathbufをStringに変換
    let database_path_str = database_path.into_os_string().into_string().unwrap();
    
    // DBファイルの存在確認
    let is_db_exists = std::fs::metadata(&database_path_str).is_ok();

    // 接続オプションの設定
    let connection_options = SqliteConnectOptions::from_str(&database_path_str).unwrap()
        // DBが存在しないなら作成する
        .create_if_missing(true)
        .synchronous(SqliteSynchronous::Normal);
    
    // コネクションプールの作成
    let db_pool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await
        .unwrap();

    // DBが存在しない場合はマイグレーションを実行
    if !is_db_exists {
        init_db_migrate(&db_pool).await.unwrap();
    }

    Ok(db_pool)
}

async fn init_db_migrate(pool: &SqlitePool) -> Result<()> {
    sqlx::migrate!("src/database/sql").run(pool).await.unwrap();
    Ok(())
}