import Aura from "@primevue/themes/aura";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2024-11-01",
  devtools: { enabled: true },
  modules: ["@nuxt/content", "@primevue/nuxt-module", "@nuxt/eslint"],
  primevue: {
    options: {
      theme: {
        preset: Aura,
      },
    },
  },
});
