import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api";

import { LodingScreen } from "~/src/components/Solid/LoadingScreen";

export default function LoadFavoriteButton() {
  const [loadStatus, setLoadStatus] = createSignal(false);

  async function loadFavoriteButton() {
    setLoadStatus(true);
    
    // デバック用に10秒間ローディングする
    // setTimeout(() => {
    //   setLoadStatus(false);
    // }, 10000);

    await invoke("load_favorite").then((res) => {
      console.log(res as string);
      setLoadStatus(false);
    });

    // await invoke("read_favorite").then((res) => {
    //   console.log(res as string);
    //   setLoadStatus(false);
    // });
  }
  

  /*
    app\vrc_favo_lister\src-tauri\src\commands\load_favorite.rs

    読み込み終了時に、
    読み込んだ複数データの内処理に失敗したデータを返し、
    APIの呼び出しができなかった場合はエラーコードを返す。

  */

  return (
    <>
        {/* 読み込み開始ボタン */}
        <button onClick={loadFavoriteButton}>Load</button>

        {/* 読み込み中に表示するローディング画面 */}
        <LodingScreen isShow={loadStatus()} /> 
        
    </>
  );
}
