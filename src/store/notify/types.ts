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
   * Indicates if `create backup` notifications are enabled.
   */
  createBackup: boolean
  /**
   * Indicates if `failed backup` notifications are enabled.
   */
  failedBackup: boolean
}
