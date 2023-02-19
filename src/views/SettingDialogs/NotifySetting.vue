<script lang="ts" setup>
import { reactive, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { storeToRefs } from 'pinia'
import type { FormInstance, FormRules } from 'element-plus'
import { useSettingStore } from '../../store/index'
import { TauriCommand, execute_rust_command } from '../../utils'
import DialogFooter from './DialogFooter.vue'

const props = defineProps({
  isShow: Boolean,
  title: String,
})

const emit = defineEmits(['close'])

const globalSetting = useSettingStore()
const { t, locale } = useI18n({ useScope: 'global' })
const {
  is_notify_when_create_backup_success,
  is_notify_when_create_backup_failed,
} = storeToRefs(globalSetting)

const dialog_visiable = ref(props.isShow)

const ruleFormRef = ref<FormInstance>()
const ruleForm = reactive({
  notify_when_create_backup_success: is_notify_when_create_backup_success.value,
  notify_when_create_backup_failed: is_notify_when_create_backup_success.value,
})

watch(props, () => {
  dialog_visiable.value = props.isShow
})

const close_dialog = () => {
  dialog_visiable.value = false
  emit('close')
}

const confirm_dialog = () => {
  emit('close')
}

async function toggle_change_is_notify_when_create_backup_success(data: boolean) {
  const res = await execute_rust_command(TauriCommand.COMMAND_CHANGE_IS_NOTIFY_WHEN_CREATE_BACKUP_SUCCESS, data)
  if (res) {
    globalSetting.update_notify_when_create_backup_success(data)
  }
  else {
    globalSetting.update_notify_when_create_backup_success(!data)
    ElMessage.error({
      showClose: true,
      message: t('error.changeNotifyFailed'),
      center: true,
    })
  }
}

async function toggle_change_is_notify_when_create_backup_failed(data: boolean) {
  const res = await execute_rust_command(TauriCommand.COMMAND_CHANGE_IS_NOTIFY_WHEN_CREATE_BACKUP_FAILED, data)
  if (res) {
    globalSetting.update_notify_when_create_backup_failed(data)
  }
  else {
    globalSetting.update_notify_when_create_backup_failed(!data)
    ElMessage.error({
      showClose: true,
      message: t('error.changeNotifyFailed'),
      center: true,
    })
  }
}
</script>

<template>
  <el-dialog v-model="dialog_visiable" :show-close="false" :title="props.title">
    <el-form
      ref="ruleFormRef"
      :model="ruleForm"
      class="setting-dialog"
      label-position="left"
      label-width="162px"
    >
      <el-form-item :label="t('setting.notifyWhenBackupSuccess')" prop="notify_when_create_backup_success">
        <el-switch v-model="ruleForm.notify_when_create_backup_success" @change="toggle_change_is_notify_when_create_backup_success" />
      </el-form-item>
      <el-form-item :label="t('setting.notifyWhenBackupFailed')" prop="notify_when_create_backup_failed">
        <el-switch v-model="ruleForm.notify_when_create_backup_failed" @change="toggle_change_is_notify_when_create_backup_failed" />
      </el-form-item>
    </el-form>
    <template #footer>
      <DialogFooter @cancel="close_dialog" @confirm="confirm_dialog" />
    </template>
  </el-dialog>
</template>

<style lang="less">
</style>
