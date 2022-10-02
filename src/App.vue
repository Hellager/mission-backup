<template>
  <TitleBar />
  <router-view v-slot="{ Component, route }">
    <transition :name="build_router_transitionname(route.meta.transitionName)">
        <component :is="Component" />      
    </transition>
  </router-view>
</template>

<script setup lang="ts">
import { watch, onMounted } from 'vue';
import router from './router';
import { storeToRefs } from 'pinia';
import { useI18n } from 'vue-i18n';
import { useSettingStore, useMissionStore } from './store/index';
import { TauriCommand, execute_rust_command } from './utils/index';
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { appWindow } from '@tauri-apps/api/window';
import TitleBar from './components/TitleBar.vue';

const { t, locale } = useI18n({ useScope: 'global' });
const globalSetting = useSettingStore();
const missionStore = useMissionStore();
const { is_initialized, language } = storeToRefs(globalSetting);

/**
 * initialize data when mounted
 */
async function initialize_data() {
    await execute_rust_command(TauriCommand.COMMAND_INITIALIZE_DATA)
            .then((data) => { 
            globalSetting.initialize_settings(data.setting); 
            missionStore.initialize_mission_list(data.list);
            locale.value = data.setting.language;
            is_initialized.value = true; 
          })    
}

async function initialize_program_status() {
  await execute_rust_command(TauriCommand.COMMAND_INITIALIZE_PROGRAM_STATUS);
}

async function initialize_mission() {
  let is_initialized = await execute_rust_command(TauriCommand.COMMAND_IS_INITIALIZED);
  if (is_initialized) return;

  let mission_list = missionStore.mission_list;
  const error_list: any = [];
  for (const mission of mission_list) {
    if (mission.info.status != "pausing") {
      await execute_rust_command(TauriCommand.COMMAND_START_MISSION, mission.config.id)
              .then((res) => {
                if (!res.data) {
                  let errMsg = mission.config.name + ": " + (res.msg.indexOf('path') != -1 ? t('error.unavailablePath') : t('error.startMissionFailed'));
                  error_list.push({
                    showClose: true,
                    message: errMsg,
                    center: true,
                  })

                  missionStore.update_mission_status(mission.config.id, "unavailable");
                }
              })      
    }    
  }

  error_list.forEach((element: any) => {
      ElMessage.error(element);            
  })
}

async function initialize_system_tray() {
  let current_lang = language.value;
  await execute_rust_command(TauriCommand.COMMAND_CHANGE_SETTING_LANGUAGE, current_lang);
}

async function initialize_page_theme() {
  let current_theme = globalSetting.is_light_theme;
  const res = await execute_rust_command(TauriCommand.COMMAND_CHANGE_SETTING_IS_LIGHT_THEME, current_theme);
  if (res) {
      globalSetting.update_theme(current_theme);
      const root_element = document.documentElement;
      root_element.setAttribute("class", current_theme ? "" : "dark");
  } else {
      globalSetting.update_theme(!current_theme);    
  }    
}

async function initialize_timing_save_data() {
  let is_initialized = await execute_rust_command(TauriCommand.COMMAND_IS_INITIALIZED);
  if (is_initialized) return;
  await execute_rust_command(TauriCommand.COMMAND_START_TIMING_SAVE_DATA);
}

const build_router_transitionname = (name: any): string => {
  return (name === undefined ? "" : name);
}

/**
 * listen events
 */
const listen_dict: Map<string, UnlistenFn> = new Map();

async function listen_to_any_error() {
  const unlisten = await listen('error', (event: any) => {
    let error_code = event.payload.code;
    switch (error_code) {
      case 1:
        // create backups failed error
        let errMsg = event.payload.data.name + ": " + t('error.createBackupsFailed');
        ElMessage.error({
          showClose: true,
          message: errMsg,
          center: true,
        })
        break;
    
      default:
        break;
    }
  })
}

async function listen_to_close_event() {
  const unlisten = await listen('close', (event) => {
    execute_rust_command(TauriCommand.COMMAND_UPDATE_LIST_INFO, missionStore.mission_list);
    execute_rust_command(TauriCommand.COMMAND_EXIT_PROGRAM);

    listen_dict.forEach(unlisten => {
      unlisten();
    })
  })

  listen_dict.set('close', unlisten);
}

async function listen_to_drop_event() {
  const unlisten = await appWindow.onFileDropEvent((event: any) => {
    if (event.payload.type === 'drop') {
      missionStore.update_current_drop_mission_path(event.payload.paths[0]);
      router.push({ path: '/config', query: { mode: 'drop'} });
    } else {
      console.log('File drop cancelled');
    }
  })

  listen_dict.set('drop', unlisten);
}

async function listen_to_system_tray_event() {
  const unlisten = await listen('hide', (event: any) => {
    execute_rust_command(TauriCommand.COMMAND_CHANGE_WINDOW_STATE_BY_SYSTEM_TRAY);
  })

  listen_dict.set('tray', unlisten);  
}

async function listen_to_mission_status_update() {
  const unlisten = await listen('status_update', (event: any) => {
    let mission_id = event.payload.config.id;
    let mission_status = event.payload.info.status;
    missionStore.update_mission_status(mission_id, mission_status);
  })

  listen_dict.set('status_update', unlisten);
}

async function listen_to_mission_info_update() {
  const unlisten = await listen('info_update', (event: any) => {
    let mission_id = event.payload.config.id;
    let mission_info = event.payload.info;
    missionStore.update_mission_info(mission_id, mission_info);
  })

  listen_dict.set('info_update', unlisten);
}

async function listen_to_cron_mission_run_time_update() {
  const unlisten = await listen('cron_time_update', (event: any) => {
    let mission_id = event.payload.config.id;
    let mission_info = event.payload.info;
    missionStore.update_mission_info(mission_id, mission_info);    
  })
}

async function listen_to_timing_save_data() {
  const unlisten = await listen('save_data', (event: any) => {
    execute_rust_command(TauriCommand.COMMAND_TIMING_SAVE_DATA);
  })
}


onMounted(() => {
  document.addEventListener('DOMContentLoaded', () => {
    execute_rust_command(TauriCommand.COMMAND_CLOSE_SPLASHSCREEN);
  });

  if (!is_initialized.value) {
    initialize_data().then(() => {
      initialize_mission();
      initialize_system_tray();
      initialize_page_theme();
      initialize_timing_save_data();
      initialize_program_status();
    });
  }
  
  listen_to_close_event();
  listen_to_any_error();
  listen_to_drop_event();
  listen_to_system_tray_event();
  listen_to_mission_status_update();
  listen_to_mission_info_update();
  listen_to_cron_mission_run_time_update();
  listen_to_timing_save_data();
})
</script>

<style lang="less">
@import "./assets/style/reset.css";
@import "./assets/style/RouteTransition.less";
@import "./assets/style/theme/default-vars.less";

html, body {
  // overflow: hidden;
  &::-webkit-scrollbar {
    display: none;
  }

  -ms-overflow-style: none;
  scrollabr-width: none;
  // scroll-behavior: smooth;
}

// .titlebar {
//   color: @primary-color;
//   background-color: @primary-bg-color;
// }
</style>
