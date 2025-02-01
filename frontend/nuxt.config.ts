import Aura from "@primevue/themes/aura";
import TailwindTypography from "@tailwindcss/typography";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-11-01",
  devtools: { enabled: true },
  modules: [
    "@nuxt/content",
    "@primevue/nuxt-module",
    "@nuxtjs/tailwindcss",
    "@nuxt/eslint",
  ],
  primevue: {
    options: {
      theme: {
        preset: Aura,
      },
    },
  },
  tailwindcss: {
    config: {
      plugins: [TailwindTypography],
    },
  },
});
