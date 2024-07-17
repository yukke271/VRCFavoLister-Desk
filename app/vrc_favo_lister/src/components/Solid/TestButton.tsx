import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api";

export default function TestButton() {
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [otpCode, setOtpCode] = createSignal("");
  const [testMsg, setTestMsg] = createSignal("");

  async function testButton() {
    invoke("login", { 
      username: username(),
      password: password(),
      otpCode: otpCode()
    }).then((result) => {
      // 1が返ってきたらusernameとpasswordの入力を求めるダイアログを出す

      setTestMsg(result as string);
    });
  }
  /*
   エラーコードを返す
    0: 成功
    1: なんらかの処理に失敗した
    2: usernameかpasswordが空
    3: メールボックスの確認が必要な2段階認証 
    4: 認証アプリによる2段階認証
    5: 2段階認証のコードが空
    6: 何らかの理由でログインに失敗した
  */

  return (
    <>
        <button onClick={testButton}>testButton</button>

        <div {...(testMsg() == "0" ? {} : {class: "none-visible"})} >
          <p> Login Success!! </p>
        </div>

        <div {...(testMsg() == "1" ? {} : {class: "none-visible"})} >
          <p> Login Failed!! </p>
        </div>

        <div {...(testMsg() == "2" ? {} : {class: "none-visible"})} >
          <p> ユーザーIDもしくはメールアドレスと、パスワードを入力してください </p>
          <input onChange={(e) => setUsername(e.currentTarget.value)} type="text" placeholder="enter a username or email ..."/>
          <input onChange={(e) => setPassword(e.currentTarget.value)} type="password" placeholder="enter a password ..."/>
        </div>

        <div {...(testMsg() == "3" ? {} : {class: "none-visible"})} >
          <p> 2段階認証コードをメールボックスから確認してください </p>
          <input onChange={(e) => setOtpCode(e.currentTarget.value)} type="text" pattern="[0-9]" maxlength="6"  placeholder="enter a one time password ..."/>
        </div>
        
        <div {...(testMsg() == "4" ? {} : {class: "none-visible"})} >
          <p> 2段階認証コードを認証アプリから確認してください </p>
          <input onChange={(e) => setOtpCode(e.currentTarget.value)} type="text" pattern="[0-9]" maxlength="6" placeholder="enter a one time password ..."/>
        </div>

        <div {...(testMsg() == "5" ? {} : {class: "none-visible"})} >
          <p> 前回の入力時に空でした。<br /> 6桁の正確な2段階認証コードを入力してください </p>
          <input onChange={(e) => setOtpCode(e.currentTarget.value)} type="text" pattern="[0-9]" maxlength="6" placeholder="enter a one time password ..."/>
        </div>
    </>
  );
}
