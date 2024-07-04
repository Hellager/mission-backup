<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import SystemConfig from './components/SystemConfig.vue'
import WatcherConfig from './components/WatcherConfig.vue'
import NotifyConfig from './components/NotifyConfig.vue'
import ScreensaverConfig from './components/ScreensaverConfig.vue'

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 * Indicates the selected tab
 */
const activeName = ref('system')
</script>

<template>
  <el-tabs v-model="activeName" class="tabs">
    <el-tab-pane :label="t('config.general.system')" name="system">
      <SystemConfig />
    </el-tab-pane>
    <el-tab-pane :label="t('config.general.notify')" name="notify">
      <NotifyConfig />
    </el-tab-pane>
    <el-tab-pane :label="t('config.general.screensaver')" name="screensaver">
      <ScreensaverConfig />
    </el-tab-pane>
    <el-tab-pane :label="t('config.general.watcher')" name="watcher">
      <WatcherConfig />
    </el-tab-pane>
  </el-tabs>
</template>

<style scoped lang="less">
.tabs {
    margin-left: 10px;
    margin-right: 10px;
}
</style>
