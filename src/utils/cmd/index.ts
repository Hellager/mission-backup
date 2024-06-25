import { invoke } from '@tauri-apps/api/tauri'
import { Command } from './types'
import type { HandlerStatus, Response } from './types'

/**
 * Executes a command with optional arguments and returns the result.
 * @param command - The command to execute.
 * @returns The result of the command execution.
 */
async function execute(command: number) {
  let result: any = false

  switch (command) {
    case Command.InitApp:
      await invoke<Response<HandlerStatus>>(`init_app`)
        .then((res: Response<HandlerStatus>) => {
          result = res.data
        })
        .catch((error: any) => {
          console.error(error)
        })
      break

    case Command.ShutdownApp:
      await invoke<Response<boolean>>(`shutdown_app`)
        .then((res: Response<boolean>) => {
          result = res.data
        })
        .catch((error: any) => {
          console.error(error)
        })
      break

    default:
      console.error('no match')
      break
  }

  return result
}

export { execute, Command }
