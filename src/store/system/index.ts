import { ref } from 'vue'
import { defineStore } from 'pinia'
import { useI18n } from 'vue-i18n'
import { useDark, useToggle } from '@vueuse/core'
import { disable, enable } from 'tauri-plugin-autostart-api'
import { appWindow } from '@tauri-apps/api/window'
import { Command, execute } from '../../utils/cmd'
import type { AppConfig } from '../types'
import { CloseOption } from './types'
import type { SystemConfig } from './types'

export const useSystemStore = defineStore('system', () => {
  const { locale } = useI18n({ useScope: 'global' })

  const theme = ref<string>('light')
  const sysTheme = ref<string>('light')
  const themeOption = ref<number>(0)
  const autoStart = ref<boolean>(false)
  const closeOption = ref<number>(0)
  const closeCnt = ref<number>(0)
  const closeLimit = ref<number>(50)
  const language = ref<string>('zh-CN')

  /**
   * Gets the current system configuration.
   * @returns The current system configuration.
   */
  function getConfig(): SystemConfig {
    return {
      theme: theme.value,
      sysTheme: sysTheme.value,
      themeOption: themeOption.value,
      autoStart: autoStart.value,
      closeOption: closeOption.value,
      closeCnt: closeCnt.value,
      closeLimit: closeLimit.value,
      language: language.value,
    }
  }

  /**
   * Sets the system configuration.
   * @param config - The system configuration to set.
   */
  function setConfig(config: SystemConfig): void {
    theme.value = config.theme
    sysTheme.value = config.sysTheme
    themeOption.value = config.themeOption
    autoStart.value = config.autoStart
    closeOption.value = config.closeOption
    closeCnt.value = config.closeCnt
    closeLimit.value = config.closeLimit
    language.value = config.language

    // udpate theme
    const isDark = useDark()
    if (config.themeOption)
      isDark.value = config.sysTheme !== 'light'
    else
      isDark.value = config.theme !== 'light'
    useToggle(isDark)

    // update lang
    locale.value = config.language
  }

  /**
   * Initializes the system with the provided data or fetches the configuration if data is undefined.
   * @param data - The optional system configuration data.
   */
  async function init(data: SystemConfig | undefined): Promise<void> {
    if (data === undefined) {
      await execute(Command.InitConfig, 'system')
        .then((config: AppConfig) => {
          setConfig(config.system)
        })
    }
    else {
      setConfig(data)
    }
  }

  /**
   * Shuts down the system.
   */
  async function shutdown(): Promise<void> {
    await execute(Command.ShutdownApp)
  }

  /**
   * Updates the theme of the app.
   * @param theme - The new theme for the app.
   */
  async function updateTheme(theme: string): Promise<void> {
    const config = getConfig()
    config.theme = theme
    await execute(Command.UpdateConfig, 'system', config)
      .then((config: AppConfig) => {
        setConfig(config.system)
        const isDark = useDark()
        isDark.value = config.system.theme !== 'light'
        useToggle(isDark)
      })
  }

  /**
   * Updates the theme of the system in config.
   * @param theme - The new theme for the system.
   */
  async function updateSysTheme(theme: string): Promise<void> {
    const config = getConfig()
    config.sysTheme = theme
    await execute(Command.UpdateConfig, 'system', config)
      .then((config: AppConfig) => {
        setConfig(config.system)

        if (config.system.themeOption) {
          const isDark = useDark()
          isDark.value = config.system.sysTheme !== 'light'
          useToggle(isDark)
        }
      })
  }

  /**
   * Updates the theme option of the app.
   * @param option - The new theme option for the app.
   */
  async function updateThemeOption(option: number): Promise<void> {
    const config = getConfig()
    config.themeOption = option
    await execute(Command.UpdateConfig, 'system', config)
      .then((config: AppConfig) => {
        setConfig(config.system)
      })
  }

  /**
   * Updates the language of the system.
   * @param lang - The new language for the system.
   */
  async function updateLanguage(lang: string): Promise<void> {
    const config = getConfig()
    config.language = lang
    await execute(Command.UpdateConfig, 'system', config)
      .then((config: AppConfig) => {
        setConfig(config.system)
        locale.value = config.system.language
      })
  }

  /**
   * Updates the auto-start setting of the system.
   * @param start - The new auto-start setting for the system.
   */
  async function updateAutoStart(start: boolean): Promise<void> {
    const config = getConfig()
    config.autoStart = start
    await execute(Command.UpdateConfig, 'system', config)
      .then(async (config: AppConfig) => {
        setConfig(config.system)
        if (config.system.autoStart)
          await enable()
        else
          await disable()
      })
  }

  /**
   * Updates the close option of the system.
   * @param option - The new close option for the system.
   */
  async function updateCloseOption(option: number): Promise<void> {
    const config = getConfig()
    config.closeOption = option
    await execute(Command.UpdateConfig, 'system', config)
      .then(async (config: AppConfig) => {
        setConfig(config.system)
      })
  }

  /**
   * Checks if a close confirmation is needed.
   * @returns A boolean indicating whether a close confirmation is needed.
   */
  function closeConfirm(): boolean {
    return closeCnt.value + 1 >= closeLimit.value
  }

  /**
   * Tries to close the system with the specified option and remember setting.
   * @param option - The close option to use.
   * @param remember - Whether to remember the close option.
   */
  async function tryClose(option: number | undefined, remember: boolean | undefined): Promise<void> {
    const config = getConfig()
    config.closeCnt++

    if (option === undefined || remember === undefined) {
      option = config.closeOption
      remember = false
    }

    if (remember) {
      config.closeOption = option
      config.closeCnt = 0
    }
    await execute(Command.UpdateConfig, 'system', config)
      .then((config: AppConfig) => {
        setConfig(config.system)
      })

    if (option === 0)
      await shutdown()
    else
      await appWindow.hide()
  }

  return {
    theme,
    themeOption,
    autoStart,
    closeOption,
    language,
    init,
    closeConfirm,
    tryClose,
    updateTheme,
    updateSysTheme,
    updateThemeOption,
    updateLanguage,
    updateAutoStart,
    updateCloseOption,
  }
})

/**
 * Generates the default system configuration.
 * @returns The default system configuration.
 */
export function defaultSystemConfig(): SystemConfig {
  return {
    theme: 'light',
    sysTheme: 'light',
    themeOption: 0,
    autoStart: false,
    closeOption: CloseOption.Exit,
    closeCnt: 0,
    closeLimit: 50,
    language: 'zh-CN',
  }
}
