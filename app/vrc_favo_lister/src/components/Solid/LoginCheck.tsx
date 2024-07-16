// import { createSignal } from "solid-js";
// import { fs,path,invoke } from "@tauri-apps/api";
// import type { Config } from "~/src/types/config";

// export default function LoginCheck() {
  
//   const [config, setConfig] = createSignal<Config>({
//     apiKey: "",
//     id: "",
//     password: "",
//     authCookie: "",
//     twoFactorAuth: "",
//     OTPAuthUrl: ""
//   });  
//   const [isLogin, setisLogin] = createSignal(false);

//   async function loginCheck() {
    
//     let localDataPath = await path.appLocalDataDir();
//     let configPath = await path.resolve(localDataPath, 'config.json' );
//     // ディレクトリが存在しなければ作成
//     await fs.createDir( localDataPath, { recursive: true } );
//     // ファイル読み込み
//     const configStr = await fs.readTextFile( configPath ).catch( ( error: string ) => {
//       // ファイルの読み込みに失敗した場合、エラーを返す
//       throw error;
//     });
//     // ファイルの中身をJSONに変換
//     setConfig( JSON.parse( configStr ) );

//     // ログイン
//     await Login();

//     return setisLogin(await invoke("login_check"));
//   }
  
//   async function Login() {
//     /*
//     let isTwoFactorAuth = false
//     let otpType: string[] = [];
//     await fetch("https://api.vrchat.cloud/api/1/auth/user", {
//         headers: {
//             "Content-Type": "application/json",
//             "Cookie": "apiKey=" + config().apiKey,
//             credentials: "same-origin",
//             // base64(urlencode(username):urlencode(password))  
//             Authorization: 'Basic ' + base64Encode(encodeURIComponent(config().id) + ":" + encodeURIComponent(config().password))
//         },
//     }).then((r) => {
//         DEBUGLOG("Login, user header", r);
//         if (r.status == 200) {
//             authCookie = r.headers.get("Set-Cookie").match(/auth=(.*?);/)[1];
//             fs.writeFileSync("secret/authCookie.txt", authCookie);
//             isLogin = true;
//             return r.json();
//         }
//     }).then((json) => {
//         DEBUGLOG("Login, user", json);
//         if (json.requiresTwoFactorAuth) {
//             isLogin = false;
//             isTwoFactorAuth = true;
//             otpType = json.requiresTwoFactorAuth;
//             console.log("Requires TwoFactorAuth");
//             return;
//         }
//         userData = json;
//     }).catch((e) => {
//     });
//     if (isTwoFactorAuth) {
//         for (let i = 0; i < otpType.length; i++) {
//             // OTP 汚いので書き直したい。
//             let token = config.OTPValue;
//             if (totpObj != null && otpType[i] == "totp") {
//                 token = totpObj.generate();
//             }
//             console.log("Try auth: " + otpType[i] + " / " + token);
//             await fetch("https://api.vrchat.cloud/api/1/auth/twofactorauth/" + otpType[i] + "/verify", {
//                 method: "POST",
//                 headers: {
//                     "Content-Type": "application/json",
//                     "Cookie": "apiKey=" + config.apiKey + "; auth=" + authCookie + "; twoFactorAuth=" + twoFactorAuth
//                 },
//                 body: JSON.stringify({
//                     "code": token
//                 })
//             }).then((r) => {
//                 DEBUGLOG("Login, otp header", r);
//                 if (r.status == 200) {
//                     isLogin = true;
//                     twoFactorAuth = r.headers.get("Set-Cookie").match(/twoFactorAuth=(.*?);/)[1];
//                     fs.writeFileSync("secret/twoFactorAuth.txt", twoFactorAuth);
//                     return r.json();
//                 }
//                 return r.json();
//             }).then((json) => {
//                 if (json == undefined || json.length == 0) return;
//                 DEBUGLOG("Login, otp", json);
//                 if (json.requiresTwoFactorAuth) {
//                     isLogin = false;
//                     console.log("TwoFactorAuth failed...");
//                     return;
//                 }
//                 userData = json;
//             }).catch((e) => {
//                 console.log(e);
//             });
//             if (isLogin) break;
//         }
        
//     }
//         */
//   }

//   function base64Encode(str: string) {
//     const textEncoder = new TextEncoder();
//     const encodeString = (string: string) => textEncoder.encode(string);
//     const decodeBinaryString = (uint8Array: Uint8Array) => uint8Array.reduce(
//       (binaryString, uint8) => binaryString + String.fromCharCode(uint8),
//       '',
//     );

//     const base64str = btoa(decodeBinaryString(encodeString(str)));

//     return base64str;
//   }

//   return (
//     <>
//       <button onClick={loginCheck}>Login Check</button>
//       <p>{isLogin() ? "Login" : "Not Login"}</p>
//     </>
//   );
// }


