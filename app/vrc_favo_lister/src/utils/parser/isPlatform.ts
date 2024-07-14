import { devLog } from '~/src/utils';

/**
 * @/utils/parser/isPlatform.ts
 *
 * 受け取ったunityPackages配列とキーワード(PC or Quest)からBooleanを返す
 * @param unityPackages unityPackages配列
 * @returns true or false
 */

export const isPlatform = ( unityPackages: { platform: string; }[] , keyword : string ) => {
  if (unityPackages.length >= 1) {
    unityPackages.forEach((unityPackages: { platform: string; }) => {
      if (unityPackages.platform === "standalonewindows" && keyword === "PC") return true;
      if (unityPackages.platform === "android" && keyword === "Quest") return true;      
    });
  } else {
    devLog(unityPackages.length.toString());
  }
  return false;
}
