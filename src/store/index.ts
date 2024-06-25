import { createPinia } from 'pinia'
import { useStatusStore } from './status'

const pinia = createPinia()

export {
  useStatusStore,
}

export default pinia
