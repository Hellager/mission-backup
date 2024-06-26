<script setup lang="ts">
// import { reactive, ref } from 'vue';
import { useI18n } from 'vue-i18n'
import { storeToRefs } from 'pinia'
import { useNotifyStore } from '../../../store'

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 * Initializes the notify store and sets up necessary variables and functions for managing notifications.
 */
const store = useNotifyStore()
const { isGranted, enable, createBackup, failedBackup } = storeToRefs(store)

/**
 * Updates the notification enable status.
 *
 * @param value - The new value for notification enable status
 */
async function onEnableNotifyUpdate(value: boolean) {
  store.enableNotify(value)
}

/**
 * Updates the mission notification status.
 *
 * @param value - The new value for mission notification status
 */
async function onCreataBackupNotifyUpdate(value: boolean) {
  store.updateCreateBackupNotify(value)
}

/**
 * Updates the mission notification status.
 *
 * @param value - The new value for mission notification status
 */
async function onFailedBackupUpdate(value: boolean) {
  store.updateFailedBackupNotify(value)
}
</script>

<template>
  <div class="config">
    <el-form class="config__form">
      <el-form-item :label="t('config.notify.enable')" :disable="isGranted">
        <el-switch v-model="enable" @change="onEnableNotifyUpdate" />
      </el-form-item>

      <el-form-item :label="t('config.notify.createBackup')" :disable="isGranted">
        <el-switch v-model="createBackup" @change="onCreataBackupNotifyUpdate" />
      </el-form-item>

      <el-form-item :label="t('config.notify.failedBackup')" :disable="isGranted">
        <el-switch v-model="failedBackup" @change="onFailedBackupUpdate" />
      </el-form-item>
    </el-form>
  </div>
</template>

<style scoped lang="less">
.config__form {
  :deep(.el-form-item__label) {
    width: 110px;
  }
}
</style>
