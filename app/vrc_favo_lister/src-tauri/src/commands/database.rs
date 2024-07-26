use std::str::FromStr;
use std::ops::DerefMut;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};

use crate::structs::favorite_world::{FavoriteWorldFromAPI};

use crate::structs::app_struct::AppFavoriteWorldCard;

use crate::structs::db_structs::{
    SelectFavoriteWorldTag,
    SelectFavoriteItemPlatform,
};

use crate::commands::utils::{debug_log, get_file_path_str};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// アプリ起動時のDB初期化処理
pub async fn init_db_pool() -> Result<SqlitePool> {
    
    // DBファイルのパスを取得
    let database_path_str = get_file_path_str("db.sqlite");
    // DBファイルの存在確認
    let is_db_exists = std::fs::metadata(&database_path_str).is_ok();

    // 接続オプションの設定
    let connection_options = SqliteConnectOptions::from_str(&database_path_str).unwrap()
        // DBが存在しないなら作成する
        .create_if_missing(true)
        .synchronous(SqliteSynchronous::Normal);
    
    // コネクションプールの作成
    let db_pool:SqlitePool = SqlitePoolOptions::new()
        .connect_with(connection_options)
        .await
        .expect("Failed to create DB pool");

    // DBが存在しない場合はマイグレーションを実行
    if !is_db_exists {
        init_db_migrate(&db_pool).await.expect("Failed to migrate DB");
        debug_log("DB initialized");

        // 動作確認
        let platform_id: u32 = 1;
        let platform: String = select_favorite_item_platform(&db_pool, platform_id).await.expect("Failed to select FavoriteItemPlatform");
        debug_log(format!("platform: {}", platform));
    }

    Ok(db_pool)
}

async fn init_db_migrate(pool: &SqlitePool) -> Result<()> {
    // TODO:マイグレーションを実行してもテーブルが作成されない問題を確認する
    // 正確には、「_sqlx_migrations」テーブルのみ作成されるが、その他のテーブルは作成されない。
    // そのため、マイグレーションファイルを直接実行することでテーブルを作成する必要がある？
    // Resolve:マイグレーションファイルのファイル名の頭に「000_」をつけることで解決した。

    // マイグレーションの実行
    sqlx::migrate!("./src/sql").run(pool).await.unwrap();
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
    debug_log(format!("platform_id: {}", platform_id));
    
    // image_urlからimage_idを取得
    // 例として、image_urlが"https://api.vrchat.cloud/api/1/image/file_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/1/256"の場合、
    // image_idは"file_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/1"となる
    let image_id: String = world.image_url.split("/").skip(6).take(2).collect::<Vec<&str>>().join("/");
    debug_log(format!("image_id: {}", image_id));

    // FavoriteWorldテーブルにワールド情報を挿入する
    sqlx::query(
        "INSERT INTO FavoriteWorld 
        (id, name, description, authorName, releaseStatus, recommendedCapacity, capacity, previewYoutubeId, imageId, publicationDate, updated_at, platform)
        VALUES
        (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) 
        ON CONFLICT(id) DO UPDATE SET 
        name = ?, description = ?, authorName = ?, releaseStatus = ?, recommendedCapacity = ?, capacity = ?, previewYoutubeId = ?, imageId = ?, publicationDate = ?, updated_at = ?, platform = ?
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
        .bind(platform_id.clone())
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
        .bind(platform_id.clone())
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
            SELECT 
                ?, ?
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
pub async fn select_favorite_world(pool: &SqlitePool) -> Result<Vec<AppFavoriteWorldCard>> {

    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to acquire connection");

    let res: Vec<AppFavoriteWorldCard> = sqlx::query_as::<_, AppFavoriteWorldCard>(
        "SELECT
            fw.id,
            fw.name,
            fw.description,
            fw.authorName,
            fw.releaseStatus,
            fw.recommendedCapacity,
            fw.capacity,
            fw.previewYoutubeId,
            fw.imageId,
            fw.publicationDate,
            fw.updated_at,
            fip.platform AS platform
        FROM
            FavoriteWorld fw
        JOIN
            FavoriteItemPlatform fip ON fw.platform = fip.id;
        ")
        .fetch_all(&mut *conn)
        .await
        .expect("Failed to select FavoriteWorld");

    let worlds: Vec<AppFavoriteWorldCard> = res;
    Ok(worlds)
}

// FavoriteWorldTagsテーブルから、world_idと紐づいたタグの一覧を取得する
pub async fn select_favorite_world_tags(pool: &SqlitePool, world_id: String) -> Result<Vec<String>> {
    
    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to acquire connection");
    
    let res = sqlx::query_as::<_, SelectFavoriteWorldTag>(
        "SELECT
            fwt.tags
        FROM
            FavoriteWorldTags fwt
        JOIN
            FavoriteWorldTagMap fwtm ON fwt.id = fwtm.tagId
        WHERE
            fwtm.worldId = ?;",
        )
        .bind(world_id)
        .fetch_all(&mut *conn)
        .await
        .expect("Failed to select FavoriteWorldTags");
    
    let tags: Vec<String> = res.iter().map(|tag| tag.tag.clone()).collect();
    Ok(tags)
}

// FavoriteItemPlatformテーブルから、id(u32)を元にplatform(string)を取得する
pub async fn select_favorite_item_platform(pool: &SqlitePool, platform_id: u32) -> Result<String> {

    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to acquire connection");
    
    let res = sqlx::query_as::<_, SelectFavoriteItemPlatform>(
        "SELECT 
            platform
        FROM
            FavoriteItemPlatform
        WHERE
            id = ?;",
        )
        .bind(platform_id)
        .fetch_one(&mut *conn)
        .await
        .expect("Failed to select FavoriteItemPlatform");
    
    let platform: String = res.platform.clone();
    Ok(platform)
}



// TODO:query_as関数はいつかマクロへの置き換えを考える
// query_asマクロでは、ビルド時には先にsqlx用のSQLクエリキャッシュを作っておかないといけないので、今回は断念。