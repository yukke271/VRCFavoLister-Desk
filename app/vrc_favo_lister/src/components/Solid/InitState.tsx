import { invoke } from "@tauri-apps/api";
import { setIsLogin } from "~/src/components/Solid/StoreState";

export default async function InitState() {
  // todo:ログインする機能を使うかのオプションを読み込む
  // setIsLogin(true);
  
  setIsLogin(await invoke("get_is_login") as boolean);
}

