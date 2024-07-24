use tauri::{api::path::{BaseDirectory, resolve_path}, Env};

use futures::TryStreamExt;

use std::str::FromStr;
use std::ops::DerefMut;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,Row,
};

use crate::structs::favorite_world::FavoriteWorldFromAPI;

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
        println!("DB initialized");

        // 動作確認の実行
        // let _ = select_favorite_world(&db_pool).await.unwrap();
        // println!("DB test passed");
    }

    Ok(db_pool)
}

async fn init_db_migrate(pool: &SqlitePool) -> Result<()> {
    // TODO:マイグレーションを実行してもテーブルが作成されない問題を確認する


    // マイグレーションの実行
    sqlx::migrate!("src/sql").run(pool).await.unwrap();
    Ok(())
}

pub async fn insert_favorite_world(pool: &SqlitePool, world: FavoriteWorldFromAPI) -> Result<()> {
    
    // トランザクションの開始
    let mut tx = pool.begin().await.expect("Failed to begin transaction");

    // platform_idを取得
    /*
        standalonewindowsの文字列が含まれれば1
        androidの文字列が含まれれば2
        両方の文字列が含まれれば3
    */
    let mut platform_id: u32 = 1;
    for platform in world.unity_packages.unwrap_or_default() {
        if platform.platform.contains("windows") {
            if platform_id == 2 {
                platform_id = 3;
            } else {
                platform_id = 1;
            }
        } else if platform.platform.contains("android") {
            if platform_id == 1 {
                platform_id = 3;
            } else {
                platform_id = 2;
            }
        } 
    }
    println!("platform_id: {}", platform_id);

    // image_urlからimage_idを取得
    // 例として、image_urlが"https://api.vrchat.cloud/api/1/image/file_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/1/256"の場合、
    // image_idは"file_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/1"となる
    let image_id: String = world.image_url.split("/").skip(6).take(2).collect::<Vec<&str>>().join("/");
    println!("image_id: {}", image_id);

    // FavoriteWorldテーブルにワールド情報を挿入する
    // sqlx::query(
    //     "INSERT INTO FavoriteWorld 
    //     (id, name, description, authorName, releaseStatus, recommendedCapacity, capacity, previewYoutubeId, imageId, publicationDate, updated_at, platform)
    //     VALUES
    //     (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) 
    //     ON CONFLICT(id) DO UPDATE SET 
    //     name = ?, description = ?, authorName = ?, releaseStatus = ?, recommendedCapacity = ?, capacity = ?, previewYoutubeId = ?, imageId = ?, publicationDate = ?, updated_at = ?, platform = ?"
    //     )
    //     .bind(world.world_id.clone())
    //     .bind(world.world_name.clone())
    //     .bind(world.description.clone())
    //     .bind(world.author_name.clone())
    //     .bind(world.release_status.clone())
    //     .bind(world.recommended_capacity)
    //     .bind(world.capacity)
    //     .bind(world.preview_youtube_id.clone())
    //     .bind(image_id.clone())
    //     .bind(world.publication_date.clone())
    //     .bind(world.updated_at.clone())
    //     .bind(platform_id)
    //     .bind(world.world_name.clone())
    //     .bind(world.description.clone())
    //     .bind(world.author_name.clone())
    //     .bind(world.release_status.clone())
    //     .bind(world.recommended_capacity)
    //     .bind(world.capacity)
    //     .bind(world.preview_youtube_id.clone())
    //     .bind(image_id.clone())
    //     .bind(world.publication_date.clone())
    //     .bind(world.updated_at.clone())
    //     .bind(platform_id)
    //     .execute(tx.deref_mut())
    //     .await
    //     .expect("Failed to insert FavoriteWorld");
    

    sqlx::query(
        "INSERT INTO FavoriteWorld 
        (id, name, description, authorName, releaseStatus, recommendedCapacity, capacity, previewYoutubeId, imageId, publicationDate, updated_at, platform)
        VALUES
        (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) 
        ")
        .bind(world.world_id.clone())
        .bind(world.world_name.clone())
        .bind(world.description.clone())
        .bind(world.author_name.clone())
        .bind(world.release_status.clone())
        .bind(world.recommended_capacity)
        .bind(world.capacity)
        .bind(world.preview_youtube_id.clone())
        .bind(image_id.clone())
        .bind(world.publication_date.clone())
        .bind(world.updated_at.clone())
        .bind(platform_id)
        .execute(tx.deref_mut())
        .await
        .expect("Failed to insert FavoriteWorld");

    for tag in world.tags.unwrap_or_default() {
        // FavoriteWorldTagsテーブルに既存のタグではないタグを挿入する
        sqlx::query("INSERT INTO FavoriteWorldTags (tags) VALUES (?) ON CONFLICT(tags) DO NOTHING")
            .bind(tag.clone())
            .execute(tx.deref_mut())
            .await
            .expect("Failed to insert FavoriteWorldTags");
        
        // FavoriteWorldTagsテーブルからタグの主キーを取得する
        let tag_id: u32 = sqlx::query_scalar("SELECT id FROM FavoriteWorldTags WHERE tags = ?")
            .bind(tag.clone())
            .fetch_one(tx.deref_mut())
            .await
            .expect("Failed to get tag id");

        // FavoriteWorldTagMapテーブルにワールドとタグの関連を挿入する
        sqlx::query(
            "INSERT INTO FavoriteWorldTagMap 
            (worldId, tagId) 
            SELECT ?, ?
            WHERE NOT EXISTS (
                SELECT 1
                FROM FavoriteWorldTagMap
                WHERE worldId = ? AND tagId = ?
            )")
            .bind(world.world_id.clone())
            .bind(tag_id.clone())
            .bind(world.world_id.clone())
            .bind(tag_id.clone())
            .execute(tx.deref_mut())
            .await
            .expect("Failed to insert FavoriteWorldTagMap");
    }

    // トランザクションのコミット
    tx.commit().await.expect("Failed to commit transaction");
    
    Ok(())
}

// FavoriteWorldテーブルからワールド情報を取得する
// todo:いずれマクロ版を使用する...
pub async fn select_favorite_world(pool: &SqlitePool) -> Result<Vec<FavoriteWorldFromAPI>> {
    
    // 下記のquery_as関数を使用したコードでは、「the trait bound `&mut &Pool<Sqlite>: Executor<'_>` is not satisfied」というエラーが発生するため、別の方法を考える
    // let worlds: Vec<FavoriteWorldFromAPI> = sqlx::query_as(
    //         "SELECT * FROM FavoriteWorld"
    //     )
    //     .fetch_all(&mut pool)
    //     .await
    //     .expect("Failed to select FavoriteWorld");
    // Ok(worlds)

    // query_asマクロでは、ビルド時には先にsqlx用のSQLクエリキャッシュを作っておかないといけないので、今回は断念。

    // query関数を使用して、Rowを取得する方法でどうにかする
    const SQL1: &str = "SELECT * FROM FavoriteWorld";
    let mut rows = sqlx::query(SQL1).fetch(pool);
    let mut worlds: Vec<FavoriteWorldFromAPI> = Vec::new();
    while let Ok(Some(row)) = rows.try_next().await {
        let world = FavoriteWorldFromAPI {
            world_id: row.get(0),
            world_name: row.get(1),
            description: row.get(2),
            author_name: row.get(3),
            release_status: row.get(4),
            recommended_capacity: row.get(5),
            capacity: row.get(6),
            preview_youtube_id: row.get(7),
            image_url: row.get(8),
            publication_date: row.get(9),
            updated_at: row.get(10),
            tags: None,
            unity_packages: None,
        };
        worlds.push(world);
    }

    Ok(worlds)
}