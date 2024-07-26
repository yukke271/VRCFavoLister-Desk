import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api";

import { isLogin } from "~/src/components/Solid/StoreState";
import InitState from "~/src/components/Solid/InitState";

import { TooltipsGrayOut } from "~/src/components/Solid/TooltipsGrayOut";

export default function NavigationHeader() {
  
  InitState();

  return (
    
    <header class="flex justify-between p-2 border-b-4 border-black">
      <nav class="flex items-center gap-6 font-semibold text-lg">

        {/* トップページに戻るリンク */}
        <a href="/">Home</a>
        
        {/* DBに保存されたお気に入りのリストを表示する機能 */}
        {
          isLogin() ?
          <a href="/favorite/">Favorite</a>
          :
          <TooltipsGrayOut tips="Login required" text="Favorite" />
          // <div />
        }
        
        {/* フレンド関係の機能を呼び出すページ */}
        {/* 
          - noteを編集できる機能とかほしい
        */}
        {
          isLogin() ?
          <a href="/friend/">Friend</a>
          :
          <TooltipsGrayOut tips="Login required" text="Friend" />
          // <div />
        }

        {/* 自身に関する情報を表示するページ */}
        {
          isLogin() ?
          <a href="/profile/">Profile</a>
          :
          <TooltipsGrayOut tips="Login required" text="Profile" />
        }

        {/* 各種設定項目のあるページ */}
        {/* 
          - 設定できるようにしたい項目
          - loginが必須な項目を表示するか
        */}
        <a href="/settings/">Settings</a>

        {/* ドキュメントをまとめてる */}
        <a href="/docs/">Docs</a>

        {/* ログイン状況によって表示されるコンポーネントを切り替えたい */}
        <a href="/login/">Login</a>
      </nav>
    </header>

  );
}
