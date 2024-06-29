<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { ElMessage } from 'element-plus'
import { useI18n } from 'vue-i18n'
import * as dayjs from 'dayjs'
import { Create, Delete, Search } from '../../../assets/icons'
import { useMissionStore } from '../../../store'
import type { Ignore } from '../../../store/mission/types'
import { DialogMode } from '../../../types'

interface IgnoreTableData {
  id: number
  ignoreId: string
  keyword: string
}

/**
 * Props definition for the dialog component.
 */
const props = defineProps({
  visiable: { type: Boolean, required: true },
  mode: { type: Number, required: true },
  uuid: { type: String, required: true },
})

/**
 * Emits events for the dialog.
 */
const emit = defineEmits<{
  (event: 'hide', res: boolean): void
  (event: 'createIgnores', data: Ignore[]): void
}>()

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 * Initializes the mission store and sets up necessary variables and functions for managing notifications.
 */
const store = useMissionStore()
const { ignores } = storeToRefs(store)

/**
 * Reference to the dialog title.
 */
const dialogTitle = ref<string>('')

/**
 * Indicates whether data is currently loading.
 */
const loading = ref<boolean>(true)

/**
 * Reference to the data for the ignore table.
 */
const tableData = ref<IgnoreTableData[]>([])

/**
 * Reference to the saved data for the ignore table.
 */
const savedData = ref<IgnoreTableData[]>([])

/**
 * Reference to the input keyword for filtering.
 */
const inputKeyword = ref<string>('')

/**
 * Computed property for managing dialog visibility and behavior.
 */
const visiable = computed({
  get: () => {
    return props.visiable
  },
  set: (value: boolean) => emit('hide', value),
})

watch(visiable, async (_new, _old) => {
  if (props.mode === DialogMode.Create) {
    dialogTitle.value = t('ignore.createTitle')
    tableData.value = []
  }
  else if (props.mode === DialogMode.Edit) {
    dialogTitle.value = t('ignore.editTitle')
    setTimeout(async () => {
      await buildTableData()
    }, 0)
  }
})

/**
 * Asynchronously builds the data for the ignore table by syncing records and filtering the data.
 */
async function buildTableData() {
  await store.syncRecords('ignore')

  const build_data: IgnoreTableData[] = []
  for (const item of ignores.value) {
    if (item.procedureId === props.uuid) {
      build_data.push({
        id: item.id,
        ignoreId: item.ignoreId,
        keyword: item.keyword,
      })
    }
  }

  tableData.value = build_data

  loading.value = false
}

/**
 * Handles the action to create a new ignore entry.
 */
async function onIgnoreCreateClicked() {
  if (inputKeyword.value !== '') {
    for (const item of tableData.value) {
      if (inputKeyword.value.includes(item.keyword)) {
        ElMessage.error(`Same keyword`)
        return
      }
    }
    tableData.value = [{
      id: 0,
      ignoreId: '',
      keyword: inputKeyword.value.toString(),
    }, ...tableData.value]
    savedData.value = tableData.value

    inputKeyword.value = ''
  }
}

/**
 * Handles the search action to filter the ignore table data by keyword.
 */
async function onIgnoreSearch() {
  tableData.value = savedData.value
  if (inputKeyword.value !== '') {
    savedData.value = tableData.value

    const search_keyword = inputKeyword.value.toString()
    tableData.value = tableData.value.filter(item => item.keyword.includes(search_keyword))
  }
}

/**
 * Handles the action to delete an ignore entry.
 *
 * @param keyword - The keyword of the ignore entry to delete
 */
async function onIgnoreDeleteClicked(keyword: string) {
  tableData.value = tableData.value.filter(item => item.keyword !== keyword)
}

/**
 * Handles the cancel action for the dialog.
 */
async function onDialogCancelClicked() {
  inputKeyword.value = ''
  emit('hide', true)
}

/**
 * Handles the confirm action for the dialog, creating new ignore entries.
 */
async function onDialogConfirmClicked() {
  inputKeyword.value = ''

  const new_ignores: Ignore[] = []
  for (const item of tableData.value) {
    new_ignores.push({
      id: item.id,
      ignoreId: '',
      procedureId: '',
      keyword: item.keyword,
      createAt: dayjs.utc().format().slice(0, -1),
      updateAt: dayjs.utc().format().slice(0, -1),
      isDeleted: 0,
      deleteAt: dayjs.utc().format().slice(0, -1),
    })
  }

  emit('createIgnores', new_ignores)
  emit('hide', true)
}

/**
 * Executes the build_tableData function when the component is mounted.
 */
onMounted(async () => {
  await buildTableData()
})
</script>

<template>
  <el-dialog
    v-model="visiable"
    :title="dialogTitle"
    class="dialog"
  >
    <div class="ignore">
      <div class="ignore__action">
        <el-form class="ignore__action__form">
          <el-form-item :label="t('ignore.keyword')">
            <div class="ignore__action__form__content">
              <div class="ignore__action__form__content__keyword">
                <el-input v-model="inputKeyword">
                  <template #append>
                    <el-button :icon="Search" @click="onIgnoreSearch" />
                  </template>
                </el-input>
              </div>

              <div class="ignore__action__form__content__action">
                <el-button :icon="Create" type="primary" @click="onIgnoreCreateClicked" />
              </div>
            </div>
          </el-form-item>
        </el-form>
      </div>
      <div class="ignore__table">
        <div v-if="loading" class="ignore__table__skeleton">
          <el-skeleton :rows="5" animated />
        </div>
        <el-table v-else :data="tableData" stripe style="width: 100%">
          <el-table-column type="index" />
          <el-table-column prop="keyword" :label="t('ignore.keyword')" />
          <el-table-column :label="t('ignore.operation')">
            <template #default="scope">
              <el-button type="danger" text circle @click="onIgnoreDeleteClicked(scope.row.keyword)">
                <el-icon><Delete /></el-icon>
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </div>
    </div>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="onDialogCancelClicked">
          {{ t("common.cancel") }}
        </el-button>
        <el-button type="primary" @click="onDialogConfirmClicked">
          {{ t("common.confirm") }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<style scoped lang="less">
.ignore {
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

.ignore__table__skeleton {
    margin-left: 10px;
    margin-top: 10px;
}

.ignore__table__create {
    margin-left: -10px;
}

.ignore__action__form__content {
    display: flex;
    flex-direction: row;
    width: -webkit-fill-available;
}

.ignore__action__form__content__keyword {
    width: 100%;
    margin-right: 3px;
}
</style>
