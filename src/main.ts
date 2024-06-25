import { createApp } from 'vue'
import './styles.css'
import App from './App.vue'
import router from './router'

/**
 * Creates the Vue application instance.
 */
const app = createApp(App)

/**
 * Uses the router plugin.
 */
app.use(router)

/**
 * Mounts the application to the specified element with id 'app'.
 */
app.mount('#app')
