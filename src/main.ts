import { createApp } from 'vue'
import './styles.css'
import App from './App.vue'
import router from './router'
import i18n from './locales'

/**
 * Creates the Vue application instance.
 */
const app = createApp(App)

/**
 * Uses the router plugin.
 */
app.use(router)

/**
 * Uses the i18n plugin.
 */
app.use(i18n)

/**
 * Mounts the application to the specified element with id 'app'.
 */
app.mount('#app')
