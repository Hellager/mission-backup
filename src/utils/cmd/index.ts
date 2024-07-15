import { invoke } from '@tauri-apps/api/tauri'
import { set as _set } from 'lodash'
import type { HandlerStatus } from '../../store/status/types'
import { toCamelCase, toSnakeCase } from '../common/index'
import { defaultAppConfig } from '../../store'
import type { AppConfig } from '../../store/types'
import type { Backup, Mission, Record } from '../../store/mission/types'
import { Command } from './types'
import type { DBInfo, LogInfo, MigratedData, Response } from './types'

/**
 * Executes a command with optional arguments and returns the result.
 * @param command - The command to execute.
 * @param arg0 - The first optional argument.
 * @param arg1 - The second optional argument.
 * @param arg2 - The second optional argument.
 * @returns The result of the command execution.
 */
async function execute(command: number, arg0?: any, arg1?: any, arg2?: any) {
  let result: any = false

  switch (command) {
    case Command.InitApp:
      await invoke<Response<HandlerStatus>>(`init_app`)
        .then((res: Response<HandlerStatus>) => {
          result = res.data
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.ShutdownApp:
      await invoke<Response<boolean>>(`shutdown_app`)
        .then((res: Response<boolean>) => {
          result = res.data
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.WebLog:
      await invoke<Response<boolean>>('web_log', { level: arg0, msg: arg1 })
        .then((res: Response<boolean>) => {
          result = res.data
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.ShowInExplorer:
      await invoke<Response<boolean>>('show_item_in_explorer', { path: arg0 })
        .then((res: Response<boolean>) => {
          result = res.data
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.InitConfig: {
      const defaultConfig = defaultAppConfig()
      await invoke<Response<any>>('sync_config', { group: arg0, overwrite: false, config: toSnakeCase(defaultConfig) })
        .then((res: Response<any>) => {
          result = toCamelCase(res.data) as AppConfig
        })
        .catch((error: any) => {
          throw error
        })
    } break

    case Command.UpdateConfig: {
      const curConfig = defaultAppConfig()
      _set(curConfig, arg0, arg1)
      await invoke<Response<any>>('sync_config', { group: arg0, overwrite: true, config: toSnakeCase(curConfig) })
        .then((res: Response<any>) => {
          result = toCamelCase(res.data) as AppConfig
        })
        .catch((error: any) => {
          throw error
        })
    } break

    case Command.CreateRecord:
      await invoke<Response<any>>('create_record', { table: arg0, data: toSnakeCase(arg1) }) // res -> Response(Record)
        .then((res: Response<any>) => {
          result = toCamelCase(res.data) as Record
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.UpdateRecord:
      await invoke<Response<any>>('update_record', { table: arg0, data: toSnakeCase(arg1) }) // res -> Response(Record)
        .then((res: Response<any>) => {
          result = toCamelCase(res.data) as Record
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.QueryRecord:
      await invoke<Response<any>>('query_record', { table: arg0, uuid: arg1 }) // res -> Response(Record[])
        .then((res: Response<any>) => {
          result = res.data.map((item: any) => toCamelCase(item)) as Record[]
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.DeleteRecord:
      await invoke<Response<number>>('delete_record', { table: arg0, uuid0: arg1, uuid1: arg2 }) // res -> Response(number)
        .then((res: Response<number>) => {
          result = res.data
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.ClearRecord:
      await invoke<Response<number>>('clear_record', { table: arg0 }) // res -> Response(number)
        .then((res: Response<number>) => {
          result = res.data
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.DeleteBackup:
      await invoke<Response<boolean>>('delete_backup', { uuid: arg0 }) // res -> Response(boolean)
        .then((res: Response<boolean>) => {
          result = res.data
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.CreateMission:
      await invoke<Response<boolean>>('create_mission', { mission: toSnakeCase(arg0) }) // res -> Response(bool)
        .then((res: Response<boolean>) => {
          result = res.data
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.SetMissionStatus:
      await invoke<Response<any>>('set_mission_status', { uuid: arg0, stat: arg1 }) // res -> Response(Mission)
        .then((res: Response<any>) => {
          result = toCamelCase(res.data) as Mission
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.DeleteMission:
      await invoke<Response<boolean>>('delete_mission', { uuid: arg0 }) // res -> Response(bool)
        .then((res: Response<boolean>) => {
          result = res.data
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.QueryStatistic:
      await invoke<Response<any>>('query_statistic_record', { mid: arg0, start: arg1, stop: arg2 }) // res -> Response(Backup[])
        .then((res: Response<any>) => {
          result = toCamelCase(res.data) as Backup
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.QueryDBInfo:
      await invoke<Response<DBInfo>>('query_db_info')
        .then((res: Response<any>) => {
          result = toCamelCase(res.data) as DBInfo
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.CleanDatabase:
      await invoke<Response<DBInfo>>('clean_database')
        .then((res: Response<any>) => {
          result = toCamelCase(res.data) as DBInfo
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.QueryLogInfo:
      await invoke<Response<LogInfo>>('query_log_info')
        .then((res: Response<any>) => {
          result = toCamelCase(res.data) as LogInfo
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.CleanAppLog:
      await invoke<Response<number>>('clean_app_log')
        .then((res: Response<any>) => {
          result = res.data
        })
        .catch((error: any) => {
          throw error
        })
      break

    case Command.MigrateFromOld:
      await invoke<Response<MigratedData>>('migrate_from_old', { path: arg0 })
        .then((res: Response<any>) => {
          result = toCamelCase(res.data) as MigratedData
        })
        .catch((error: any) => {
          throw error
        })
      break

    default:
      throw new Error('invalid command')
      break
  }

  return result
}

export { execute, Command }
