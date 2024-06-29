import * as dayjs from 'dayjs'

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
  hasIgnore: boolean

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
   * The backup method for the procedure.
   */
  backupMethod: number

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
   * The path type of the mission.
   */
  pathType: number

  /**
   * The source path of the mission.
   */
  srcPath: string

  /**
   * The destination path of the mission.
   */
  dstPath: string

  /**
   * The next runtime of the mission.
   */
  nextRuntime: string

  /**
   * The last trigger timestamp of the mission.
   */
  lastTrigger: string

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
   * The creation timestamp of the backup.
   */
  createAt: string

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

/**
 * Get the default ignore object.
 * @returns The default Ignore object.
 */
function defaultIgnore(): Ignore {
  return {
    id: 0,
    ignoreId: '',
    procedureId: '',
    keyword: '',
    createAt: dayjs.utc().format().slice(0, -1),
    updateAt: dayjs.utc().format().slice(0, -1),
    isDeleted: 0,
    deleteAt: dayjs.utc().format().slice(0, -1),
  }
}

/**
 * Get the default procedure object.
 * @returns The default Procedure object.
 */
function defaultProcedure(): Procedure {
  return {
    id: 0,
    procedureId: '',
    name: '',
    hasIgnore: false,
    ignoreMethod: 1,
    isCompress: false,
    compressFormat: 1,
    trigger: 2,
    cronExpression: '',
    restrict: 0,
    restrictDays: 3,
    restrictSize: 1024,
    backupMethod: 1,
    createAt: dayjs.utc().format().slice(0, -1),
    updateAt: dayjs.utc().format().slice(0, -1),
    isDeleted: 0,
    deleteAt: dayjs.utc().format().slice(0, -1),
  }
}

/**
 * Get the default mission object.
 * @returns The default Mission object.
 */
function defaultMission(): Mission {
  return {
    id: 0,
    missionId: '',
    procedureId: '',
    name: '',
    status: 0,
    description: '',
    pathType: 1,
    srcPath: '',
    dstPath: '',
    nextRuntime: dayjs.utc().format().slice(0, -1),
    lastTrigger: dayjs.utc().format().slice(0, -1),
    createAt: dayjs.utc().format().slice(0, -1),
    updateAt: dayjs.utc().format().slice(0, -1),
    isDeleted: 0,
    deleteAt: dayjs.utc().format().slice(0, -1),
  }
}

/**
 * Get the default backup object.
 * @returns The default Backup object.
 */
function defaultBackup(): Backup {
  return {
    id: 0,
    backupId: '',
    missionId: '',
    savePath: '',
    backupSize: 0,
    createAt: dayjs.utc().format().slice(0, -1),
    isDeleted: 0,
    deleteAt: dayjs.utc().format().slice(0, -1),
  }
}

/**
 * Get the default record object.
 * @returns The default Record object.
 */
function defaultRecord(): Record {
  return {
    ignore: defaultIgnore(),
    procedure: defaultProcedure(),
    mission: defaultMission(),
    backup: defaultBackup(),
  }
}

export {
  IgnoreMethod,
  CompressFormat,
  MissionTrigger,
  BackupRestrict,
  PathType,
  MissionStatus,
  defaultBackup,
  defaultIgnore,
  defaultMission,
  defaultProcedure,
  defaultRecord,
}
export type {
  Ignore,
  Procedure,
  Mission,
  Backup,
  Record,
}
