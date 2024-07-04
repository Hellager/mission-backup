<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSystemStore } from '../store'
import { CloseOption } from '../store/system/types'

const props = defineProps({
  visiable: { type: Boolean, required: true },
})

const emit = defineEmits<{
  (event: 'hide', value: boolean): void
}>()

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 * Custom store for managing system settings.
 */
const store = useSystemStore()

/**
 * Represents the selected close option .
 *
 * Default value is `0` (Exit app).
 */
const closeOption = ref<number>(CloseOption.Exit)

/**
 * Indicates whether the selected option should be remembered.
 */
const remember = ref<boolean>(false)

/**
 * Controls the visibility of the dialog based on the `props.visiable` value.
 */
const visiable = computed({
  get: () => {
    return props.visiable
  },
  set: (value: boolean) => emit('hide', value),
})

watch(visiable, async (_new, _old) => {
  remember.value = false
  closeOption.value = store.closeOption
})

/**
 * Handle dialog cancel click event.
 */
async function onDialogCancelClicked() {
  emit('hide', true)
}

/**
 * Handle dialog confirm click event.
 */
async function onDialogConfirmClicked() {
  store.tryClose(closeOption.value, remember.value)
  emit('hide', true)
}
</script>

<template>
  <el-dialog
    v-model="visiable"
    :title="t('closeDialog.title')"
    class="dialog"
  >
    <div class="dialog__option">
      <el-radio-group v-model="closeOption">
        <el-radio :value="0" size="large">
          {{ t("config.system.closeExit") }}
        </el-radio>
        <el-radio :value="1" size="large">
          {{ t("config.system.closeTray") }}
        </el-radio>
      </el-radio-group>
    </div>

    <div class="dialog__remember">
      <el-checkbox v-model="remember" :label="t('closeDialog.remember')" size="large" />
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

<style lang="less" scoped>
</style>
