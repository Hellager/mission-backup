<script setup lang="ts">
import { ref } from 'vue'
import type { FormInstance } from 'element-plus'
import { InfoFilled, Moon, Sunny } from '@element-plus/icons-vue'
import { storeToRefs } from 'pinia'
import { useI18n } from 'vue-i18n'
import { fetch } from '@tauri-apps/api/http'
import { useMissionStore, useSettingStore } from '../store/index'
import PageHeader from '../components/PageHeader.vue'
import Lock from '../components/Lock.vue'
import { TauriCommand, execute_rust_command } from '../utils'
import PasswordReSet from './SettingDialogs/PasswordReSet.vue'
import DelaySetting from './SettingDialogs/DelaySetting.vue'
import NotifySetting from './SettingDialogs/NotifySetting.vue'
import ContactSetting from './SettingDialogs/ContactSetting.vue'
import UpdateMessage from './SettingDialogs/UpdateMessage.vue'
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

const password_dialog_display = ref(false)
const delay_dialog_display = ref(false)
const notify_dialog_display = ref(false)
const update_dialog_display = ref(false)
const update_msg = ref('')
const release_url = ref('')

const show_error_message = (mode: string) => {
  let msg = ''
  if (mode === 'normal')
    msg = t('error.changeSettingFailed')
  else if (mode === 'password')
    msg = t('error.changePasswordFailed')
  else if (mode === 'delay')
    msg = t('error.changeDelayFailed')
  else if (mode === 'update')
    msg = t('error.checkUpdateFailed')

  ElMessage.error({
    showClose: true,
    message: msg,
    center: true,
  })
}

const show_dialog = (dialog: String) => {
  switch (dialog) {
    case 'password':
      password_dialog_display.value = !password_dialog_display.value
      break

    case 'delay':
      delay_dialog_display.value = !delay_dialog_display.value
      break

    case 'notify':
      notify_dialog_display.value = !notify_dialog_display.value
      break

    case 'update':
      update_dialog_display.value = !update_dialog_display.value
      break

    default:
      break
  }
}

const close_dialog = () => {
  password_dialog_display.value = false
  delay_dialog_display.value = false
  notify_dialog_display.value = false
  update_dialog_display.value = false
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

async function check_for_update() {
  const local_version: any = globalSetting.software_version

  const unsafe_response = await fetch(
    'https://api.github.com/repos/Hellager/mission-backup/releases/latest',
    {
      method: 'GET',
      timeout: 30,
    },
  )

  const response: any = Object.assign({}, unsafe_response)

  if (response.code === 204) {
    show_error_message('update')
  }
  else {
    // console.log(response.data)
    const en_abstract_idx = response.data.body.indexOf('English Abstract:')
    const cn_abstract_idx = response.data.body.indexOf('中文摘要：')

    update_msg.value = (globalSetting.language === 'en-US'
      ? response.data.body.substring(en_abstract_idx + 'English Abstract:'.length + 2, cn_abstract_idx)
      : response.data.body.substring(cn_abstract_idx + '中文摘要：'.length + 2)
    )

    release_url.value = response.data.html_url
    const remote_version = response.data.tag_name

    if (local_version === null || remote_version === null)
      show_error_message('update')

    const local_ver_arr = local_version.match(/\d+/g)
    const remote_ver_arr = remote_version.match(/\d+/g)

    remote_ver_arr[2] += 1

    let isNeedUpdate = false

    for (let i = 0; i < 3; i++) {
      if (parseInt(remote_ver_arr[i]) > parseInt(local_ver_arr[i])) {
        isNeedUpdate = true
        break
      }
    }

    if (isNeedUpdate) { show_dialog('update') }
    else {
      ElMessage({
        showClose: true,
        message: t('setting.noNeedUpdate'),
      })
    }
  }
}

async function open_user_guidance() {
  console.log('click to open user guidance')
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
              <el-button type="primary" @click="show_dialog('password')">
                {{ t('setting.clickToReset') }}
              </el-button>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="t('setting.resetMonitorDelay')">
              <el-button type="primary" @click="show_dialog('delay')">
                {{ t('setting.clickToReset') }}
              </el-button>
            </el-form-item>
          </el-col>
          <el-col :span="12" :pull="2">
            <el-form-item :label="t('setting.userGuidance')">
              <el-button type="primary" @click="open_user_guidance">
                {{ t('setting.clickToOpenGuidance') }}
              </el-button>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="t('setting.setNotify')">
              <el-button type="primary" @click="show_dialog('notify')">
                {{ t('setting.clickToSet') }}
              </el-button>
            </el-form-item>
          </el-col>
          <el-col :span="12" :pull="2">
            <el-form-item :label="t('setting.contactDeveloper')">
              <ContactSetting />
            </el-form-item>
          </el-col>

          <el-col :span="12">
            <el-form-item :label="t('setting.checkUpdate')">
              <el-button type="primary" @click="check_for_update">
                {{ t('setting.clickToCheckUpdate') }}
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

    <PasswordReSet :is-show="password_dialog_display" :title="t('setting.resetPassword')" @close="close_dialog" />
    <DelaySetting :is-show="delay_dialog_display" :title="t('setting.resetMonitorDelay')" @close="close_dialog" />
    <NotifySetting :is-show="notify_dialog_display" :title="t('setting.setNotifyTitle')" @close="close_dialog" />
    <UpdateMessage :is-show="update_dialog_display" :title="t('setting.goUpdate')" :message="update_msg" :release-url="release_url" @close="close_dialog">
      <Lock :tray="['lock', 'home']" />
    </updatemessage>
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
