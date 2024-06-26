import { invoke } from '@tauri-apps/api/tauri'
import type { HandlerStatus } from '../../store/status/types'
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

    default:
      throw new Error('invalid command')
      break
  }

  return result
}

export { execute, Command }
