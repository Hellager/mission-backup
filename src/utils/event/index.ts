import { listen } from '@tauri-apps/api/event'
import { useSystemStore } from '../../store'

async function listenToSysThemeEvent(): Promise<void> {
  await listen('sys_theme', (event: any) => {
    if (event.payload.code === 200) {
      const store = useSystemStore()

      store.updateSysTheme(event.payload.data)
    }
  })
}

/**
 * Listens to multiple events by calling individual event listeners.
 */
async function listenToEvents(): Promise<void> {
  await listenToSysThemeEvent()
}

export {
  listenToEvents,
}
