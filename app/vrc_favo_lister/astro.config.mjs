import { defineConfig } from "astro/config";
import solidJs from "@astrojs/solid-js";

// https://astro.build/config
export default defineConfig({
  image: {
    domains: ["vrchat.com"],
  },
  integrations: [solidJs()],
});
