/**
 * Represents the NotifyConfig interface.
 */
export interface NotifyConfig {
  /**
   * Indicates if the notification is granted.
   */
  isGranted: boolean
  /**
   * Indicates if the notification is enabled.
   */
  enable: boolean
  /**
   * Indicates if `when create` notifications are enabled.
   */
  whenCreate: boolean
  /**
   * Indicates if `when failed` notifications are enabled.
   */
  whenFailed: boolean
}
