<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import LockCover from '../components/LockCover.vue'

const pageLock = ref(true)
const setMode = ref(true)

const router = useRouter()

const handle_validate = (res: boolean) => {
  pageLock.value = false
  console.log('should set password')
  router.push({ path: '/' })
}
</script>

<template>
  <LockCover :is-locked="pageLock" :is-set-mode="setMode" @validate="handle_validate" />
</template>

<style lang="less" scoped>
@import "../assets/style/theme/default-vars.less";
</style>
