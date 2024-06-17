import { invoke } from '@tauri-apps/api/tauri'
import { Command } from './types'
import type { Response } from "./types"

/**
 * Executes a command with optional arguments and returns the result.
 * @param command - The command to execute.
 * @param arg0 - The first optional argument.
 * @param arg1 - The second optional argument.
 * @param arg2 - The third optional argument.
 * @returns The result of the command execution.
 */
async function execute(command: Number) {
    let result: any = false

    switch(command) {
        case Command.ShowMainWindow: {
            await invoke<Response<boolean>>(`show_main_window`)
                .then((res: Response<boolean>) => {
                    result = res.data
                })
                .catch((error: any) => {
                    console.log(error)
                })
        } break;

        default: {
            console.log("no match");
        } break;
    }

    return result;
}

export { execute, Command };
