<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { computed, onMounted } from 'vue'
import { storeToRefs } from 'pinia'
import uiCN from 'element-plus/es/locale/lang/zh-cn'
import uiEN from 'element-plus/es/locale/lang/en'
import type { HandlerStatus } from './store/status/types'
import { Command, execute } from './utils/cmd'
import TitleBar from './components/TitleBar.vue'
import SideMenu from './components/SideMenu.vue'
import { useNotifyStore, useScreensaverStore, useStatusStore, useSystemStore, useWatcherStore } from './store'
import type { AppConfig } from './store/types'
import { listenToEvents } from './utils/event'

/**
 * Represents the status store instance.
 */
const statusStore = useStatusStore()

/**
 * Represents the system store instance.
 */
const systemStore = useSystemStore()

/**
 * Represents the watcher store instance.
 */
const watcherStore = useWatcherStore()

/**
 * Represents the screensaver store instance.
 */
const screensaverStore = useScreensaverStore()

/**
 * Represents the notify store instance.
 */
const notifyStore = useNotifyStore()

/**
 * Represents the language value from the system store.
 */
const { language } = storeToRefs(systemStore)

/**
 * Represents the UI locale based on the selected language.
 */
const uiLocale = computed(() => {
  return language.value === 'zh-CN' ? uiCN : uiEN
})

/**
 * Initializes the handler by executing the init command and setting the status.
 */
async function initApp() {
  await execute(Command.InitApp)
    .then((res: HandlerStatus) => {
      statusStore.setHandlerStatus(res)
    })
}

/**
 * Initializes the configuration store by executing the init command with 'all' parameter.
 * Initializes various stores with the received configuration.
 */
async function initConfigStore() {
  await execute(Command.InitConfig, 'all')
    .then(async (config: AppConfig) => {
      await systemStore.init(config.system)
      await watcherStore.init(config.watcher)
      await screensaverStore.init(config.screensaver)
      await notifyStore.init(config.notify)

      await notifyStore.tryGetPermission()
    })
}

onMounted(() => {
  document.addEventListener('DOMContentLoaded', async () => {
    await initApp()
    await initConfigStore()
    await listenToEvents()
  })
})
</script>

<template>
  <div class="main">
    <el-config-provider :locale="uiLocale">
      <el-container>
        <el-header class="container__header">
          <TitleBar />
        </el-header>
        <el-container>
          <el-aside class="container__aside">
            <SideMenu />
          </el-aside>
          <el-main class="container__main">
            <!-- <router-view /> -->
            <router-view v-slot="{ Component, route }">
              <transition :name="(route.meta.transition as string)">
                <component :is="Component" :key="route.path" />
              </transition>
            </router-view>
          </el-main>
        </el-container>
      </el-container>
    </el-config-provider>
  </div>
</template>

<style lang="less">
@import "./assets/style/reset.css";

html, body {
  &::-webkit-scrollbar {
    display: none;
    -ms-overflow-style: none;
  }
}

.container__header {
  width: 100%;
  height: 25px;
  padding: 0;
}

.container__aside {
  width: 140px;
  height: calc(100vh - 25px);
}

.container__aside::-webkit-scrollbar {
  width: 0;  /* Remove scrollbar space */
  background: transparent;  /* Optional: just make scrollbar invisible */
}

.container__main {
  width: 100%;
  height: calc(100vh - 25px);
  padding: 0;
  overflow-x: hidden;
}
</style>
