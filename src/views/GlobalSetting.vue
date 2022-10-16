<script setup lang="ts">
import { ref } from 'vue'
import type { FormInstance } from 'element-plus'
import { InfoFilled, Moon, Sunny } from '@element-plus/icons-vue'
import { storeToRefs } from 'pinia'
import { useI18n } from 'vue-i18n'
import { useMissionStore, useSettingStore } from '../store/index'
import PageHeader from '../components/PageHeader.vue'
import Lock from '../components/Lock.vue'
import { TauriCommand, execute_rust_command } from '../utils'
// import { ElMessage } from 'element-plus';

const { t, locale } = useI18n({ useScope: 'global' })

const globalSettingFormRef = ref<FormInstance>()
const globalSetting = useSettingStore()
const missionStore = useMissionStore()
const {
  is_auto_start,
  is_light_theme,
  is_close_to_tray,
  language,
  is_password_protected,
  monitor_delay,
  software_version,
} = storeToRefs(globalSetting)
const { mission_list } = storeToRefs(missionStore)

const langOptions = [
  {
    value: 'zh-CN',
    label: '简体中文',
  },
  {
    value: 'en-US',
    label: 'English',
  },
]

const input_origin_password = ref('')
const input_new_password = ref('')
const change_password_dialog_visiable = ref(false)
const input_delay_time = ref(monitor_delay.value)
const change_delay_dialog_visiable = ref(false)

const show_error_message = (mode: string) => {
  let msg = ''
  if (mode === 'normal')
    msg = t('error.changeSettingFailed')
  else if (mode === 'password')
    msg = t('error.changePasswordFailed')
  else if (mode === 'delay')
    msg = t('error.changeDelayFailed')

  ElMessage.error({
    showClose: true,
    message: msg,
    center: true,
  })
}

const toggle_dialog_cancel = () => {
  input_origin_password.value = ''
  input_new_password.value = ''
  change_password_dialog_visiable.value = false
  change_delay_dialog_visiable.value = false
}

async function toggle_change_auto_start(data: boolean) {
  const res = await execute_rust_command(TauriCommand.COMMAND_CHANGE_SETTING_IS_AUTO_START, data)
  if (res) {
    console.log(globalSetting.is_auto_start)
    globalSetting.update_auto_start(data)
  }
  else {
    globalSetting.update_auto_start(!data)
    show_error_message('normal')
  }
}

async function toggle_change_theme(data: boolean) {
  const res = await execute_rust_command(TauriCommand.COMMAND_CHANGE_SETTING_IS_LIGHT_THEME, data)
  if (res) {
    globalSetting.update_theme(data)
    const root_element = document.documentElement
    root_element.setAttribute('class', data ? '' : 'dark')
  }
  else {
    globalSetting.update_theme(!data)
    show_error_message('normal')
  }
}

async function toggle_change_language(data: string) {
  const res = await execute_rust_command(TauriCommand.COMMAND_CHANGE_SETTING_LANGUAGE, data)
  if (res) {
    locale.value = data
    globalSetting.update_language(data)
  }
  else {
    show_error_message('normal')
  }
}

async function toggle_change_close_event(data: boolean) {
  const res = await execute_rust_command(TauriCommand.COMMAND_CHANGE_SETTING_IS_CLOSE_TO_TRAY, data)
  if (res) {
    globalSetting.update_close_to_tray(data)
  }
  else {
    globalSetting.update_close_to_tray(!data)
    show_error_message('normal')
  }
}

async function toggle_change_password_protect(data: boolean) {
  const res = await execute_rust_command(TauriCommand.COMMAND_CHANGE_SETTING_IS_PASSWORD_PROTECTED, data)
  if (res) {
    globalSetting.update_password_protect(data)
  }
  else {
    globalSetting.update_password_protect(!data)
    show_error_message('normal')
  }
}

async function toggle_change_password() {
  const res = await execute_rust_command(TauriCommand.COMMAND_CHANGE_SETTING_PASSWORD, input_origin_password.value, input_new_password.value)
  if (res) {
    input_origin_password.value = ''
    input_new_password.value = ''
    change_password_dialog_visiable.value = false
    ElMessage.success({
      showClose: true,
      message: t('setting.changePasswordSuccess'),
      center: true,
    })
  }
  else {
    input_origin_password.value = ''
    input_new_password.value = ''
    show_error_message('password')
  }
}

async function toggle_change_monitor_delay() {
  let any_trouble = false
  if (typeof input_delay_time.value != 'number') {
    ElMessage.error({
      showClose: true,
      message: t('error.dataFormatError'),
      center: true,
    })
    any_trouble = true
  }

  mission_list.value.forEach((item) => {
    if (item.config.monitor_enable && item.info.status === 'executing') {
      ElMessage.error({
        showClose: true,
        message: t('error.jobRunningError'),
        center: true,
      })
    }

    any_trouble = true
  })

  if (any_trouble)
    return

  const res = await execute_rust_command(TauriCommand.COMMAND_CHANGE_SETTING_MONITOR_DELAY, input_delay_time.value)
  if (res) {
    globalSetting.update_monitor_delay(input_delay_time.value)
    change_delay_dialog_visiable.value = false
    ElMessage.success({
      showClose: true,
      message: t('setting.changeDelaySuccess'),
      center: true,
    })
  }
  else {
    show_error_message('delay')
  }
}
</script>

<template>
  <div class="container">
    <PageHeader :title="t('component.pageHeaderGlobalSetting')" to="/" />
    <div class="settingForm">
      <el-form
        ref="globalSettingFormRef"
        label-width="130px"
      >
        <el-row :gutter="10" justify="space-between">
          <el-col :span="12">
            <el-form-item :label="t('setting.autoStart')">
              <el-switch v-model="is_auto_start" @change="toggle_change_auto_start" />
            </el-form-item>
          </el-col>
          <el-col :span="12" :pull="2">
            <el-form-item :label="t('setting.theme')">
              <el-switch
                v-model="is_light_theme"
                inline-prompt
                :active-icon="Sunny"
                :inactive-icon="Moon"
                @change="toggle_change_theme"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="t('setting.systemTray')">
              <el-switch
                v-model="is_close_to_tray"
                @change="toggle_change_close_event"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12" :pull="2">
            <el-form-item :label="t('setting.language')">
              <el-select v-model="language" placeholder="" @change="toggle_change_language">
                <el-option
                  v-for="item in langOptions"
                  :key="item.value"
                  :label="item.label"
                  :value="item.value"
                />
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="t('setting.security')">
              <el-switch
                v-model="is_password_protected"
                @change="toggle_change_password_protect"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12" :pull="2">
            <el-form-item :label="t('setting.resetPassword')">
              <el-button type="primary" @click="change_password_dialog_visiable = true">
                {{ t('setting.clickToReset') }}
              </el-button>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="t('setting.resetMonitorDelay')">
              <el-button type="primary" @click="change_delay_dialog_visiable = true">
                {{ t('setting.clickToReset') }}
              </el-button>
            </el-form-item>
          </el-col>
          <el-col :span="12" :pull="2">
            <el-form-item :label="t('setting.version')">
              <div class="software-version">
                {{ software_version }}
              </div>
            </el-form-item>
          </el-col>
        </el-row>
      </el-form>
    </div>

    <el-dialog v-model="change_password_dialog_visiable" :show-close="false" :title="t('setting.resetPassword')">
      <el-form class="setting-dialog" label-position="right" :label-width="language === 'zh-CN' ? 'auto' : '110px'">
        <el-form-item :label="t('setting.originPassword')">
          <el-input v-model="input_origin_password" class="oldPasswordInput" show-password />
        </el-form-item>
        <el-form-item :label="t('setting.newPassword')">
          <el-input v-model="input_new_password" show-password />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="toggle_dialog_cancel">{{ t('general.cancel') }}</el-button>
          <el-button type="primary" @click="toggle_change_password">{{ t('general.confirm') }}</el-button>
        </span>
      </template>
    </el-dialog>

    <el-dialog v-model="change_delay_dialog_visiable" :show-close="false" :title="t('setting.resetMonitorDelay')">
      <el-form class="setting-dialog" label-position="right" :label-width="language === 'zh-CN' ? 'auto' : '110px'">
        <el-form-item :label="t('setting.targetDelay')">
          <el-input-number v-model="input_delay_time" :min="1" :max="60" />
        </el-form-item>
      </el-form>
      <span class="delayTip">
        {{ t('general.validAfterRestart') }}
      </span>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="toggle_dialog_cancel">{{ t('general.cancel') }}</el-button>
          <el-button type="primary" @click="toggle_change_monitor_delay">{{ t('general.confirm') }}</el-button>
        </span>
      </template>
    </el-dialog>

    <Lock :tray="['lock', 'home']" />
  </div>
</template>

<style lang="less" scoped>
@import "../assets/style/theme/default-vars.less";
.container {
    width: 100%;
    min-height: 100vh;
    padding-top: @title-bar-height;
    color: var(--el-color-primary);
    background-color: var(--el-bg-color);

    :deep(.el-overlay-dialog){
        &::-webkit-scrollbar {
            display: none;
        }

        -ms-overflow-style: none;
        scrollbar-width: none;
    }
}

.title {
    padding-top: 20px;
    text-align: center;
}

.action {
    margin-top: 20px;
    text-align: center;
}

.settingForm {
    padding-top: 28px;
    padding-bottom: 26px;
}

.content {
    display: flex;
    flex-direction: row;
}

.menu_tabs {
    display: flex;
    flex-direction: column;
}

.delayTip {
    display: flex;
    flex-direction: row-reverse;
    font-size: 78%;
}

.software-version {
    color: var(--el-text-color-regular);
}
</style>
