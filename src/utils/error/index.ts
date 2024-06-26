import { ElMessage } from 'element-plus'
import { isResponse } from '../cmd/types'
import type { Response } from '../cmd/types'
import { Command, execute } from '../cmd'

/**
 * Handles errors that occur during execution.
 * @param error - The error that occurred.
 * @param _vm - The first optional argument.
 * @param _info - Additional information.
 */
export async function errorHandler(error: any, _vm: any, _info: any) {
  if (isResponse(error)) {
    const response = error as Response<boolean>
    ElMessage.error(response.msg)
  }
  else {
    await execute(Command.WebLog, 'error', `Web: ${error}`)
  }
}
