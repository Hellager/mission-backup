<script setup lang="ts">
import { ref, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { useI18n } from 'vue-i18n'
import { useSystemStore } from '../../../store'

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 * Initializes the system store and sets up necessary variables and functions for managing notifications.
 */
const store = useSystemStore()
const {
  theme,
  themeOption,
  autoStart,
  closeOption,
  language,
} = storeToRefs(store)

/**
 * Indicates the theme option in type of boolean
 */
const themeOptionBool = ref<boolean>(false)
watch(themeOption, () => {
  themeOptionBool.value = !!themeOption.value
})

/**
 * Updates the language setting.
 *
 * @param value - The new language value
 */
async function onLanguageUpdate(value: string) {
  store.updateLanguage(value)
}

/**
 * Updates the auto start setting.
 *
 * @param value - The new auto start value
 */
async function onAutoStartUpdate(value: boolean) {
  store.updateAutoStart(value)
}

/**
 * Updates the theme setting.
 *
 * @param value - The new theme value
 */
async function onThemeUpdate(value: string) {
  store.updateTheme(value)
}

/**
 * Updates the theme option.
 *
 * @param value - The new theme option
 */
async function onThemeOptionUpdate(value: boolean) {
  store.updateThemeOption(value ? 1 : 0)
}

/**
 * Updates the close option setting.
 *
 * @param value - The new close option value
 */
async function onCloseOptionUpdate(value: number) {
  store.updateCloseOption(value)
}
</script>

<template>
  <div class="config">
    <el-form class="config__form">
      <el-form-item :label="t('config.system.theme')">
        <el-select v-model="theme" :disabled="themeOptionBool" @change="onThemeUpdate">
          <el-option :label="t('config.system.themeLight')" value="light" />
          <el-option :label="t('config.system.themeDark')" value="dark" />
        </el-select>
      </el-form-item>

      <el-form-item :label="t('config.system.themeOption')">
        <el-switch v-model="themeOptionBool" @change="onThemeOptionUpdate" />
      </el-form-item>

      <el-form-item :label="t('config.system.language')">
        <el-select v-model="language" @change="onLanguageUpdate">
          <el-option :label="t('config.system.langCN')" value="zh-CN" />
          <el-option :label="t('config.system.langEN')" value="en-US" />
        </el-select>
      </el-form-item>

      <el-form-item :label="t('config.system.autoStart')">
        <el-switch v-model="autoStart" @change="onAutoStartUpdate" />
      </el-form-item>

      <el-form-item :label="t('config.system.closeOption')">
        <el-select v-model="closeOption" @change="onCloseOptionUpdate">
          <el-option :label="t('config.system.closeExit')" :value="0" />
          <el-option :label="t('config.system.closeTray')" :value="1" />
        </el-select>
      </el-form-item>
    </el-form>
  </div>
</template>

<style scoped lang="less">
.config__form {
  :deep(.el-form-item__label) {
    width: 110px;
  }

  :deep(.el-select){
    width: 130px;
  }
}
</style>
