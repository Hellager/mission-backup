import { reactive } from 'vue'
import { defineStore } from 'pinia'
import type { HandlerStatus, Status } from './types'

export const useStatusStore = defineStore('status', () => {
  /**
   * The reactive object for handler status.
   */
  let handlerStatus = reactive<HandlerStatus>({
    log: false,
    app: false,
    cron: false,
    watcher: false,
    config: false,
    database: false,
  })

  /**
   * Gets the current handler status.
   * @returns The current handler status.
   */
  function getHandlerStatus(): HandlerStatus {
    return handlerStatus
  }

  /**
   * Sets the handler status.
   * @param status - The handler status to set.
   */
  function setHandlerStatus(status: HandlerStatus): void {
    handlerStatus = status
  }

  /**
   * Gets the overall status.
   * @returns The overall status.
   */
  function getStatus(): Status {
    return {
      handler: getHandlerStatus(),
    }
  }

  /**
   * Sets the overall status.
   * @param status - The overall status to set.
   */
  function setStatus(status: Status): void {
    handlerStatus = status.handler
  }

  return {
    handlerStatus,
    getHandlerStatus,
    setHandlerStatus,
    getStatus,
    setStatus,
  }
})
