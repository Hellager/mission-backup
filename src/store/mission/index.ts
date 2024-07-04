import { ref } from 'vue'
import { defineStore } from 'pinia'
import dayjs from 'dayjs'
import { Command, execute } from '../../utils/cmd'
import type { Backup, Ignore, Mission, Procedure, Record } from './types'

export const useMissionStore = defineStore('mission', () => {
  const ignores = ref<Ignore[]>([])
  const procedures = ref<Procedure[]>([])
  const missions = ref<Mission[]>([])
  const backups = ref<Backup[]>([])

  /**
   * Get the list of ignores.
   * @returns The list of ignores.
   */
  function getIgnores(): Ignore[] {
    return ignores.value
  }

  /**
   * Set the list of ignores.
   * @param data - The list of ignores to set.
   */
  function setIgnores(data: Ignore[]) {
    ignores.value = data
  }

  /**
   * Create a record.
   * @param table - The table name.
   * @param data - The record data.
   * @returns The created record or false.
   */
  async function createRecord(table: string, data: Record): Promise<Record> {
    const res = await execute(Command.CreateRecord, table, data)
    return res
  }

  /**
   * Update a record in the specified table.
   * @param table - The table name.
   * @param data - The record data to update.
   * @returns The updated record or false.
   */
  async function updateRecord(table: string, data: Record): Promise<Record> {
    const res = await execute(Command.UpdateRecord, table, data)
    return res
  }

  /**
   * Delete a record from the specified table.
   * @param table - The table name.
   * @param uuid_0 - The first UUID for table record.
   * @param uuid_1 - The second UUID optional for table `ignore` and `backup`.
   *                 once set, `procedure` related ignores or `mission` related backups will be all deleted.
   * @returns True if the record is deleted successfully, otherwise the error data.
   */
  async function deleteRecord(table: string, uuid_0: string | undefined, uuid_1: string | undefined): Promise<number> {
    const res = await execute(Command.DeleteRecord, table, uuid_0, uuid_1)
    return res
  }

  /**
   * Synchronize records for the specified table.
   * @param table - The table name.
   * @returns An array of records.
   */
  async function syncRecords(table: string): Promise<Record[]> {
    const records: Record[] = await execute(Command.QueryRecord, table)
    switch (table) {
      case 'ignore':
        ignores.value = records.map(r => r.ignore)
        break

      case 'procedure':
        procedures.value = records.map(r => r.procedure)
        break

      case 'mission':
        missions.value = records.map(r => r.mission)
        break

      case 'backup':
        backups.value = records.map(r => r.backup)
        break

      default: {
        throw new Error(`No match table for syncRecords`)
      }
    }

    return records
  }

  /**
   * Set the status of a mission by UUID.
   * @param uuid - The UUID of the mission.
   * @param status - The status to set.
   * @returns True if the status is set successfully, otherwise false.
   */
  async function setMissionStatus(uuid: string, status: number): Promise<boolean> {
    const res = await execute(Command.SetMissionStatus, uuid, status)
    return res
  }

  /**
   * Create a new mission.
   * @param mission - The mission data to create.
   * @returns True if the mission is created successfully, otherwise false.
   */
  async function createMission(mission: Mission): Promise<boolean> {
    const res = await execute(Command.CreateMission, mission)
    return res
  }

  /**
   * Delete a mission by UUID.
   * @param uuid - The UUID of the mission to delete.
   * @returns True if the mission is deleted successfully, otherwise false.
   */
  async function deleteMission(uuid: string): Promise<boolean> {
    const res = await execute(Command.DeleteMission, uuid)
    return res
  }

  return {
    ignores,
    procedures,
    missions,
    backups,
    getIgnores,
    setIgnores,
    createRecord,
    updateRecord,
    deleteRecord,
    syncRecords,
    setMissionStatus,
    createMission,
    deleteMission,
  }
})

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
    reserved0: '',
    reserved1: '',
    reserved2: '',
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
    hasIgnores: false,
    ignoreMethod: 1,
    isCompress: false,
    compressFormat: 1,
    trigger: 2,
    cronExpression: '',
    restrict: 0,
    restrictDays: 3,
    restrictSize: 1024,
    reserved0: '',
    reserved1: '',
    reserved2: '',
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
    reserved0: '',
    reserved1: '',
    reserved2: '',
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
    reserved0: '',
    reserved1: '',
    reserved2: '',
    createAt: dayjs.utc().format().slice(0, -1),
    updateAt: dayjs.utc().format().slice(0, -1),
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
  defaultBackup,
  defaultIgnore,
  defaultMission,
  defaultProcedure,
  defaultRecord,
}
