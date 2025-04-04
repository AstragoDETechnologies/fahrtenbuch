import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
	compatibilityDate: "2024-04-03",
	devtools: { enabled: true },
	css: ["~/assets/css/main.css"],

	runtimeConfig: {
		public: {
			api_url: process.env.API_URL,
		},
	},

	vite: {
		plugins: [tailwindcss()],
	},

	modules: ["@nuxt/icon"],
});
