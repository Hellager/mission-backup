import { createApp } from 'vue'
import dayjs from 'dayjs'
import utc from 'dayjs/plugin/utc.js'
import App from './App.vue'
import router from './router'
import i18n from './locales'
import pinia from './store'
import { errorHandler } from './utils/error'

import 'element-plus/theme-chalk/dark/css-vars.css'
import 'element-plus/theme-chalk/el-message.css'
import 'element-plus/theme-chalk/el-message-box.css'

/**
 * Creates the Vue application instance.
 */
const app = createApp(App)

/**
 * Extends dayjs with UTC plugin.
 */
dayjs.extend(utc)

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
