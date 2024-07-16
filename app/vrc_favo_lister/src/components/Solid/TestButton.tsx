import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api";

export default function TestButton() {
  const [testMsg, setTestMsg] = createSignal("");

  async function testButton() {
    invoke("login").then((result) => {
      // 1が返ってきたらusernameとpasswordの入力を求めるダイアログを出す

      setTestMsg(result as string);
    });
  }

  return (
    <>
        <button onClick={testButton}>testButton</button>
        <p class="row">{testMsg()}</p>
    </>
  );
}
