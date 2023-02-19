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
const { language } = storeToRefs(globalSetting)

const dialog_visiable = ref(props.isShow)
const input_origin_password = ref('')
const input_new_password = ref('')

const ruleFormRef = ref<FormInstance>()
const ruleForm = reactive({
  origin_password: '',
  new_password: '',
})
const rules = reactive<FormRules>({
  origin_password: [
    {
      required: true,
      message: t('setting.inputOriginPassword'),
      trigger: 'blur',
    },
  ],
  new_password: [
    {
      required: true,
      message: t('setting.inputNewPassword'),
      trigger: 'blur',
    },
  ],
})

watch(props, () => {
  dialog_visiable.value = props.isShow
})

const close_dialog = async (formEl: FormInstance | undefined) => {
  dialog_visiable.value = false
  formEl?.resetFields()
  emit('close')
}

async function toggle_change_password() {
  const res = await execute_rust_command(TauriCommand.COMMAND_CHANGE_SETTING_PASSWORD, ruleForm.origin_password, ruleForm.new_password)
  if (res) {
    ruleForm.origin_password = ''
    ruleForm.new_password = ''
    dialog_visiable.value = false
    ElMessage.success({
      showClose: true,
      message: t('setting.changePasswordSuccess'),
      center: true,
    })
  }
  else {
    ruleForm.origin_password = ''
    ruleForm.new_password = ''
    ElMessage.error({
      showClose: true,
      message: t('error.changePasswordFailed'),
      center: true,
    })
  }

  emit('close')
}

const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl)
    return
  await formEl.validate((valid, fields) => {
    if (valid) {
      toggle_change_password()
    }

    else {
      ElMessage.error({
        showClose: true,
        message: t('error.changePasswordFailed'),
        center: true,
      })
    }
  })
}
</script>

<template>
  <el-dialog v-model="dialog_visiable" :show-close="false" :title="props.title">
    <el-form
      ref="ruleFormRef"
      :model="ruleForm"
      :rules="rules"
      class="setting-dialog"
      label-position="right"
      :label-width="language === 'zh-CN' ? 'auto' : '120px'"
    >
      <el-form-item :label="t('setting.originPassword')" prop="origin_password">
        <el-input v-model="ruleForm.origin_password" class="oldPasswordInput" show-password />
      </el-form-item>
      <el-form-item :label="t('setting.newPassword')" prop="new_password">
        <el-input v-model="ruleForm.new_password" show-password />
      </el-form-item>
    </el-form>
    <template #footer>
      <DialogFooter @cancel="close_dialog(ruleFormRef)" @confirm="submitForm(ruleFormRef)" />
    </template>
  </el-dialog>
</template>

<style lang="less">
</style>
