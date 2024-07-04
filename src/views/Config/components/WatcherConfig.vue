<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useWatcherStore } from '../../../store'

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 * Initializes the watcher store and sets up necessary variables and functions for managing notifications.
 */
const store = useWatcherStore()

/**
 * Reference to the delay value.
 */
const delay = ref<number>(3)

/**
 * Indicates whether the dialog is visible.
 */
const visiable = ref<boolean>(false)

/**
 * Indicates whether to relaunch.
 */
const relunch = ref<boolean>(false)

/**
 * Shows the edit dialog and sets the delay value to the store's timeout value.
 */
async function showEditDialog() {
  visiable.value = true
  delay.value = store.timeout
}

/**
 * Cancels the dialog operation and resets the delay value to the store's timeout value.
 */
async function onDialogCancelClicked() {
  visiable.value = false
  delay.value = store.timeout
}

/**
 * Confirms the dialog operation and updates the timeout value in the store.
 */
async function onDialogConfirmClicked() {
  visiable.value = false
  store.updateTimeout(delay.value, relunch.value)
}
</script>

<template>
  <div class="config">
    <el-form class="config__form">
      <el-form-item :label="t('config.watcher.timeout')">
        <el-button type="primary" @click="showEditDialog">
          {{ t("config.watcher.edit") }}
        </el-button>
      </el-form-item>
    </el-form>

    <el-dialog
      v-model="visiable"
      :title="t('config.watcher.title')"
      class="config__dialog"
    >
      <div class="config__dialog__timeout">
        <span>{{ t("config.watcher.timeout") }}</span>
        <el-input-number v-model="delay" :min="1" :max="60" />
      </div>

      <div class="config__dialog__relunch">
        <el-checkbox v-model="relunch" :label="t('config.watcher.relunch')" size="large" />
      </div>

      <template #footer>
        <div class="config__dialog__footer">
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
.config__form {
  :deep(.el-form-item__label) {
    width: 100px;
  }
}

.config__dialog__timeout {
  :deep(.el-input-number) {
    margin-left: 6px;
  }
}
</style>
