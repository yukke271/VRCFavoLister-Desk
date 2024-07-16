// import { createSignal } from "solid-js";
// import { fs,path } from "@tauri-apps/api";
// import type { Config } from "~/src/types/config";
// import { set } from "astro/zod";

// export default function LoginForm() {
  
//   const [config, setConfig] = createSignal<Config>({
//     apiKey: "",
//     id: "",
//     password: "",
//     authCookie: "",
//     twoFactorAuth: "",
//     OTPAuthUrl: ""
//   });
//   const [twoFactorCode, setTwoFactorCode] = createSignal("");
//   const [authResponse, setAuthResponse] = createSignal("");
//   const [error, setError] = createSignal("");
  
//   const handleAuth = async () => {

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
    
//     try {
      
//       const base64Credentials = base64Encode(encodeURIComponent(config().id) + ":" + encodeURIComponent(config().password))

//       console.log(base64Credentials);

//       const authResponse = await fetch('https://api.vrchat.cloud/api/1/auth/user', {
//         mode: 'cors',
//         method: 'GET',
//         headers: {
//           "Content-Type": "application/json",
//           'Authorization': 'Basic ' + base64Credentials,
//           // "Cookie": "apiKey=" + config().apiKey,
//         },
//         credentials: 'include'
//       });
//       const authData = await authResponse.json();

//       console.log(authData);

//       if (authData.requiresTwoFactorAuth && authData.requiresTwoFactorAuth.includes('totp')) {
//         const twoFactorResponse = await fetch('https://api.vrchat.cloud/api/1/auth/twofactorauth/totp/verify', {
//           mode: 'cors',
//           method: 'POST',
//           headers: {
//             'Accept': 'application/json',
//             'Content-Type': 'application/json'
//           },
//           credentials: 'include',
//           body: JSON.stringify({ code: twoFactorCode })
//         });
//         const twoFactorData = await twoFactorResponse.json();
        
//         console.log(twoFactorData);

//         setAuthResponse(twoFactorData);
//       } else {
//         setAuthResponse(authData);
//       }
//     } catch (e) {
//       console.log(typeof(e));
//       console.log((e as Object).toString());
//       setError((e as Object).toString());
//     }
//   };

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
//       <input
//         type="text"
//         placeholder="2FA Code"
//         value={twoFactorCode()}
//         onChange={(e) => setTwoFactorCode(e.target.value)}
//       />
//       <button onClick={handleAuth}>Authenticate</button>
//       <p>{authResponse()}</p>
//       <p>{error()}</p>
//     </>
//   );
// }


