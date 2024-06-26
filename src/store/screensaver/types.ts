/**
 * Represents the configuration for the screensaver.
 */
export interface ScreensaverConfig {
  /**
   * Indicates if the screensaver is enabled.
   */
  enable: boolean
  /**
   * The password for the screensaver.
   */
  password: string
  /**
   * Indicates if the screensaver is locked.
   */
  isLocked: boolean
}
