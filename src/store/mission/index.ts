import { ref } from 'vue'
import { defineStore } from 'pinia'
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
   * @param uuid_0 - The first UUID.
   * @param uuid_1 - The second UUID.
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
  }
})
