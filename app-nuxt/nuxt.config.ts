// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },

  css: ['vuetify/lib/styles/main.sass',
  // '@mdi/font/css/materialdesignicons.min.css',
],
  build: {
    transpile: ['vuetify'],
  },
  vite: {
    define: {
      'process.env.DEBUG': false,
    },
  },

  app: {
      head: {
        charset: 'utf-8',
        viewport: 'width=device-width, initial-scale=1',
      }
    }
})
