import { compareKeys } from '../common'

/**
 * Represents various commands for execution.
 */
enum Command {
  InitApp,
  ShutdownApp,
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
 * Represents the status of different handlers.
 */
interface HandlerStatus {
  /**
   * Indicates the status of the log handler.
   */
  log: boolean

  /**
   * Indicates the status of the app handler.
   */
  app: boolean

  /**
   * Indicates the status of the config handler.
   */
  config: boolean

  /**
   * Indicates the status of the database handler.
   */
  database: boolean
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
export type { Response, HandlerStatus }
