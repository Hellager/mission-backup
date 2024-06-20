import { invoke } from '@tauri-apps/api/tauri'
import { Command } from './types'
import type { Response } from './types'

/**
 * Executes a command with optional arguments and returns the result.
 * @param command - The command to execute.
 * @returns The result of the command execution.
 */
async function execute(command: number) {
  let result: any = false

  switch (command) {
    case Command.ShowMainWindow:
      await invoke<Response<boolean>>(`show_main_window`)
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
