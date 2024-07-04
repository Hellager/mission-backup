/**
 * Represents the status of different handlers.
 */
export interface HandlerStatus {
  /**
   * Indicates the status of the log handler.
   */
  log: boolean

  /**
   * Indicates the status of the app handler.
   */
  app: boolean

  /**
   * Indicates the status of the cron handler.
   */
  cron: boolean

  /**
   * Indicates the status of the watcher handler.
   */
  watcher: boolean

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
 * Represents the overall status.
 */
export interface Status {
  /**
   * The status of different handlers.
   */
  handler: HandlerStatus
  // /**
  //  * The status of the window.
  //  */
  // window: WindowStatus;
}
