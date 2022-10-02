<template>
  <div data-tauri-drag-region class="titlebar">
    <div class="titlebar-left" id="titlebar-left">
        <div class="titlebar-icon" id="titlebar-icon">
            <el-icon class="app-icon">
                <AppIcon />
            </el-icon>
        </div>
        <div class="titlebar-title" id="titlebar-title">
            {{t('general.AppTitle')}}
        </div>        
    </div>
    <div class="titlebar-right" id="titlebar-right">
        <div class="titlebar-button" id="titlebar-minimize">
            <el-icon><SemiSelect /></el-icon>
        </div>
        <div class="titlebar-button" id="titlebar-maximize">
            <el-icon><BorderOutlined /></el-icon>
        </div>
        <div class="titlebar-button" id="titlebar-close">
            <el-icon><CloseBold /></el-icon>
        </div>        
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { appWindow } from '@tauri-apps/api/window';
import { AppstoreOutlined, BorderOutlined } from '@vicons/antd';
import { SemiSelect, FullScreen, CloseBold } from '@element-plus/icons-vue';
import { useSettingStore, useMissionStore } from '../store/index';
import { TauriCommand, execute_rust_command } from '../utils';
import AppIcon from '~icons/icons/favicon';

const { t, locale } = useI18n({ useScope: 'global' });
const globalSetting = useSettingStore();
const missionStore = useMissionStore();

onMounted(() => {
    (document.getElementById('titlebar-minimize') as HTMLElement).addEventListener('click', () => appWindow.minimize());
    (document.getElementById('titlebar-maximize') as HTMLElement).addEventListener('click', () => appWindow.toggleMaximize());
    (document.getElementById('titlebar-close') as HTMLElement).addEventListener('click', () => {
        execute_rust_command(TauriCommand.COMMAND_UPDATE_LIST_INFO, missionStore.mission_list);
        if (globalSetting.is_close_to_tray) {
            execute_rust_command(TauriCommand.COMMAND_CLOSE_TO_TRAY);
        } else {
            execute_rust_command(TauriCommand.COMMAND_EXIT_PROGRAM);
        }
    });
})
</script>

<style lang="less" scoped>
@import "../assets/style/theme/default-vars.less";
.titlebar {
  height: 30px;
  color: var(--el-text-color-regular);
  background-color: var(--el-bg-color);
  user-select: none;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 999;

  .titlebar-left {
    display: flex;
    flex-direction: row;

    .titlebar-icon {
        width: 30px;
        height: 30px;
        padding-top: 5px;
        padding-left: 5px;

        .app-icon {
            font-size: 20px;
        }
    }

    .titlebar-title {
        padding-top: 6px;     
    }
  }

  .titlebar-right {
    display: flex;
    flex-direction: row;
    
    .titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 30px;
    height: 30px;
    }
    .titlebar-button:hover {
        background: var(--el-border-color);
    }    

    #titlebar-close:hover {
        background: red;
    }
  }
}


</style>