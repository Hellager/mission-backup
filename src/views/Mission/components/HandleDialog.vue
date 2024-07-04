<script setup lang="ts">
import type { PropType } from 'vue'
import { computed, reactive, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { ElMessage } from 'element-plus'
import { isBoolean } from 'lodash'
import { open } from '@tauri-apps/api/dialog'
import { homeDir } from '@tauri-apps/api/path'
import { storeToRefs } from 'pinia'
import type { Mission } from '../../../store/mission/types'
import { DialogMode } from '../../../types'
import { MissionStatus, PathType } from '../../../store/mission/types'
import { defaultMission, defaultRecord } from '../../../store/mission'
import { useMissionStore } from '../../../store'

interface SelectOptions {
  value: number | string
  label: string
}

/**
 * Props definition for the dialog component.
 */
const props = defineProps({
  visiable: { type: Boolean, required: true },
  mode: { type: Number, required: true },
  data: { type: Object as PropType<Mission>, required: true },
})

/**
 * Emits events for hiding the dialog.
 */
const emit = defineEmits<{
  (event: 'hide', res: boolean): void
}>()

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 * Initializes the mission store and sets up necessary variables and functions for managing notifications.
 */
const store = useMissionStore()
const { procedures } = storeToRefs(store)

/**
 * Reference to the procedure options.
 */
const procedureOptions = ref<SelectOptions[]>([])

/**
 * Options for selecting path type.
 */
const pathTypeOptions: SelectOptions[] = [
  {
    value: PathType.File,
    label: t('mission.file'),
  },
  {
    value: PathType.Directory,
    label: t('mission.directory'),
  },
]

/**
 * Reference to the dialog title.
 */
const dialogTitle = ref<string>('')

/**
 * Reference to the form data.
 */
const formData = ref<Mission>(defaultMission())

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
    dialogTitle.value = t('mission.newTitle')
    formData.value = defaultMission()
  }
  else if (props.mode === DialogMode.Edit) {
    dialogTitle.value = t('mission.editTitle')
    formData.value = props.data
  }

  setTimeout(async () => {
    await buildProcedureOptions()
  }, 0)
})

/**
 * Reactive object for form validation rules.
 */
const formRules = reactive({
  name: [{ required: true, message: '', trigger: 'blur' }],
  // description: [{ message: '', trigger: 'blur' }],
  pathType: [{ required: true, message: '', trigger: 'blur' }],
  srcPath: [{ required: true, message: '', trigger: 'change' }],
  dstPath: [{ required: true, message: '', trigger: 'change' }],
  procedureId: [{ required: true, message: '', trigger: 'change' }],
})

/**
 * Opens a file dialog to select a path.
 *
 * @param target - The target path to update ('from' or 'to')
 */
async function selectPath(target: string) {
  const selected = await open({
    directory: (formData.value.pathType !== PathType.File || target === 'to'),
    multiple: false,
    defaultPath: await homeDir(),
  })

  if (selected !== null) {
    if (target === 'from')
      formData.value.srcPath = Array.isArray(selected) ? selected[0] : selected
    else if (target === 'to')
      formData.value.dstPath = Array.isArray(selected) ? selected[0] : selected
  }
}

/**
 * Builds the procedure options for the select dropdown.
 */
async function buildProcedureOptions() {
  await store.syncRecords('procedure')

  const option_data = [] as SelectOptions[]
  for (const item of procedures.value) {
    option_data.push({
      value: item.procedureId,
      label: item.name,
    })
  }

  procedureOptions.value = option_data
}

/**
 * Handles the cancel action for the dialog.
 */
async function onDialogCancelClicked() {
  formData.value = defaultMission()
  emit('hide', true)
}

/**
 * Handles the confirm action for the dialog, creating or updating a mission record.
 */
async function onDialogConfirmClicked() {
  if (formData.value.name === '') {
    ElMessage.error('Empty name')
    return
  }

  if (props.mode === DialogMode.Create) {
    const record = defaultRecord()
    formData.value.status = MissionStatus.Running
    record.mission = formData.value
    const res = await store.createRecord('mission', record)
    if (!isBoolean(res)) {
      const create_res = await store.createMission(res.mission)
      if (create_res) {
        ElMessage.success('Create mission success')
      }
      else {
        await store.deleteRecord('mission', res.mission.missionId, undefined)
        ElMessage.error('Failed to create mission')
      }
    }
    else {
      ElMessage.error('Failed to create mission')
    }
  }
  else if (props.mode === DialogMode.Edit) {
    const record = defaultRecord()
    record.mission = formData.value
    const res = await store.updateRecord('mission', record)
    if (!isBoolean(res))
      ElMessage.success('Update mission success')
    else
      ElMessage.error('Failed to create mission')
  }

  emit('hide', true)
  formData.value = defaultMission()
}
</script>

<template>
  <div class="container">
    <el-dialog
      v-model="visiable"
      :title="dialogTitle"
      class="dialog"
    >
      <el-form
        :model="formData"
        :rules="formRules"
        class="dialog__form"
      >
        <el-form-item :label="t('mission.name')" prop="name">
          <el-input
            v-model="formData.name"
            maxlength="20"
            show-word-limit
            type="text"
          />
        </el-form-item>

        <el-form-item :label="t('mission.description')" prop="description">
          <el-input
            v-model="formData.description"
            maxlength="20"
            show-word-limit
            type="text"
          />
        </el-form-item>

        <el-form-item :label="t('mission.pathType')" prop="pathType">
          <el-select
            v-model="formData.pathType"
          >
            <el-option
              v-for="item in pathTypeOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            />
          </el-select>
        </el-form-item>

        <el-form-item :label="t('mission.srcPath')" prop="srcPath">
          <div class="dialog__form__select">
            <div class="dialog__form__select__input">
              <el-tooltip effect="light" :content="formData.srcPath" placement="top" :disabled="formData.srcPath === ''">
                <el-input v-model="formData.srcPath" clearable :disabled="props.mode === DialogMode.Edit" />
              </el-tooltip>
            </div>
            <el-button type="primary" :disabled="props.mode === DialogMode.Edit" @click="selectPath('from')">
              {{ t("mission.select") }}
            </el-button>
          </div>
        </el-form-item>

        <el-form-item :label="t('mission.dstPath')" prop="dstPath">
          <div class="dialog__form__select">
            <div class="dialog__form__select__input">
              <el-tooltip effect="light" :content="formData.dstPath" placement="top">
                <el-input v-model="formData.dstPath" clearable />
              </el-tooltip>
            </div>
            <el-button type="primary" @click="selectPath('to')">
              {{ t("mission.select") }}
            </el-button>
          </div>
        </el-form-item>

        <el-form-item :label="t('mission.procedure')" prop="procedureId">
          <el-select
            v-model="formData.procedureId"
          >
            <el-option
              v-for="item in procedureOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            />
          </el-select>
        </el-form-item>
      </el-form>

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
  </div>
</template>

<style scoped lang="less">
.container {
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

.dialog__form {
    :deep(.el-form-item__label) {
        width: 100px;
    }

    :deep(.el-form-item_content) {
        width: 100%;
    }
}

.dialog__form__select {
    display: flex;
    flex-direction: row;
    width: -webkit-fill-available;
}

.dialog__form__ignore_config {
    display: flex;
    flex-direction: row;
}

.dialog__form__select__input {
    width: 100%;
    margin-right: 3px;
}
</style>
