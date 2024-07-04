<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { ElMessage } from 'element-plus'
import { useI18n } from 'vue-i18n'
import { useMissionStore } from '../../store'
import { CreateBox, Delete, Edit } from '../../assets/icons'
import { DialogMode } from '../../types.ts'
import { defaultProcedure } from '../../store/mission'
import type { Procedure } from '../../store/mission/types'
import HandleDialog from './components/HandleDialog.vue'

interface ProcedureTableData {
  id: number
  procedureId: string
  name: string
  applied: string[]
}

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 * Initializes the mission store and sets up necessary variables and functions for managing notifications.
 */
const store = useMissionStore()
const { procedures, missions } = storeToRefs(store)

/**
 * Indicates whether data is currently loading.
 */
const loading = ref<boolean>(true)

/**
 * Represents the loading state.
 */
const showHandleDialog = ref(false)

/**
 * Mode for the handle dialog (Create or Edit).
 */
const handleDialogMode = ref<number>(DialogMode.Create)

/**
 * Data for the handle dialog.
 */
const handleDialogData = ref<Procedure>(defaultProcedure())

/**
 * Reference to the data for the mission table.
 */
const tableData = ref<ProcedureTableData[]>([])

/**
 * Asynchronously builds the table data by syncing records and populating the table with procedures and applied missions.
 */
async function buildTableData() {
  await store.syncRecords('procedure')
  await store.syncRecords('mission')

  const build_data: ProcedureTableData[] = []
  for (const item of procedures.value) {
    const applied_missions = missions.value
      .filter(m => m.procedureId === item.procedureId)
      .map(m => m.name)
    build_data.push({
      id: item.id,
      procedureId: item.procedureId,
      name: item.name,
      applied: applied_missions,
    })
  }

  tableData.value = build_data
  loading.value = false
}

/**
 * Handles the action to create a new mission.
 */
async function onProcedureCreateClicked() {
  handleDialogMode.value = DialogMode.Create
  showHandleDialog.value = true
}

/**
 * Handles the action to edit a procedure.
 *
 * @param uuid - The UUID of the procedure to edit
 */
async function onProcedureEditClicked(uuid: string) {
  handleDialogMode.value = DialogMode.Edit
  showHandleDialog.value = true
  for (const item of procedures.value) {
    if (item.procedureId === uuid) {
      handleDialogData.value = item
      break
    }
  }
}

/**
 * Handles the action to delete a procedure.
 *
 * @param uuid - The UUID of the procedure to delete
 */
async function onProcedureDeleteClicked(uuid: string) {
  for (const item of procedures.value) {
    if (item.procedureId === uuid) {
      const applied_missions = missions.value
        .filter(m => m.procedureId === item.procedureId)
        .map(m => m.name)
      if (applied_missions.length === 0) {
        await store.deleteRecord('ignore', '', uuid)
        // console.log(`delete ${del_cnt} ignores`)
        const res = await store.deleteRecord('procedure', uuid, '')
        if (res === 1)
          ElMessage.success(`Delete procedure ${item.name} success`)
        else
          ElMessage.error(`Failed to delete procedure ${item.name}`)
      }
      else {
        ElMessage.error(`${applied_missions.length} missions working with this procedure`)
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
  <div class="procedure">
    <el-skeleton v-if="loading" class="procedure__skeleton" :rows="5" animated />
    <div v-else class="procedure__table">
      <el-table :data="tableData" stripe style="width: 100%">
        <el-table-column type="index">
          <template #header>
            <el-button type="primary" text circle class="procedure__table__create" @click="onProcedureCreateClicked">
              <el-icon><CreateBox /></el-icon>
            </el-button>
          </template>
        </el-table-column>
        <el-table-column prop="name" :label="t('procedure.name')" />
        <el-table-column :label="t('procedure.applied')">
          <template #default="scope">
            <el-popover placement="right" :disabled="scope.row.applied.length === 0">
              <template #reference>
                <span>{{ scope.row.applied.length }}</span>
              </template>
              <li v-for="item in scope.row" :key="item.id">
                {{ item.applied }}
              </li>
            </el-popover>
          </template>
        </el-table-column>
        <el-table-column :label="t('procedure.operation')">
          <template #default="scope">
            <el-button type="primary" text circle @click="onProcedureEditClicked(scope.row.procedureId)">
              <el-icon><Edit /></el-icon>
            </el-button>
            <el-button type="danger" text circle :disabled="scope.row.applied.length !== 0" @click="onProcedureDeleteClicked(scope.row.procedureId)">
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
.procedure {
    width: 100%;
    height: 100%;

    :deep(.el-overlay-dialog::-webkit-scrollbar) {
        display: none;
    }

    :deep(.el-overlay-dialog) {
        -ms-overflow-style: none;  /* IE and Edge */
        scrollbar-width: none;  /* Firefox */
    }
}

.procedure__skeleton {
    margin-left: 10px;
    margin-top: 10px;
}

.procedure__table__create {
    margin-left: -10px;
}
</style>
