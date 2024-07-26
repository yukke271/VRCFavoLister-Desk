import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api";
import { LodingScreen } from "~/src/components/Solid/LoadingScreen";
import type { AppFavoriteWorldCard } from "~/src/types/app_types";
import WorldCard  from "~/src/components/Solid/WorldCard";

export default function LoadFavoriteButton() {
  const [loadStatus, setLoadStatus] = createSignal(false);
  const [favoriteList, setFavoriteList] = createSignal([] as AppFavoriteWorldCard[]);

  async function readFavoriteButton() {
    setLoadStatus(true);
    
    await invoke("read_favorite").then((res) => {
      console.log(res);
      setFavoriteList(res as AppFavoriteWorldCard[]);
      setLoadStatus(false);
    });
  }
  
  // 表示テスト用のダミーデータ

  // setFavoriteList([
  //   {
  //     id: "wrld_12345678-1234-1234-1234-123456789012",
  //     name: "Test World",
  //     description: "This is a test world.",
  //     authorName: "Test Author",
  //     releaseStatus: "public",
  //     recommendedCapacity: 16,
  //     capacity: 16,
  //     previewYoutubeId: null,
  //     imageId: "file_12345678-1234-1234-1234-123456789012/1",
  //     publicationDate: "2021-10-01T00:00:00.000Z",
  //     updated_at: "2021-10-01T00:00:00.000Z",
  //     platform: "PCOnly"
  //   },
  //   {
  //     id: "wrld_12345678-1234-1234-1234-123456789012",
  //     name: "Test World",
  //     description: "This is a test world.",
  //     authorName: "Test Author",
  //     releaseStatus: "public",
  //     recommendedCapacity: 16,
  //     capacity: 16,
  //     previewYoutubeId: null,
  //     imageId: "file_12345678-1234-1234-1234-123456789012/1",
  //     publicationDate: "2021-10-01T00:00:00.000Z",
  //     updated_at: "2021-10-01T00:00:00.000Z",
  //     platform: "PCOnly"
  //   },
  //   {
  //     id: "wrld_12345678-1234-1234-1234-123456789012",
  //     name: "Test World",
  //     description: "This is a test world.",
  //     authorName: "Test Author",
  //     releaseStatus: "public",
  //     recommendedCapacity: 16,
  //     capacity: 16,
  //     previewYoutubeId: null,
  //     imageId: "file_12345678-1234-1234-1234-123456789012/1",
  //     publicationDate: "2021-10-01T00:00:00.000Z",
  //     updated_at: "2021-10-01T00:00:00.000Z",
  //     platform: "PCOnly"
  //   },
  //   {
  //     id: "wrld_12345678-1234-1234-1234-123456789012",
  //     name: "Test World",
  //     description: "This is a test world.",
  //     authorName: "Test Author",
  //     releaseStatus: "public",
  //     recommendedCapacity: 16,
  //     capacity: 16,
  //     previewYoutubeId: null,
  //     imageId: "file_12345678-1234-1234-1234-123456789012/1",
  //     publicationDate: "2021-10-01T00:00:00.000Z",
  //     updated_at: "2021-10-01T00:00:00.000Z",
  //     platform: "PCOnly"
  //   }
  // ]);
  
  return (
    <>
        {/* 読み込み開始ボタン */}
        <button onClick={readFavoriteButton}>Load</button>

        {/* 読み込み中に表示するローディング画面 */}
        <LodingScreen isShow={loadStatus()} /> 

        {/* お気に入りリスト */} 
        <WorldCard items={favoriteList()} />
        
    </>
  );
}
