import { createPinia } from 'pinia'
import { useStatusStore } from './status'
import type { AppConfig } from './types'
import { defaultSystemConfig, useSystemStore } from './system'
import { defaultScreensaverConfig, useScreensaverStore } from './screensaver'
import { defaultNotifyConfig, useNotifyStore } from './notify'
import { defaultWatcherConfig, useWatcherStore } from './watcher'
import { useMissionStore } from './mission'

const pinia = createPinia()

export {
  useStatusStore,
  useSystemStore,
  useWatcherStore,
  useNotifyStore,
  useScreensaverStore,
  useMissionStore,
}

/**
 * Generates the default application configuration.
 * @returns The default application configuration.
 */
export function defaultAppConfig(): AppConfig {
  return {
    system: defaultSystemConfig(),
    notify: defaultNotifyConfig(),
    watcher: defaultWatcherConfig(),
    screensaver: defaultScreensaverConfig(),
  }
}

export default pinia
