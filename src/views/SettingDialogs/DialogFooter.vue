<script lang="ts" setup>
import { useI18n } from 'vue-i18n'

const emit = defineEmits(['cancel', 'confirm'])

const { t, locale } = useI18n({ useScope: 'global' })

const toggle_cancel_click = () => {
  emit('cancel')
}

const toggle_confirm_click = () => {
  emit('confirm')
}
</script>

<template>
  <div>
    <span class="dialog-footer">
      <el-button @click="toggle_cancel_click">{{ t('general.cancel') }}</el-button>
      <el-button type="primary" @click="toggle_confirm_click">{{ t('general.confirm') }}</el-button>
    </span>
  </div>
</template>

<style lang="less">
</style>
