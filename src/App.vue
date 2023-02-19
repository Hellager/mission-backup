<script setup lang="ts">
import { onMounted, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { useI18n } from 'vue-i18n'
import type { UnlistenFn } from '@tauri-apps/api/event'
import { listen } from '@tauri-apps/api/event'
import { appWindow } from '@tauri-apps/api/window'
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification'
import { useMissionStore, useSettingStore } from './store/index'
import { TauriCommand, execute_rust_command } from './utils/index'
import router from './router'
import TitleBar from './components/TitleBar.vue'

const { t, locale } = useI18n({ useScope: 'global' })
const globalSetting = useSettingStore()
const missionStore = useMissionStore()
const {
  is_initialized,
  language,
  is_notify_when_create_backup_success,
  is_notify_when_create_backup_failed,
} = storeToRefs(globalSetting)

/**
 * initialize data when mounted
 */
async function initialize_data() {
  await execute_rust_command(TauriCommand.COMMAND_INITIALIZE_DATA)
    .then((data) => {
      globalSetting.initialize_settings(data.setting)
      missionStore.initialize_mission_list(data.list)
      locale.value = data.setting.language
      is_initialized.value = true
    })
}

async function initialize_program_status() {
  await execute_rust_command(TauriCommand.COMMAND_INITIALIZE_PROGRAM_STATUS)
}

async function initialize_mission() {
  const is_initialized = await execute_rust_command(TauriCommand.COMMAND_IS_INITIALIZED)
  if (is_initialized)
    return

  const mission_list = missionStore.mission_list
  const error_list: any = []
  for (const mission of mission_list) {
    if (mission.info.status !== 'pausing') {
      await execute_rust_command(TauriCommand.COMMAND_START_MISSION, mission.config.id)
        .then((res) => {
          if (!res.data) {
            const errMsg = `${mission.config.name}: ${res.msg.includes('path') ? t('error.unavailablePath') : t('error.startMissionFailed')}`
            error_list.push({
              showClose: true,
              message: errMsg,
              center: true,
            })

            missionStore.update_mission_status(mission.config.id, 'unavailable')
          }
        })
    }
  }

  error_list.forEach((element: any) => {
    ElMessage.error(element)
  })
}

async function initialize_system_tray() {
  const current_lang = language.value
  await execute_rust_command(TauriCommand.COMMAND_CHANGE_SETTING_LANGUAGE, current_lang)
}

async function initialize_page_theme() {
  const current_theme = globalSetting.is_light_theme
  const res = await execute_rust_command(TauriCommand.COMMAND_CHANGE_SETTING_IS_LIGHT_THEME, current_theme)
  if (res) {
    globalSetting.update_theme(current_theme)
    const root_element = document.documentElement
    root_element.setAttribute('class', current_theme ? '' : 'dark')
  }
  else {
    globalSetting.update_theme(!current_theme)
  }
}

async function initialize_timing_save_data() {
  const is_initialized = await execute_rust_command(TauriCommand.COMMAND_IS_INITIALIZED)
  if (is_initialized)
    return
  await execute_rust_command(TauriCommand.COMMAND_START_TIMING_SAVE_DATA)
}

const build_router_transitionname = (name: any): string => {
  return (name === undefined ? '' : name)
}

/**
 * Notification related
 */
async function get_notify_permission() {
  let permission_granted = await isPermissionGranted()
  if (!permission_granted) {
    const permission = await requestPermission()
    permission_granted = permission === 'granted'
  }

  return permission_granted
}

/**
 * listen events
 */
const listen_dict: Map<string, UnlistenFn> = new Map()

async function listen_to_any_error() {
  const is_notify_permission_granted = await get_notify_permission()
  const unlisten = await listen('error', (event: any) => {
    const error_code = event.payload.code
    switch (error_code) {
      case 1: {
        const errMsg = `${t('general.mission')} ${event.payload.data.name} ${t('error.createBackupsFailed')}`
        if (is_notify_permission_granted && is_notify_when_create_backup_failed) {
          sendNotification({ title: t('general.mission'), body: errMsg })
        }
        else {
          ElMessage.error({
            showClose: true,
            message: errMsg,
            center: true,
          })
        }
      }
        break

      default:
        break
    }
  })

  listen_dict.set('error', unlisten)
}

async function listen_to_any_success() {
  const is_notify_permission_granted = await get_notify_permission()
  const unlisten = await listen('success', (event: any) => {
    const success_code = event.payload.code
    switch (success_code) {
      case 1: {
        const success_msg = `${t('general.mission')} ${event.payload.data.name} ${t('success.createBackupsSuccess')}`
        if (is_notify_permission_granted && is_notify_when_create_backup_success) {
          sendNotification({ title: t('general.mission'), body: success_msg })
        }
        else {
          ElMessage.error({
            showClose: true,
            message: success_msg,
            center: true,
          })
        }
      }
        break

      default:
        break
    }
  })

  listen_dict.set('success', unlisten)
}

async function listen_to_close_event() {
  const unlisten = await listen('close', (event) => {
    execute_rust_command(TauriCommand.COMMAND_UPDATE_LIST_INFO, missionStore.mission_list)
    execute_rust_command(TauriCommand.COMMAND_EXIT_PROGRAM)

    listen_dict.forEach((unlisten) => {
      unlisten()
    })
  })

  listen_dict.set('close', unlisten)
}

async function listen_to_drop_event() {
  const unlisten = await appWindow.onFileDropEvent((event: any) => {
    if (event.payload.type === 'drop') {
      missionStore.update_current_drop_mission_path(event.payload.paths[0])
      router.push({ path: '/config', query: { mode: 'drop' } })
    }
    else {
      // console.log('File drop cancelled')
    }
  })

  listen_dict.set('drop', unlisten)
}

async function listen_to_system_tray_event() {
  const unlisten = await listen('hide', (event: any) => {
    execute_rust_command(TauriCommand.COMMAND_CHANGE_WINDOW_STATE_BY_SYSTEM_TRAY)
  })

  listen_dict.set('tray', unlisten)
}

async function listen_to_mission_status_update() {
  const unlisten = await listen('status_update', (event: any) => {
    const mission_id = event.payload.config.id
    const mission_status = event.payload.info.status
    missionStore.update_mission_status(mission_id, mission_status)
  })

  listen_dict.set('status_update', unlisten)
}

async function listen_to_mission_info_update() {
  const unlisten = await listen('info_update', (event: any) => {
    const mission_id = event.payload.config.id
    const mission_info = event.payload.info
    missionStore.update_mission_info(mission_id, mission_info)
  })

  listen_dict.set('info_update', unlisten)
}

async function listen_to_cron_mission_run_time_update() {
  const unlisten = await listen('cron_time_update', (event: any) => {
    const mission_id = event.payload.config.id
    const mission_info = event.payload.info
    missionStore.update_mission_info(mission_id, mission_info)
  })

  listen_dict.set('cron_time_update', unlisten)
}

async function listen_to_timing_save_data() {
  const unlisten = await listen('save_data', (_event: any) => {
    execute_rust_command(TauriCommand.COMMAND_TIMING_SAVE_DATA)
  })

  listen_dict.set('save_data', unlisten)
}

async function listen_to_another_instance() {
  const unlisten = await listen('another_instance', (_event: any) => {
    execute_rust_command(TauriCommand.COMMAND_SHOW_MAIN_WINDOW)
    ElMessage.error({
      showClose: true,
      message: t('error.anotherInstance'),
      center: true,
    })
  })

  listen_dict.set('another_instance', unlisten)
}

onMounted(() => {
  document.addEventListener('DOMContentLoaded', () => {
    execute_rust_command(TauriCommand.COMMAND_CLOSE_SPLASHSCREEN)
    execute_rust_command(TauriCommand.COMMAND_IS_PASSWORD_SET).then((res) => {
      if (!res)
        router.push({ path: '/password_setting' })
    })
  })

  if (!is_initialized.value) {
    initialize_data().then(() => {
      initialize_mission()
      initialize_system_tray()
      initialize_page_theme()
      initialize_timing_save_data()
      initialize_program_status()
    })
  }

  listen_to_another_instance()
  listen_to_close_event()
  listen_to_any_error()
  listen_to_any_success()
  listen_to_drop_event()
  listen_to_system_tray_event()
  listen_to_mission_status_update()
  listen_to_mission_info_update()
  listen_to_cron_mission_run_time_update()
  listen_to_timing_save_data()
})
</script>

<template>
  <TitleBar />
  <router-view v-slot="{ Component, route }">
    <transition :name="build_router_transitionname(route.meta.transitionName)">
      <component :is="Component" />
    </transition>
  </router-view>
</template>

<style lang="less">
@import "./assets/style/reset.css";
@import "./assets/style/RouteTransition.less";
@import "./assets/style/theme/default-vars.less";

html, body {
  // overflow: hidden;
  &::-webkit-scrollbar {
    display: none;
  }

  width: 100% !important;
  -ms-overflow-style: none;
  scrollabr-width: none;
  // scroll-behavior: smooth;
}

// .titlebar {
//   color: @primary-color;
//   background-color: @primary-bg-color;
// }
</style>
