import type { Avatar } from "~/src/types/avatar";

/**
 * @/utils/parser/avatarsParser.ts
 *
 * 受け取ったjson配列をAvatar型の配列に変換する
 * @param avatarList Avatar型の配列
 * @param json json配列
 */
export const avatarsParser = (avatarList: Avatar[], json: string) => {
  // 受け取ったjson配列をパースする
  let avatarsJson: Avatar[] = [];
  try {
    avatarsJson = JSON.parse(json);
  } catch (error) {
    throw new Error("データの読み取りに失敗しました");
  }

  avatarsJson.forEach((avatar: Avatar) => {
    avatarList.push(avatar);
  });
};
