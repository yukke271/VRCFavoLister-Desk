import { defineConfig } from "astro/config";
import solidJs from "@astrojs/solid-js";
import tailwind from "@astrojs/tailwind";

// https://astro.build/config
export default defineConfig({
  vite: {
    server: {
      watch: {
        // viteの設定からポーリングを有効にすることでWSL外部のファイル監視を有効にする
        usePolling: true
      }
    }
  },
  image: {
    domains: ["vrchat.com"]
  },
  integrations: [solidJs(), tailwind()]
});