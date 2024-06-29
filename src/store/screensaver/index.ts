import { ref } from 'vue'
import { defineStore } from 'pinia'
import type { AppConfig } from '../types'
import { Command, execute } from '../../utils/cmd'
import type { ScreensaverConfig } from './types'

export const useScreensaverStore = defineStore('screensaver', () => {
  const enable = ref<boolean>(false)
  const password = ref<string>('')
  const isLocked = ref<boolean>(false)

  /**
   * Gets the current screensaver configuration.
   * @returns The current screensaver configuration.
   */
  function getConfig(): ScreensaverConfig {
    return {
      enable: enable.value,
      password: password.value,
      isLocked: isLocked.value,
    }
  }

  /**
   * Sets the screensaver configuration.
   * @param config - The screensaver configuration to set.
   */
  function setConfig(config: ScreensaverConfig): void {
    enable.value = config.enable
    password.value = config.password
    isLocked.value = config.isLocked
  }

  /**
   * Initializes the screensaver with the provided data or fetches the configuration if data is undefined.
   * @param data - The optional screensaver configuration data.
   */
  async function init(data: ScreensaverConfig | undefined): Promise<void> {
    if (data === undefined) {
      await execute(Command.InitConfig, 'screensaver')
        .then((config: AppConfig) => {
          setConfig(config.screensaver)
        })
    }
    else {
      setConfig(data)
    }
  }

  /**
   * Enables or disables the screensaver.
   * @param value - The value to set for enabling the screensaver.
   */
  async function enableScreensaver(value: boolean): Promise<void> {
    const config = getConfig()
    config.enable = value

    await execute(Command.UpdateConfig, 'screensaver', config)
      .then((config: AppConfig) => {
        setConfig(config.screensaver)
      })
  }

  /**
   * Updates the password for the screensaver.
   * @param value - The new password for the screensaver.
   */
  async function updatePassword(value: string): Promise<void> {
    const config = getConfig()
    config.password = value

    await execute(Command.UpdateConfig, 'screensaver', config)
      .then((config: AppConfig) => {
        setConfig(config.screensaver)
      })
  }

  /**
   * Updates the lock status of the screensaver.
   * @param value - The new lock status for the screensaver.
   */
  async function updateLockStatus(value: boolean): Promise<void> {
    isLocked.value = value
  }

  /**
   * Tries to unlock the screensaver with the provided password.
   * @param value - The password to try for unlocking the screensaver.
   * @returns A boolean indicating whether the unlock attempt was successful.
   */
  async function tryUnlock(value: string): Promise<boolean> {
    if (password.value !== value) {
      await execute(Command.WebLog, 'warn', `Unlock screensaver failed, invalid password: ${value}`)
      return false
    }
    else {
      await execute(Command.WebLog, 'info', `Unlock screensaver success`)
    }

    return true
  }

  return {
    enable,
    password,
    isLocked,
    init,
    enableScreensaver,
    updatePassword,
    updateLockStatus,
    tryUnlock,
  }
})

/**
 * Get the default screensaver configuration.
 * @returns The default ScreensaverConfig object.
 */
export function defaultScreensaverConfig(): ScreensaverConfig {
  return {
    enable: false,
    password: '',
    isLocked: false,
  }
}
