import { compareKeys } from '../common'

/**
 * Represents various commands for execution.
 */
enum Command {
  InitApp,
  ShutdownApp,
  WebLog,
  ShowInExplorer,
  InitConfig,
  UpdateConfig,
  CreateRecord,
  UpdateRecord,
  QueryRecord,
  DeleteRecord,
  ClearRecord,
  DeleteBackup,
  SetMissionStatus,
  DeleteMission,
}

/**
 * Represents a response object with a code, data, and message.
 * @template T - The type of data in the response.
 */
interface Response<T> {
  code: number
  data: T
  msg: string
}

/**
 * Generates a default response object with a boolean data value.
 *
 * @returns A Response object with code 200, data set to true, and an empty message.
 */
function defaultResponse(): Response<boolean> {
  return {
    code: 200,
    data: true,
    msg: '',
  }
}

/**
 * Checks if the provided data object matches the keys of the default response object.
 *
 * @param data - The data object to compare with the default response object
 * @returns Returns true if the keys of the data object match the keys of the default response object, otherwise returns false.
 */
function isResponse(data: any): boolean {
  return compareKeys(data, defaultResponse())
}

export { Command, defaultResponse, isResponse }
export type { Response }
