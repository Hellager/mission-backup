<script setup lang="ts">
import type { PropType } from 'vue'
import { computed, reactive, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import cronstrue from 'cronstrue/i18n'
import { ElMessage } from 'element-plus'
import { isBoolean } from 'lodash'
import { useMissionStore } from '../../../store'
import { defaultProcedure, defaultRecord } from '../../../store/mission'
import { BackupRestrict, CompressFormat, IgnoreMethod, MissionTrigger } from '../../../store/mission/types'

import { DialogMode } from '../../../types'
import type { Ignore, Procedure, Record } from '../../../store/mission/types'
import IgnoreDialog from './IgnoreDialog.vue'

/**
 * Props definition for the dialog component.
 */
const props = defineProps({
  visiable: { type: Boolean, required: true },
  mode: { type: Number, required: true },
  data: { type: Object as PropType<Procedure>, required: true },
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
const { t, locale } = useI18n({ useScope: 'global' })

/**
 * Initializes the mission store.
 */
const store = useMissionStore()

/**
 * Options for selecting ignore method.
 */
const ignoreOptions = [
  {
    value: IgnoreMethod.Custom,
    label: t('procedure.customIgnore'),
  },
  {
    value: IgnoreMethod.Gitignore,
    label: t('procedure.gitIgnore'),
  },
]

/**
 * Options for selecting compress format.
 */
const compressOptions = [
  {
    value: CompressFormat.Zip,
    label: t('procedure.zip'),
  },
  {
    value: CompressFormat.Targz,
    label: t('procedure.targz'),
  },
  {
    value: CompressFormat.Tarbz2,
    label: t('procedure.tarbz2'),
  },
  {
    value: CompressFormat.Tarxz,
    label: t('procedure.tarxz'),
  },
  {
    value: CompressFormat.Sevenz,
    label: t('procedure.sevenz'),
  },
]

/**
 * Options for selecting mission trigger.
 */
const triggerOptions = [
  {
    value: MissionTrigger.Cron,
    label: t('procedure.cron'),
  },
  {
    value: MissionTrigger.Monitor,
    label: t('procedure.monitor'),
  },
]

/**
 * Options for selecting backup restrict.
 */
const RestrictOptions = [
  {
    value: BackupRestrict.None,
    label: t('procedure.restrictNone'),
  },
  {
    value: BackupRestrict.Days,
    label: t('procedure.restrictDays'),
  },
  {
    value: BackupRestrict.Size,
    label: t('procedure.restrictSize'),
  },
  {
    value: BackupRestrict.DaysAndSize,
    label: t('procedure.restrictDaysAndSize'),
  },
]

/**
 * Reference to the dialog title.
 */
const dialogTitle = ref<string>('')

/**
 * Reference to the cron hint message.
 */
const cronHint = ref<string>('')

/**
 * Reference to the form data for the procedure.
 */
const formData = ref<Procedure>(defaultProcedure())

/**
 * Indicates whether the ignore dialog is visible.
 */
const showIgnoreDialog = ref<boolean>(false)

/**
 * Mode for the ignore dialog (Create or Edit).
 */
const ignoreDialogMode = ref<number>(DialogMode.Create)

/**
 * Array to store new ignore entries.
 */
const newIgnores = ref<Ignore[]>([])

/**
 * Indicates whether the confirm button is in loading state.
 */
const confirmLoading = ref<boolean>(false)

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
    dialogTitle.value = t('procedure.newTitle')
    formData.value = defaultProcedure()
  }
  else if (props.mode === DialogMode.Edit) {
    dialogTitle.value = t('procedure.editTitle')
    formData.value = props.data
  }
})

/**
 * Builds the cron hint message based on the cron expression.
 *
 * @param value - The cron expression to build the hint for
 */
function buildCronHint(value: string) {
  try {
    const cron_locale = locale.value === 'zh-CN' ? 'zh_CN' : 'en'
    cronHint.value = cronstrue.toString(value, { locale: cron_locale })
  }
  catch (err) {
    cronHint.value = ''
    ElMessage.error('Invalid expression')
  }
}

/**
 * Validates the cron expression.
 *
 * @param value - The cron expression to validate
 * @returns True if the expression is valid, false otherwise
 */
function isValidCronExpression(value: string) {
  if (formData.value.trigger !== MissionTrigger.Cron)
    return true

  try {
    const cron_locale = locale.value === 'zh-CN' ? 'zh_CN' : 'en'
    cronHint.value = cronstrue.toString(value, { locale: cron_locale })
    return true
  }
  catch (err) {
    cronHint.value = ''
    return false
  }
}

/**
 * Reactive object for form validation rules.
 */
const formRules = reactive({
  name: [{ required: true, message: '', trigger: 'blur' }],
  // cronExpression: [{ validator: validate_cron, trigger: 'blur' }],
  hasIgnore: [{ required: true, message: '', trigger: 'blur' }],
  // ignoreMethod: [{ required: true, message: '', trigger: 'blur' }],
  isCompress: [{ required: true, message: '', trigger: 'blur' }],
  // compressFormat: [{ required: true, message: '', trigger: 'blur' }],
  trigger: [{ required: true, message: '', trigger: 'blur' }],
  restrict: [{ required: true, message: '', trigger: 'blur' }],
  restrictDays: [{ required: true, message: '', trigger: 'blur' }],
  restrict_size: [{ required: true, message: '', trigger: 'blur' }],
})

/**
 * Handles the action to open the custom ignore dialog.
 */
async function onCustomIgnoreClicked() {
  ignoreDialogMode.value = props.mode
  showIgnoreDialog.value = true
}

/**
 * Handles the action for the ignore dialog.
 */
async function handleDialogAction() {
  showIgnoreDialog.value = false
}

/**
 * Handles the action to create new ignore entries.
 *
 * @param data - The new ignore entries to create
 */
async function onCreateIgnores(data: Ignore[]) {
  newIgnores.value = data
}

/**
 * Handles the cancel action for the dialog.
 */
async function onDialogCancelClicked() {
  formData.value = defaultProcedure()
  emit('hide', true)
}

/**
 * Handles the confirm action for the dialog, creating or updating procedure and ignore entries.
 */
async function onDialogConfirmClicked() {
  if (!isValidCronExpression(formData.value.cronExpression)) {
    ElMessage.error('Invalid expression')
    return
  }

  if (formData.value.name === '') {
    ElMessage.error('Empty name')
    return
  }

  // console.log('cur target:')
  // console.log(formData.value.procedureId)

  confirmLoading.value = true

  if (props.mode === DialogMode.Create) {
    const record = defaultRecord()
    record.procedure = formData.value
    const res = await store.createRecord('procedure', record)
    // console.log(res)
    if (!isBoolean(res)) {
      ElMessage.success('Create procedure success')

      let failed_cnt: number = 0
      let ignore_res: Record | boolean = false
      // console.log('start create ignore')
      for (const item of newIgnores.value) {
        record.ignore = item
        record.procedure = res.procedure
        ignore_res = await store.createRecord('ignore', record)
        if (isBoolean(ignore_res))
          failed_cnt++
      }

      ElMessage.success(`${newIgnores.value.length - failed_cnt} ignores created, ${failed_cnt} failed`)
    }
    else {
      ElMessage.error('Failed to create procedure')
    }
  }
  else if (props.mode === DialogMode.Edit) {
    const record = defaultRecord()
    record.procedure = formData.value
    const res = await store.updateRecord('procedure', record)
    if (!isBoolean(res)) {
      ElMessage.success('Update procedure success')
      // console.log('clear record')
      await store.deleteRecord('ignore', '', res.procedure.procedureId)

      let failed_cnt: number = 0
      let ignore_res: Record | boolean = false
      // console.log('start create ignore')
      for (const item of newIgnores.value) {
        record.ignore = item
        record.procedure = res.procedure
        ignore_res = await store.createRecord('ignore', record)
        if (isBoolean(ignore_res))
          failed_cnt++
      }

      ElMessage.success(`${newIgnores.value.length - failed_cnt} ignores created, ${failed_cnt} failed`)
    }
    else {
      ElMessage.error('Failed to create procedure')
    }
  }

  confirmLoading.value = false
  emit('hide', true)
  formData.value = defaultProcedure()
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
        <el-form-item :label="t('procedure.name')" prop="name">
          <el-input
            v-model="formData.name"
            maxlength="20"
            show-word-limit
            type="text"
          />
        </el-form-item>

        <el-form-item :label="t('procedure.hasIgnore')" prop="hasIgnore">
          <div class="dialog__form__ignore">
            <div class="dialog__form__ignore__switch">
              <el-switch v-model="formData.hasIgnore" />
            </div>

            <div class="dialog__form__ignore_config">
              <el-select
                v-if="formData.hasIgnore"
                v-model="formData.ignoreMethod"
                class="dialog__form__ignore__config__select"
              >
                <el-option
                  v-for="item in ignoreOptions"
                  :key="item.value"
                  :label="item.label"
                  :value="item.value"
                />
              </el-select>

              <el-button
                v-if="formData.ignoreMethod === IgnoreMethod.Custom
                  && formData.hasIgnore === true"
                type="primary"
                @click="onCustomIgnoreClicked"
              >
                {{ t('procedure.edit') }}
              </el-button>
            </div>
          </div>
        </el-form-item>

        <el-form-item :label="t('procedure.isCompress')" prop="isCompress">
          <el-switch v-model="formData.isCompress" />

          <el-select
            v-if="formData.isCompress"
            v-model="formData.compressFormat"
          >
            <el-option
              v-for="item in compressOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            />
          </el-select>
        </el-form-item>

        <el-form-item :label="t('procedure.trigger')" prop="trigger">
          <el-select
            v-model="formData.trigger"
          >
            <el-option
              v-for="item in triggerOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            />
          </el-select>

          <!-- <el-input
                        v-if="formData.trigger === MissionTrigger.Cron"
                        v-model.trim="formData.cronExpression"
                        :placeholder="t('procedure.cronExpression')"
                        style="width: 240px"
                        type="text"
                    /> -->

          <div v-if="formData.trigger === MissionTrigger.Cron" class="dialog__form__trigger__cron__expression">
            <el-tooltip :content="cronHint" :trigger-keys="[]" :disabled="cronHint === ''" :show-after="2000" placement="top" effect="light">
              <el-input
                v-model.trim="formData.cronExpression"
                :placeholder="t('procedure.cronExpression')"
                type="text"
                @blur="buildCronHint(formData.cronExpression)"
              />
            </el-tooltip>
          </div>
        </el-form-item>

        <!-- <el-form-item :label="t('procedure.cronExpression')">
                    <el-switch v-model="formData.cronExpression"/>
                </el-form-item> -->

        <el-form-item :label="t('procedure.restrict')" prop="restrict">
          <el-select
            v-model="formData.restrict"
          >
            <el-option
              v-for="item in RestrictOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            />
          </el-select>
        </el-form-item>

        <el-form-item
          v-if="formData.restrict === BackupRestrict.Days || formData.restrict === BackupRestrict.DaysAndSize"
          :label="t('procedure.restrictDays')"
          prop="restrictDays"
        >
          <div class="dialog__form__restrict">
            <div class="dialog__form__restrict__input">
              <el-input-number
                v-model.trim="formData.restrictDays"
              />
            </div>

            <div class="dialog__form__restrict__unit">
              <span>{{ t("procedure.day") }}</span>
            </div>
          </div>
        </el-form-item>

        <el-form-item
          v-if="formData.restrict === BackupRestrict.Size || formData.restrict === BackupRestrict.DaysAndSize"
          :label="t('procedure.restrictSize')"
          prop="restrict_size"
        >
          <div class="dialog__form__restrict">
            <div class="dialog__form__restrict__input">
              <el-input-number
                v-model.trim="formData.restrictSize"
              />
            </div>

            <div class="dialog__form__restrict__unit">
              <span>{{ t("procedure.size") }}</span>
            </div>
          </div>
        </el-form-item>

        <!-- <el-form-item :label="t('procedure.backupMethod')">
                    <el-switch v-model="formData.f_mission_notify"  @change="on_mission_notify_update"/>
                </el-form-item> -->
      </el-form>

      <template #footer>
        <div class="dialog-footer">
          <el-button @click="onDialogCancelClicked">
            {{ t("common.cancel") }}
          </el-button>
          <el-button type="primary" :loading="confirmLoading" @click="onDialogConfirmClicked">
            {{ t("common.confirm") }}
          </el-button>
        </div>
      </template>
    </el-dialog>

    <IgnoreDialog
      :visiable="showIgnoreDialog" :mode="ignoreDialogMode" :uuid="props.data.procedureId"
      @hide="handleDialogAction"
      @create-ignores="onCreateIgnores"
    />
  </div>
</template>

<style scoped lang="less">
.dialog__form {
    :deep(.el-form-item__label) {
        width: 100px;
    }

    :deep(.el-form-item_content) {
        width: 100%;
    }
}

.dialog__form__ignore {
    display: flex;
    flex-direction: column;
    width: -webkit-fill-available;
}

.dialog__form__ignore_config {
    display: flex;
    flex-direction: row;
}

.dialog__form__ignore__config__select {
    margin-right: 3px;
}

.dialog__form__restrict {
    display: flex;
    flex-direction: row;
}

.dialog__form__restrict__unit {
    margin-left: 3px;
}
</style>
