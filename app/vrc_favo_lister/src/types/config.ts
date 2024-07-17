// 設定ファイルの構造

export interface Config {
  baseUrl: string;
  apiKey: string;
  userAgent: string;
  authCookie: string;
  twoFactorType: string;
  twoFactorAuth: string;
  username: string;
  password: string;
}