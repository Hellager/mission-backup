<script setup lang="ts">
import { reactive, ref } from 'vue'
import { storeToRefs } from 'pinia'
import { useI18n } from 'vue-i18n'
import { ElMessage } from 'element-plus'
import { useScreensaverStore } from '../../../store'

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 * Initializes the screensaver store and sets up necessary variables and functions for managing notifications.
 */
const store = useScreensaverStore()
const { enable } = storeToRefs(store)

/**
 * Controls the visibility of the password dialog
 */
const visiable = ref<boolean>(false)

/**
 * Reactive object for password form data.
 */
const pwdForm = reactive({
  old: '',
  new: '',
})

/**
 * Updates the screensaver enable status.
 *
 * @param value - The new value for screensaver enable status
 */
async function onEnableScreensaverUpdate(value: boolean) {
  store.enableScreensaver(value)
}

/**
 * Shows the edit dialog for screensaver password.
 */
async function showPwdDialog() {
  visiable.value = true
}

/**
 * Cancels the password dialog operation.
 */
async function onPwdDialogCancel() {
  pwdForm.old = ''
  pwdForm.new = ''
  visiable.value = false
}

/**
 * Confirms the password dialog operation and updates the password if old password matches.
 */
async function onPwdDialogConfirm() {
  if (pwdForm.old !== store.password) {
    ElMessage.error('Wrong password')
    return
  }
  else {
    store.updatePassword(pwdForm.new)
    ElMessage.success('Update success!')
  }
  visiable.value = false
}
</script>

<template>
  <div class="config">
    <el-form class="config__form">
      <el-form-item :label="t('config.screensaver.enable')">
        <el-switch v-model="enable" @change="onEnableScreensaverUpdate" />
      </el-form-item>

      <el-form-item :label="t('config.screensaver.password')">
        <el-button type="primary" :disabled="!enable" @click="showPwdDialog">
          {{ t("config.screensaver.edit") }}
        </el-button>
      </el-form-item>
    </el-form>

    <el-dialog
      v-model="visiable"
      :title="t('config.screensaver.title')"
      class="config__dialog"
    >
      <el-form :model="pwdForm">
        <el-form-item :label="t('config.screensaver.oldPwd')" required>
          <el-input
            v-model="pwdForm.old"
            type="password"
            show-password
          />
        </el-form-item>

        <el-form-item :label="t('config.screensaver.newPwd')" required>
          <el-input
            v-model="pwdForm.new"
            type="password"
            show-password
          />
        </el-form-item>
      </el-form>

      <template #footer>
        <div class="dialog-footer">
          <el-button @click="onPwdDialogCancel">
            {{ t("common.cancel") }}
          </el-button>
          <el-button type="primary" @click="onPwdDialogConfirm">
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
</style>
