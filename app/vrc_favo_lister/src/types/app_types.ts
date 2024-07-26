// バックエンドとのやり取りで受け取るデータの型定義

export interface AppFavoriteWorldCard {
  id: string;
  name: string;
  description: string;
  authorName: string;
  releaseStatus: string;
  recommendedCapacity: number;
  capacity: number;
  previewYoutubeId: string | null;
  imageId: string;
  publicationDate: string;
  updated_at: string;
  platform: string;
}
