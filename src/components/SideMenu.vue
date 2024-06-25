<script setup lang="ts">
import { ref } from 'vue'

/**
 * Interface with optional properties `key` (string) and `name` (string) for menu items.
 *
 * `key` -> router path
 *
 * `name` -> router name
 */
interface ASideMenuItems {
  key?: string
  name?: string
}

/**
 * Indicates whether the application is in debug mode.
 */
const debugmode = ref(import.meta.env.DEV)

/**
 * Contains key-value pairs for menu items.
 */
const menuitems: ASideMenuItems[] = [
  {
    key: '/mission',
    name: 'mission',
  },
  {
    key: '/procedure',
    name: 'procedure',
  },
  {
    key: '/backup',
    name: 'backup',
  },
  {
    key: '/statistic',
    name: 'statistic',
  },
  {
    key: '/config',
    name: 'config',
  },
]
</script>

<template>
  <div class="aside">
    <el-menu
      default-active="/mission"
      class="aside__menu"
      router
    >
      <el-menu-item v-for="item in menuitems" :key="item.key" :index="item.key">
        <span> {{ item.name }}</span>
      </el-menu-item>
    </el-menu>
  </div>
</template>

<style scoped lang="less">
.aside__menu {
  :deep(.el-menu-item) {
    justify-content: center;
  }

  // overflow-y: hidden;
}
</style>
