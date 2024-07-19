---
layout: ../layouts/LayoutMarkDown.astro
title: ぶいちゃふぁぼりすについての内部資料
author: yukke271
description: 使用フレームワークやDB設計だったり機能の詳細について記載していきます。
---

# ぶいちゃふぁぼりすについての内部資料

このページはMarkdownで記述されています。<br />
問題が見つかった場合は
  <a 
    class="extlink" 
    href="https://github.com/yukke271/VRCFavoLister-Desk/discussions" 
    target="_blank"> 
    Github 
  </a> 
よりご連絡ください。

<br />

<h2> 免責事項 </h2>
本ツールはVRChat公式チームと一切関係のない非公式ツールです。<br />
公式にはサポートのされていないAPIを使用する関係上、過度な機能の呼び出し等でアカウントがBANされてしまう可能性に留意して自己責任でご利用ください。<br />
開発時に「Photon」と呼ばれるゲーム内クライアント専用のAPIは使用しておらず、過度なAPIの呼び出しが抑えられるように努めておりますが、開発者は本ツールの利用によって生じた損害等の一切の責任を負いかねます。ご了承ください。<br />

以下、VRChat APIの使用に関するVRChatチーム(Tupper氏)の公式な回答です。
```
Use of the API using applications other than the approved methods (website, VRChat application) are not officially supported. You may use the API for your own application, but keep these guidelines in mind:
● We do not provide documentation or support for the API.
● Do not make queries to the API more than once per 60 seconds.
● Abuse of the API may result in account termination.
● Access to API endpoints may break at any given time, with no warning.
```

___

目次
- [機能詳細](#機能詳細)
- [追加予定の機能](#追加予定の機能)
- [使用フレームワーク・ライブラリ・言語・ツール](#使用フレームワーク・ライブラリ・言語・ツール)
- [DB設計](#DB設計)
- [免責事項](#免責事項)
___

## 機能詳細

後で書きます...
___

## 追加予定の機能

- お気に入りのjsonインポート
- ソート
- タグ付け
- 自身へのSend Invite
___

## 使用フレームワーク・ライブラリ・言語・ツール

- Docker(DockerDesktop)
- VSCode
- Tauri
- Rust
- JavaScript(TypeScript)
- Astro
- SolidJS
- Tailwind CSS
___

## DB設計

<!-- このAPIは単体のIDしか取得できないので却下 -->
<!-- 
FavoriteList GET /favorites <br />
※tagsのstringには、「avatarsN」「worldsN」「group_N」の三種類が一つだけ格納されることを確認。また、Nには1~4の数値が入るのを確認。

| id | type | favoriteId | tags |
|:--|:--|:--|:--|
| fvrt_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx | friend | usr_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx | ["string"] |
| fvrt_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx | world | wrld_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx | ["string"] |
| fvrt_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx | avatar | avtr_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx | ["string"] | -->

<!-- GET worlds/favorites?n=N&offset=N -->
FavoriteWorld TransactionTable  <br />
| id | name | description | authorName | releaseStatus | recommendedCapacity | capacity | previewYoutubeId | imageId | publicationDate | updated_at | platform |
|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|
| wrld_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx | worldNameA | world description |userA | pubric | 1 | 1 | null | file_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx | 20xx-01-02T01:01:01.001Z | 20xx-01-01T01:01:01.001Z | 1 |
| wrld_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx | worldNameB | comment |userB | private | 4 | 0 | xxxxxxxxxxx | file_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx | 20xx-01-03T01:01:01.001Z | 20xx-01-04T01:01:01.001Z | 3 |
| ??? | ??? | comment | ??? | hidden | 0 | 0 | null | https: //assets.vrchat.com/default/unavailable-world.png | null | null | 1 |
<br />

FavoriteItemPlatform MasterTable <br />
| id | platform |
|:--|:--|
| 1 | PCOnly |
| 2 | QuestOnly |
| 3 | CrossPlatform |
<br />

FavoriteWorldTags AutoInsertTable <br />
| id | tags |
|:--|:--|
| 1 | system_approved |
| 2 | system_monetized_world |
| 3 | author_tag_avatar |
| 4 | author_tag_game |
<br />

FavoriteWorldTagMap TagMapTable <br />
| id | worldId | tags |
|:--|:--|:--|
| 1 | wrld_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxx1 | 1 |
| 2 | wrld_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxx2 | 1 | 
| 3 | wrld_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxx2 | 4 |
<br />

DatabaseVersion <br />
| minor | minor | patch |
|:--|:--|:--|
| 1 | 0 | 0 |
<br />

___


## 免責事項
本ツールはVRChat公式チームと一切関係のない非公式ツールです。<br />
公式にはサポートのされていないAPIを使用する関係上、過度な機能の呼び出し等でアカウントがBANされてしまう可能性に留意して自己責任でご利用ください。<br />
開発時に「Photon」と呼ばれるゲーム内クライアント専用のAPIは使用しておらず、過度なAPIの呼び出しが抑えられるように努めておりますが、開発者は本ツールの利用によって生じた損害等の一切の責任を負いかねます。ご了承ください。<br />

以下、VRChat APIの使用に関するVRChatチーム(Tupper氏)の公式な回答です。
```
Use of the API using applications other than the approved methods (website, VRChat application) are not officially supported. You may use the API for your own application, but keep these guidelines in mind:
● We do not provide documentation or support for the API.
● Do not make queries to the API more than once per 60 seconds.
● Abuse of the API may result in account termination.
● Access to API endpoints may break at any given time, with no warning.
```
___

© 2024 yukke271(@yukke_vrc) All Rights Reserved.



<style>
  h1 { 
    padding: 0.5rem 0 0.5rem;
  }
  ul {
    list-style-type: disc;
    padding-left: 2rem;
  }
  li a,.extlink{
    text-decoration-line: underline;
    color: rgb(59, 130, 246);
  }
  table, td, th {
    border-collapse: collapse;
    border: 2px #000000 solid;
  }

</style>