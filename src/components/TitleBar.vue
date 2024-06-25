<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window'
import {
  Close,
  Logo,
  Maxmize,
  Minimize,
} from '../assets/icons'

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
  await appWindow.close()
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
        {{ "Mission Backup" }}
      </el-text>
    </div>

    <div class="bar__action">
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
