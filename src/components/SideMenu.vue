<script setup lang="ts">
import { useI18n } from 'vue-i18n'
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
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

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
  {
    key: '/toolbox',
    name: 'toolbox',
  },
]
</script>

<template>
  <div class="aside">
    <el-menu
      default-active="/config"
      class="aside__menu"
      router
    >
      <el-menu-item v-for="item in menuitems" :key="item.key" :index="item.key">
        <span> {{ t(`sidemenu.${item.name}`) }}</span>
      </el-menu-item>
    </el-menu>
  </div>
</template>

<style scoped lang="less">
.aside__menu {
  :deep(.el-menu-item) {
    justify-content: center;
  }

  height: calc(100vh - 25px);
}
</style>
