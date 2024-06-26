import { invoke } from '@tauri-apps/api/tauri'
import { set as _set } from 'lodash'
import type { HandlerStatus } from '../../store/status/types'
import { toCamelCase, toSnakeCase } from '../common/index'
import { defaultAppConfig } from '../../store'
import type { AppConfig } from '../../store/types'
import { Command } from './types'
import type { Response } from './types'

/**
 * Executes a command with optional arguments and returns the result.
 * @param command - The command to execute.
 * @param arg0 - The first optional argument.
 * @param arg1 - The second optional argument.
 * @returns The result of the command execution.
 */
async function execute(command: number, arg0?: any, arg1?: any) {
  let result: any = false

  switch (command) {
    case Command.InitApp:
      await invoke<Response<HandlerStatus>>(`init_app`)
        .then((res: Response<HandlerStatus>) => {
          result = res.data
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.ShutdownApp:
      await invoke<Response<boolean>>(`shutdown_app`)
        .then((res: Response<boolean>) => {
          result = res.data
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.WebLog:
      await invoke<Response<boolean>>('web_log', { level: arg0, msg: arg1 })
        .then((res: Response<boolean>) => {
          result = res.data
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.InitConfig: {
      const defaultConfig = defaultAppConfig()
      await invoke<Response<any>>('sync_config', { group: arg0, overwrite: false, config: toSnakeCase(defaultConfig) })
        .then((res: Response<any>) => {
          result = toCamelCase(res.data) as AppConfig
        })
        .catch((error: any) => {
          throw error
        })
    } break

    case Command.UpdateConfig: {
      const curConfig = defaultAppConfig()
      _set(curConfig, arg0, arg1)
      await invoke<Response<any>>('sync_config', { group: arg0, overwrite: true, config: toSnakeCase(curConfig) })
        .then((res: Response<any>) => {
          result = toCamelCase(res.data) as AppConfig
        })
        .catch((error: any) => {
          throw error
        })
    } break

    default:
      throw new Error('invalid command')
      break
  }

  return result
}

export { execute, Command }
