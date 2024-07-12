import { listen } from '@tauri-apps/api/event'
import { ElMessage } from 'element-plus'
import { appWindow } from '@tauri-apps/api/window'
import i18n from '../../locales/index.ts'
import { useSystemStore } from '../../store'

/**
 * Listens to the 'instance' event and shows a warning message.
 */
async function listenToInstanceEvent(): Promise<void> {
  await listen('instance', async () => {
    await appWindow.show()
    await appWindow.setFocus()
    ElMessage({
      message: i18n.global.t('warning.anotherInstance'),
      type: 'warning',
    })
  })
}

/**
 * Listens to system theme update event
 */
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
  await listenToInstanceEvent()
  await listenToSysThemeEvent()
}

export {
  listenToEvents,
}
