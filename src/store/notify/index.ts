import { ref } from 'vue'
import { defineStore } from 'pinia'
import { isPermissionGranted, requestPermission } from '@tauri-apps/api/notification'
import type { AppConfig } from '../types'
import { Command, execute } from '../../utils/cmd'
import type { NotifyConfig } from './types'

export const useNotifyStore = defineStore('notify', () => {
  const isGranted = ref<boolean>(false)
  const enable = ref<boolean>(false)
  const whenCreate = ref<boolean>(false)
  const whenFailed = ref<boolean>(false)

  /**
   * Get the current notification configuration.
   * @returns The current NotifyConfig object.
   */
  function getConfig(): NotifyConfig {
    return {
      isGranted: isGranted.value,
      enable: enable.value,
      whenCreate: whenCreate.value,
      whenFailed: whenFailed.value,
    }
  }

  /**
   * Set the notification configuration.
   * @param config - The NotifyConfig object to set.
   */
  function setConfig(config: NotifyConfig) {
    isGranted.value = config.isGranted
    enable.value = config.enable
    whenCreate.value = config.whenCreate
    whenFailed.value = config.whenFailed
  }

  /**
   * Initialize the notification configuration.
   * @param data - The initial NotifyConfig data.
   */
  async function init(data: NotifyConfig | undefined) {
    if (data === undefined) {
      await execute(Command.InitConfig, 'notify')
        .then((config: AppConfig) => {
          setConfig(config.notify)
        })
    }
    else {
      setConfig(data)
    }
  }

  /**
   * Enable or disable notifications.
   * @param value - The value to set for notification enablement.
   */
  async function enableNotify(value: boolean) {
    const config = getConfig()
    config.enable = value
    await execute(Command.UpdateConfig, 'notify', config)
      .then((config: AppConfig) => {
        setConfig(config.notify)
      })
  }

  /**
   * Update the create backup notification setting.
   * @param value - The value to set for create backup notification.
   */
  async function updatewhenCreateNotify(value: boolean) {
    const config = getConfig()
    config.whenCreate = value
    await execute(Command.UpdateConfig, 'notify', config)
      .then((config: AppConfig) => {
        setConfig(config.notify)
      })
  }

  /**
   * Update the failed backup notification setting.
   * @param value - The value to set for failed backup notification.
   */
  async function updatewhenFailedNotify(value: boolean) {
    const config = getConfig()
    config.whenFailed = value
    await execute(Command.UpdateConfig, 'notify', config)
      .then((config: AppConfig) => {
        setConfig(config.notify)
      })
  }

  /**
   * Update the notification granted status.
   * @param value - The value to set for notification granted status.
   */
  async function updateNotifyGranted(value: boolean) {
    const config = getConfig()
    config.isGranted = value
    await execute(Command.UpdateConfig, 'notify', config)
      .then((config: AppConfig) => {
        setConfig(config.notify)
      })
  }

  /**
   * Attempt to get permission for notifications.
   * @returns A boolean indicating if permission was granted.
   */
  async function tryGetPermission(): Promise<boolean> {
    const granted = await isPermissionGranted()
    if (!granted) {
      const permission = await requestPermission()
      if (permission !== 'granted')
        return false
    }

    updateNotifyGranted(true)
    return true
  }

  return {
    isGranted,
    enable,
    whenCreate,
    whenFailed,
    init,
    enableNotify,
    updatewhenCreateNotify,
    updatewhenFailedNotify,
    tryGetPermission,
  }
})

/**
 * Get the default notify configuration object.
 * @returns The default NotifyConfig object.
 */
export function defaultNotifyConfig(): NotifyConfig {
  return {
    isGranted: false,
    enable: false,
    whenCreate: false,
    whenFailed: false,
  }
}
