/**
 * Enum for IgnoreMethod.
 */
enum IgnoreMethod {
  None,
  Custom,
  Gitignore,
}

/**
 * Enum for CompressFormat.
 */
enum CompressFormat {
  None,
  Zip,
  Targz,
  Tarbz2,
  Tarxz,
  Sevenz,
}

/**
 * Enum for MissionTrigger.
 */
enum MissionTrigger {
  None,
  Cron,
  Monitor,
}

/**
 * Enum for BackupRestrict.
 */
enum BackupRestrict {
  None,
  Days,
  Size,
  DaysAndSize,
}

/**
 * Enum for PathType.
 */
enum PathType {
  None,
  File,
  Directory,
}

/**
 * Enum for MissionStatus.
 */
enum MissionStatus {
  Stopped,
  Running,
  Backuping,
}

/**
 * Represents the Ignore interface.
 */
interface Ignore {
  /**
   * The ID of the ignore.
   */
  id: number

  /**
   * The ignore ID.
   */
  ignoreId: string

  /**
   * The procedure ID associated with the ignore.
   */
  procedureId: string

  /**
   * The keyword for the ignore.
   */
  keyword: string

  /**
   * Reserved value field
   */
  reserved0: string

  /**
   * Reserved value field
   */
  reserved1: string

  /**
   * Reserved value field
   */
  reserved2: string

  /**
   * The creation timestamp of the ignore.
   */
  createAt: string

  /**
   * The last update timestamp of the ignore.
   */
  updateAt: string

  /**
   * Indicates if the ignore is deleted.
   */
  isDeleted: number

  /**
   * The deletion timestamp of the ignore.
   */
  deleteAt: string
}

/**
 * Represents the Procedure interface.
 */
interface Procedure {
  /**
   * The ID of the procedure.
   */
  id: number

  /**
   * The procedure ID.
   */
  procedureId: string

  /**
   * The name of the procedure.
   */
  name: string

  /**
   * Indicates if the procedure has ignore.
   */
  hasIgnores: boolean

  /**
   * The ignore method for the procedure.
   */
  ignoreMethod: number

  /**
   * Indicates if the procedure is compressed.
   */
  isCompress: boolean

  /**
   * The compression format for the procedure.
   */
  compressFormat: number

  /**
   * The trigger for the procedure.
   */
  trigger: number

  /**
   * The cron expression for scheduling the procedure.
   */
  cronExpression: string

  /**
   * The restriction type for the procedure.
   */
  restrict: number

  /**
   * The number of days for restriction.
   */
  restrictDays: number

  /**
   * The size restriction for the procedure.
   */
  restrictSize: number

  /**
   * Reserved value field
   */
  reserved0: string

  /**
   * Reserved value field
   */
  reserved1: string

  /**
   * Reserved value field
   */
  reserved2: string

  /**
   * The creation timestamp of the procedure.
   */
  createAt: string

  /**
   * The last update timestamp of the procedure.
   */
  updateAt: string

  /**
   * Indicates if the procedure is deleted.
   */
  isDeleted: number

  /**
   * The deletion timestamp of the procedure.
   */
  deleteAt: string
}

/**
 * Represents the Mission interface.
 */
interface Mission {
  /**
   * The ID of the mission.
   */
  id: number

  /**
   * The mission ID.
   */
  missionId: string

  /**
   * The procedure ID associated with the mission.
   */
  procedureId: string

  /**
   * The name of the mission.
   */
  name: string

  /**
   * The status of the mission.
   */
  status: number

  /**
   * The description of the mission.
   */
  description: string

  /**
   * The source path of the mission.
   */
  srcPath: string

  /**
   * The destination path of the mission.
   */
  dstPath: string

  /**
   * The path type of the mission.
   */
  pathType: number

  /**
   * The next runtime of the mission.
   */
  nextRuntime: string

  /**
   * The last trigger timestamp of the mission.
   */
  lastTrigger: string

  /**
   * Reserved value field
   */
  reserved0: string

  /**
   * Reserved value field
   */
  reserved1: string

  /**
   * Reserved value field
   */
  reserved2: string

  /**
   * The creation timestamp of the mission.
   */
  createAt: string

  /**
   * The last update timestamp of the mission.
   */
  updateAt: string

  /**
   * Indicates if the mission is deleted.
   */
  isDeleted: number

  /**
   * The deletion timestamp of the mission.
   */
  deleteAt: string
}

/**
 * Represents the Backup interface.
 */
interface Backup {
  /**
   * The ID of the backup.
   */
  id: number

  /**
   * The backup ID.
   */
  backupId: string

  /**
   * The mission ID associated with the backup.
   */
  missionId: string

  /**
   * The save path of the backup.
   */
  savePath: string

  /**
   * The size of the backup.
   */
  backupSize: number

  /**
   * Reserved value field
   */
  reserved0: string

  /**
   * Reserved value field
   */
  reserved1: string

  /**
   * Reserved value field
   */
  reserved2: string

  /**
   * The creation timestamp of the backup.
   */
  createAt: string

  /**
   * The last update timestamp of the mission.
   */
  updateAt: string

  /**
   * Indicates if the backup is deleted.
   */
  isDeleted: number

  /**
   * The deletion timestamp of the backup.
   */
  deleteAt: string
}

/**
 * Represents the Record interface.
 */
interface Record {
  /**
   * The backup associated with the record.
   */
  backup: Backup

  /**
   * The ignore associated with the record.
   */
  ignore: Ignore

  /**
   * The mission associated with the record.
   */
  mission: Mission

  /**
   * The procedure associated with the record.
   */
  procedure: Procedure
}
/**
 * Represents the Record interface.
 */
interface Record {
  /**
   * The backup associated with the record.
   */
  backup: Backup
  /**
   * The ignore associated with the record.
   */
  ignore: Ignore
  /**
   * The mission associated with the record.
   */
  mission: Mission
  /**
   * The procedure associated with the record.
   */
  procedure: Procedure
}

export {
  IgnoreMethod,
  CompressFormat,
  MissionTrigger,
  BackupRestrict,
  PathType,
  MissionStatus,
}
export type {
  Ignore,
  Procedure,
  Mission,
  Backup,
  Record,
}
