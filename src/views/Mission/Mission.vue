<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { ElMessage } from 'element-plus'
import { useI18n } from 'vue-i18n'
import StatusDot from '../../components/StatusDot.vue'
import { useMissionStore } from '../../store'
import { CreateBox, Delete, Edit, Start, Stop } from '../../assets/icons'
import { DialogMode } from '../../types'
import type { Mission } from '../../store/mission/types'
import { MissionStatus } from '../../store/mission/types'
import { defaultMission } from '../../store/mission'
import HandleDialog from './components/HandleDialog.vue'

interface MissionTableData {
  id: number
  missionId: string
  name: string
  status: number
}

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 * Initializes the mission store and sets up necessary variables and functions for managing notifications.
 */
const store = useMissionStore()
const { missions, procedures } = storeToRefs(store)

/**
 * Indicates whether data is currently loading.
 */
const loading = ref<boolean>(true)

/**
 * Indicates whether the handle dialog is visible.
 */
const showHandleDialog = ref(false)

/**
 * Mode for the handle dialog (Create or Edit).
 */
const handleDialogMode = ref<number>(DialogMode.Create)

/**
 * Data for the handle dialog.
 */
const handleDialogData = ref<Mission>(defaultMission())

/**
 * Reference to the data for the mission table.
 */
const tableData = ref<MissionTableData[]>([])

/**
 * Asynchronously builds the data for the mission table by syncing records and formatting the data.
 */
async function buildTableData() {
  await store.syncRecords('mission')

  const build_data: MissionTableData[] = []
  for (const item of missions.value) {
    build_data.push({
      id: item.id,
      missionId: item.missionId,
      name: item.name,
      status: item.status,
    })
  }

  tableData.value = build_data
  loading.value = false
}

/**
 * Handles the action to create a new mission.
 */
async function onMissionCreateClicked() {
  if (procedures.value.length === 0) {
    ElMessage.error(`No available procedure`)
    return
  }
  handleDialogMode.value = DialogMode.Create
  showHandleDialog.value = true
}

/**
 * Sets the status of a mission and updates the table data.
 *
 * @param uuid - The UUID of the mission
 * @param status - The new status for the mission
 */
async function onSetMissionStatusClicked(uuid: string, status: number) {
  await store.setMissionStatus(uuid, status)
  await buildTableData()
}

/**
 * Handles the action to edit a mission.
 *
 * @param uuid - The UUID of the mission to edit
 */
async function onMissionEditClicked(uuid: string) {
  handleDialogMode.value = DialogMode.Edit
  showHandleDialog.value = true
  for (const item of missions.value) {
    if (item.missionId === uuid) {
      handleDialogData.value = item
      break
    }
  }
}

/**
 * Handles the action to delete a mission.
 *
 * @param uuid - The UUID of the mission to delete
 */
async function onMissionDeleteClicked(uuid: string) {
  /// query whether delete related backups
  for (const item of missions.value) {
    if (item.missionId === uuid) {
      // if (item.status != MissionStatus.Stopped) {
      //     ElMessage.error(`Mission ${item.name} is running`);
      //     return;
      // }

      const delete_res = await store.deleteMission(uuid)
      if (delete_res) {
        const res = await store.deleteRecord('mission', uuid, '')
        await store.deleteRecord('backup', '', uuid)
        if (res > 0)

          ElMessage.success(`Delete mission ${item.name} success`)
        else
          ElMessage.error(`Failed to delete mission ${item.name}`)
      }

      break
    }
  }

  await buildTableData()
}

/**
 * Handles the action for the handle dialog and updates the table data.
 */
async function handleDialogAction() {
  showHandleDialog.value = false
  store.syncRecords('mission')
  await buildTableData()
}

/**
 * Executes the build_tableData function when the component is mounted.
 */
onMounted(async () => {
  await buildTableData()
})
</script>

<template>
  <div class="mission">
    <el-skeleton v-if="loading" class="mission__skeleton" :rows="5" animated />
    <div v-else class="mission__table">
      <el-table :data="tableData" stripe style="width: 100%">
        <el-table-column type="index">
          <template #header>
            <el-button type="primary" text circle class="mission__table__create" @click="onMissionCreateClicked">
              <el-icon><CreateBox /></el-icon>
            </el-button>
          </template>
        </el-table-column>
        <el-table-column prop="name" :label="t('mission.name')" />
        <el-table-column :label="t('mission.status')">
          <template #default="scope">
            <StatusDot :status="scope.row.status" />
          </template>
        </el-table-column>
        <el-table-column :label="t('mission.operation')">
          <template #default="scope">
            <el-button v-if="scope.row.status === 0" type="success" text circle @click="onSetMissionStatusClicked(scope.row.missionId, MissionStatus.Running)">
              <el-icon><Start /></el-icon>
            </el-button>
            <el-button v-else type="warning" text circle @click="onSetMissionStatusClicked(scope.row.missionId, MissionStatus.Stopped)">
              <el-icon><Stop /></el-icon>
            </el-button>
            <el-button type="primary" text circle @click="onMissionEditClicked(scope.row.missionId)">
              <el-icon><Edit /></el-icon>
            </el-button>
            <el-button type="danger" text circle :disabled="scope.row.status !== MissionStatus.Stopped" @click="onMissionDeleteClicked(scope.row.missionId)">
              <el-icon><Delete /></el-icon>
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <HandleDialog :visiable="showHandleDialog" :mode="handleDialogMode" :data="handleDialogData" @hide="handleDialogAction" />
  </div>
</template>

<style scoped lang="less">
.container {
    width: 100%;
    height: 100%;
    background-color: #fff7e6;
}

.mission__skeleton {
    margin-left: 10px;
    margin-top: 10px;
}

.mission__table__create {
    margin-left: -10px;
}
</style>
