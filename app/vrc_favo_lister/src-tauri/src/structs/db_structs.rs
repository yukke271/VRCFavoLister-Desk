// DB操作用の構造体を定義する

use sqlx::{FromRow};

#[derive(Debug, FromRow)]
pub(crate) struct SelectFavoriteWorldTag {
  pub tag: String,
}

#[derive(Debug, FromRow)]
pub(crate) struct SelectFavoriteItemPlatform {
  pub platform: String,
}