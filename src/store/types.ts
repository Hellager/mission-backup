import type { SystemConfig } from './system/types'
import type { ScreensaverConfig } from './screensaver/types'
import type { NotifyConfig } from './notify/types'
import type { WatcherConfig } from './watcher/types'

/**
 * Represents the application configuration.
 */
export interface AppConfig {
  system: SystemConfig
  notify: NotifyConfig
  watcher: WatcherConfig
  screensaver: ScreensaverConfig
}
