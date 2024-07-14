import type { World } from "~/src/types/world";

/**
 * @/utils/parser/worldsParser.ts
 *
 * 受け取ったjson配列をWorld型の配列に変換する
 * @param worldList World型の配列
 * @param json json配列
 */
export const worldsParser = (worldList: World[], json: string) => {
  // 受け取ったjson配列をパースする
  let worldsJson: World[] = [];
  try {
    worldsJson = JSON.parse(json);
  } catch (error) {
    throw new Error("データの読み取りに失敗しました");
  }

  worldsJson.forEach((world: World) => {
    worldList.push(world);
  });
};
