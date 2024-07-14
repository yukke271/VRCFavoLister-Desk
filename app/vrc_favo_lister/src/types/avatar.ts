import type { unityPackages } from "./unityPackages";

// avatarParser で使用する型定義
export interface Avatar {
  id: string; // URLに含まれるID
  name: string; // アバター名
  description: string; // アバターの説明
  authorId: string; // 作者のページのURLに含まれるID
  authorName: string; // 作者の名前
  releaseStatus: string; // アバターの公開状態
  thumbnailImageUrl: string; // アバターのサムネイル画像
  unityPackages: unityPackages[]; // unitypackageの情報
}
