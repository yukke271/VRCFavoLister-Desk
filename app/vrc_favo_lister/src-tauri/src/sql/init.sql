-- FavoriteWorld TransactionTable
CREATE TABLE IF NOT EXISTS FavoriteWorld (
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
CREATE TABLE IF NOT EXISTS FavoriteItemPlatform (
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

-- DatabaseVersionTable
CREATE TABLE IF NOT EXISTS database_version (
    id INTEGER PRIMARY KEY,
    major INTEGER NOT NULL,
    minor INTEGER NOT NULL,
    patch INTEGER NOT NULL
);

-- FavoriteItemPlatform MasterTable Insert
INSERT INTO favorite_item_platform (id, platform) VALUES (1, 'PCOnly');
INSERT INTO favorite_item_platform (id, platform) VALUES (2, 'QuestOnly');
INSERT INTO favorite_item_platform (id, platform) VALUES (3, 'CrossPlatform');

-- DatabaseVersionTable Insert
INSERT INTO database_version (id, major, minor, patch) VALUES (1, 0, 0, 1);


