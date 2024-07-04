<script setup lang="ts">
import { ref } from 'vue'
import { appWindow } from '@tauri-apps/api/window'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import {
  Close,
  Locked,
  Logo,
  Maxmize,
  Minimize,
} from '../assets/icons'
import { useScreensaverStore, useSystemStore } from '../store'
import CloseDialog from './CloseDialog.vue'

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 * Used for routing within the application.
 */
const router = useRouter()

/**
 * Custom stores for system settings.
 */
const systemStore = useSystemStore()

/**
 * Custom stores for managing screensaver.
 */
const store = useScreensaverStore()
const { isLocked } = storeToRefs(store)

/**
 * Indicates whether the close dialog is visible.
 */
const showCloseDialog = ref<boolean>(false)

/**
 * Handles the lock action of the application.
 */
async function onLockClicked() {
  if (!isLocked.value) {
    router.push('/screensaver')
    store.updateLockStatus(true)
  }
}

/**
 * Minimizes the application window.
 */
function onActionMinimizeClicked() {
  appWindow.minimize()
}

/**
 * Toggles the application window between maximize and restore.
 */
function onActionMaxmizeClicked() {
  appWindow.toggleMaximize()
}

/**
 * Handles the close action of the application window.
 */
async function onActionCloseClicked() {
  if (systemStore.closeConfirm())
    showCloseDialog.value = !showCloseDialog.value
  else
    systemStore.tryClose(undefined, undefined)
}
</script>

<template>
  <div
    class="bar"
    data-tauri-drag-region
  >
    <div class="bar__title">
      <el-icon class="bar__title__icon">
        <Logo />
      </el-icon>

      <el-text class="bar__title__text">
        {{ t("common.appName") }}
      </el-text>
    </div>

    <div class="bar__action">
      <div class="bar__action__lock" @click="onLockClicked">
        <transition
          name="bar__action__lock__transition"
          enter-active-class="animate__animated animate__fadeIn"
          leave-active-class="animate__animated animate__fadeOut"
        >
          <el-icon v-if="!isLocked" class="bar__action__lock__icon">
            <Locked />
          </el-icon>
        </transition>
      </div>

      <el-divider direction="vertical" />

      <div
        class="bar__action__minimize"
        @click="onActionMinimizeClicked"
      >
        <el-icon>
          <Minimize />
        </el-icon>
      </div>

      <div
        class="bar__action__maxmize"
        @click="onActionMaxmizeClicked"
      >
        <el-icon>
          <Maxmize />
        </el-icon>
      </div>

      <div
        class="bar__action__close"
        @click="onActionCloseClicked"
      >
        <el-icon>
          <Close />
        </el-icon>
      </div>
    </div>
  </div>

  <CloseDialog
    :visiable="showCloseDialog"
    @hide="showCloseDialog = false"
  />
</template>

<style scoped lang="less">
.bar {
    height: 25px;
    z-index: 1001;
    background-color: var(--el-bg-color-page);
    display: flex;
    width: 100%;
    user-select: none;
    flex-direction: row;
    flex-wrap:nowrap;
    align-items: center;
    align-content: center;
    justify-content: space-between;
}

.bar__title {
    display: flex;
}

.bar__title__icon {
  margin-left: 3px;
}

.bar__title__text {
  margin-left: 3px;
}

.bar__action {
  display: flex;
  flex-direction: row;
  align-items: center;
}

.bar__action__minimize {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 25px;
  height: 25px;
}

.bar__action__minimize:hover {
  background-color: #d9d9d9;
}

.bar__action__maxmize {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 25px;
  height: 25px;
}

.bar__action__maxmize:hover {
  background-color: #d9d9d9;
}

.bar__action__close {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 25px;
  height: 25px;
}

.bar__action__close:hover {
  background-color: #ff4d4f;
}
</style>
