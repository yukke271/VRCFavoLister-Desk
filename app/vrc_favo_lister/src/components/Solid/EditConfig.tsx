// import { createSignal } from "solid-js";
// import { fs,path } from '@tauri-apps/api';
// import type { Config } from "~/src/types/config";

// export default function EditConfig() {

//   const [config, setConfig] = createSignal<Config>({
//     apiKey: "",
//     id: "",
//     password: "",
//     authCookie: "",
//     twoFactorAuth: "",
//     OTPAuthUrl: ""
//   });

//   async function loadConfig() {

//     let localDataPath = await path.appLocalDataDir();
//     let configPath = await path.resolve(localDataPath, 'config.json' );
//     // ディレクトリが存在しなければ作成
//     await fs.createDir( localDataPath, { recursive: true } );

//     // ファイル読み込み
//     const configStr = await fs.readTextFile( configPath ).catch( ( error: string ) => {
//       if ( error.includes( 'os error 2' ) ) {
//         // ファイルが存在しない場合は初期設定項目が入ったJSON形式の文字列を返す
//         const config = `{
//           "apiKey": "JlE5Jldo5Jibnk5O5hTx6XVqsJu4WJ26",
//           "id": "dummyid",
//           "password": "dummypassword"
//         }`;
//         return config;
//       } else {
//         // TODO:それ以外のエラーはthrowしている
//         throw error;
//       }
//     });

//     // ファイルの中身をJSONに変換
//     setConfig( JSON.parse( configStr ) );

//     // input id, passwordに値をセット
//     (document.getElementById('input-id') as HTMLInputElement).value = config().id;
//     (document.getElementById('input-password') as HTMLInputElement).value = config().password;

//   }

//   async function saveConfig() {

//     let localDataPath = await path.appLocalDataDir();
//     let configPath = await path.resolve(localDataPath, 'config.json' );
//     // ディレクトリが存在しなければ作成
//     await fs.createDir( localDataPath, { recursive: true } );

//     // ファイルを保存
//     await fs.writeFile( { path: configPath, contents: JSON.stringify( config() ) } );
    
//   }
  

//   return (
//     <>
//       <button onClick={loadConfig}>load ConfigFile</button>
//       <button onClick={saveConfig}>Save ConfigFile</button>

//       <input
//           id="input-id"
//           onChange={(e) => setConfig((prev) => ({ ...prev, id: e.currentTarget.value, password: config().password }))}
//           placeholder="Enter a your id..."
//         />
//       <input
//           id="input-password"
//           onChange={(e) => setConfig((prev) => ({ ...prev, id: config().id, password: e.currentTarget.value }))}
//           placeholder="Enter a your password..."
//         />
//     </>
//   );
  
// }
