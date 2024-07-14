import type { unityPackages } from "./unityPackages";

// worldParser で使用する型定義
export interface World {
  id: string; // URLに含まれるID
  name: string; // ワールド名
  description: string; // ワールドの説明
  authorId: string; // 作者のページのURLに含まれるID
  authorName: string; // 作者の名前
  releaseStatus: string; // ワールドの公開状態
  capacity: number; // ワールドの最大人数
  recommendedCapacity: number; // ワールドの推奨人数
  thumbnailImageUrl: string; // ワールドのサムネイル画像
  favorites: number; // お気に入り数
  visits: number; // 訪問数
  updated_at: string; // 更新日時
  unityPackages: unityPackages[]; // unitypackageの情報
}
