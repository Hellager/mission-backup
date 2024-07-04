<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { dayjs } from 'element-plus'
import type { TableInstance } from 'element-plus'
import { useI18n } from 'vue-i18n'
import prettyBytes from 'pretty-bytes'
import { Delete, Open } from '../../assets/icons'
import { useMissionStore } from '../../store'
import { Command, execute } from '../../utils/cmd'

interface BackupTableData {
  id: number
  backup_id: string
  date: string
  mission: string
  size: string
  path: string
}

interface BackupFilterData {
  text: string
  value: string
}

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 * Initializes the mission store and sets up necessary variables and functions for managing notifications.
 */
const store = useMissionStore()
const { missions, backups } = storeToRefs(store)

/**
 * Indicates whether the data is currently loading.
 */
const loading = ref<boolean>(true)

/**
 * Reference to the table instance.
 */
const tableRef = ref<TableInstance>()

/**
 * Reference to the data for the backup table.
 */
const tableData = ref<BackupTableData[]>([])

/**
 * Reference to the filter data for the backup table.
 */
const tableFilterData = ref<BackupFilterData[]>([])

/**
 * Asynchronously builds the data for the backup table by syncing records, filtering, and formatting the data.
 */
async function buildTableData() {
  await store.syncRecords('mission')
  await store.syncRecords('backup')

  const build_data: BackupTableData[] = []
  const filter_data: BackupFilterData[] = []
  for (const item of backups.value) {
    const created_mission = missions.value
      .find(m => m.missionId === item.missionId)
    if (created_mission !== undefined) {
      filter_data.push({
        text: created_mission.name,
        value: created_mission.name,
      })

      build_data.push({
        id: item.id,
        backup_id: item.backupId,
        mission: created_mission.name,
        size: prettyBytes(item.backupSize),
        date: dayjs.utc(item.createAt.substring(0, 23)).local().format('YYYY-MM-DD HH:mm'),
        path: item.savePath,
      })
    }
  }

  tableData.value = build_data
  tableFilterData.value = filter_data.filter(
    (value, index, self) =>
      self.findIndex(obj =>
        JSON.stringify(obj) === JSON.stringify(value))
        === index,
  )

  loading.value = false
}

/**
 * Handles the filtering of data in the backup table.
 *
 * @param value - The value to filter by
 * @param row - The row data to filter
 * @returns The filtered row data
 */
function filterHandler(value: string, row: BackupTableData) {
  return row.mission = value
}

/**
 * Opens the backup folder in the file explorer.
 *
 * @param path - The path of the backup folder to open
 */
async function onBackupOpenClicked(path: string) {
  await execute(Command.ShowInExplorer, path)
    .then((_res) => {
      // if (res.code != 200) {
      //     console.log("backup open error");
      // }
    })
}

/**
 * Deletes a backup entry and rebuilds the backup table data.
 *
 * @param uuid - The UUID of the backup entry to delete
 */
async function onBackupDeleteClicked(uuid: string) {
  await execute(Command.DeleteBackup, uuid)
  // console.log(res, uuid)

  await buildTableData()
}

/**
 * Executes the buildTableData function when the component is mounted.
 */
onMounted(async () => {
  await buildTableData()
})
</script>

<template>
  <div class="backup">
    <el-skeleton v-if="loading" class="backup__skeleton" :rows="5" animated />
    <div v-else class="backup__table">
      <el-table ref="tableRef" v-loading="loading" :data="tableData" stripe style="width: 100%">
        <el-table-column type="index" />
        <el-table-column prop="date" :label="t('backup.date')" :show-overflow-tooltip="true" />
        <el-table-column
          prop="mission" :label="t('backup.mission')"
          :filters="tableFilterData"
          :filter-method="filterHandler"
          column-key="mission"
        />
        <el-table-column prop="size" :label="t('backup.size')" />
        <el-table-column :label="t('backup.operation')">
          <template #default="scope">
            <el-button type="primary" text circle @click="onBackupOpenClicked(scope.row.path)">
              <el-icon><Open /></el-icon>
            </el-button>
            <el-button type="danger" text circle @click="onBackupDeleteClicked(scope.row.backup_id)">
              <el-icon><Delete /></el-icon>
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>
  </div>
</template>

<style scoped lang="less">
.container {
    width: 100%;
    height: 100%;
    background-color: #fff7e6;
}

.backup__skeleton {
    margin-left: 10px;
    margin-top: 10px;
}

.backup__table__create {
    margin-left: -10px;
}
</style>
