-- FavoriteWorld TransactionTable
CREATE TABLE IF NOT EXISTS favorite_world (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    authorName TEXT NOT NULL,
    releaseStatus TEXT CHECK(releaseStatus IN ('public', 'private', 'hidden', 'all')) NOT NULL,
    recommendedCapacity INTEGER NOT NULL,
    capacity INTEGER NOT NULL,
    previewYoutubeId TEXT,
    imageId TEXT NOT NULL,
    publicationDate TEXT,
    updated_at TEXT NOT NULL,
    platform INTEGER NOT NULL,
    FOREIGN KEY (platform) REFERENCES FavoriteItemPlatform(id)
);

-- FavoriteItemPlatform MasterTable
CREATE TABLE IF NOT EXISTS favorite_item_platform (
    id INTEGER PRIMARY KEY,
    platform TEXT NOT NULL
);

-- FavoriteWorldTags AutoInsertTable
CREATE TABLE FavoriteWorldTags (
    id INTEGER PRIMARY KEY,
    tags TEXT NOT NULL
);

-- FavoriteWorldTagMap TagMapTable
CREATE TABLE FavoriteWorldTagMap (
    id INTEGER PRIMARY KEY,
    worldId TEXT NOT NULL,
    tagId INTEGER NOT NULL,
    FOREIGN KEY (worldId) REFERENCES FavoriteWorld(id),
    FOREIGN KEY (tagId) REFERENCES FavoriteWorldTags(id)
);
