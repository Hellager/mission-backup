import { ref } from 'vue'
import { defineStore } from 'pinia'
import { relaunch } from '@tauri-apps/api/process'
import type { AppConfig } from '../types'
import { Command, execute } from '../../utils/cmd'
import type { WatcherConfig } from './types'

export const useWatcherStore = defineStore('watcher', () => {
  const timeout = ref<number>(0)

  /**
   * Gets the current watcher configuration.
   * @returns The current watcher configuration.
   */
  function getConfig(): WatcherConfig {
    return {
      timeout: timeout.value,
    }
  }

  /**
   * Sets the watcher configuration.
   * @param config - The watcher configuration to set.
   */
  function setConfig(config: WatcherConfig): void {
    timeout.value = config.timeout
  }

  /**
   * Initializes the watcher with the provided data or fetches the configuration if data is undefined.
   * @param data - The optional watcher configuration data.
   */
  async function init(data: WatcherConfig | undefined): Promise<void> {
    if (data === undefined) {
      await execute(Command.InitConfig, 'watcher')
        .then((config: AppConfig) => {
          setConfig(config.watcher)
        })
    }
    else {
      setConfig(data)
    }
  }

  /**
   * Updates the timeout value for the watcher.
   * @param value - The new timeout value.
   * @param reboot - Whether to reboot after updating the timeout.
   */
  async function updateTimeout(value: number, reboot: boolean): Promise<void> {
    const config = getConfig()
    config.timeout = value

    await execute(Command.UpdateConfig, 'watcher', config)
      .then((config: AppConfig) => {
        setConfig(config.watcher)
      })

    if (reboot)
      await relaunch()
  }

  return {
    timeout,
    init,
    updateTimeout,
  }
})

/**
 * Generates the default watcher configuration.
 * @returns The default watcher configuration.
 */
export function defaultWatcherConfig(): WatcherConfig {
  return {
    timeout: 3,
  }
}
