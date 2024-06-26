import { createApp } from 'vue'
import './styles.css'
import App from './App.vue'
import router from './router'
import i18n from './locales'
import pinia from './store'
import { errorHandler } from './utils/error'

import 'element-plus/theme-chalk/el-message.css'

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
 * Uses the Pinia store plugin.
 */
app.use(pinia)

/**
 * Sets the error handler for the application configuration.
 */
app.config.errorHandler = errorHandler

/**
 * Mounts the application to the specified element with id 'app'.
 */
app.mount('#app')
