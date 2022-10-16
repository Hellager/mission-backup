<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { LeftOutlined } from '@vicons/antd'
import router from '../router'

const props = defineProps({
  title: { type: String, required: true },
  to: { type: String, required: true },
})

const { t, locale } = useI18n({
  useScope: 'global',
  inheritLocale: true,
})

const toggle_back = () => {
  if (props.to === undefined)
    router.push({ path: '/' })
  else
    router.push({ path: props.to })
}
</script>

<template>
  <div class="page-header">
    <el-page-header :icon="LeftOutlined" :title="t('component.pageHeaderBack')" :content="props.title" @back="toggle_back" />
  </div>
</template>

<style lang="less" scoped>
@import "../assets/style/theme/default-vars.less";
.page-header {
    width: 100%;
    position: absolute;
    display: block;
    z-index: 50;
    margin-top: -16px;
    top: @title-bar-height;
    color: var(--el-text-color-regular);
    background-color: var(--el-bg-color);

    :deep(.el-page-header__content) {
        color: var(--el-text-color-regular);
    }
}
</style>
