/**
 * Represents the options for closing.
 */
export enum ThemeOption {
  Scoped,
  FollowSystem,
}

/**
 * Represents the options for closing.
 */
export enum CloseOption {
  Exit,
  Hide,
}

/**
 * Represents the system configuration.
 */
export interface SystemConfig {
  /**
   * The theme of the app.
   */
  theme: string

  /**
   * The theme of the system.
   */
  sysTheme: string

  /**
   * The option for theme.
   */
  themeOption: ThemeOption

  /**
   * Indicates whether the system should auto-start.
   */
  autoStart: boolean

  /**
   * The option for closing the system.
   */
  closeOption: CloseOption

  /**
   * The count for closing.
   */
  closeCnt: number

  /**
   * The limit for closing.
   */
  closeLimit: number

  /**
   * The language of the system.
   */
  language: string
}
