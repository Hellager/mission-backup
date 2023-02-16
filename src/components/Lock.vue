<script setup lang="ts">
import { ref, watch } from 'vue'
import LockCover from '../components/LockCover.vue'
import NavigateTray from '../components/NavigateTray.vue'

const props = defineProps({
  tray: { type: Array, required: true },
  defaultLock: { type: Boolean },
})

const emit = defineEmits<{
  (event: 'changeDefaultLock', res: boolean): void
}>()

const pageLock = ref(false)

const handle_validate = (res: boolean) => {
  if (res)
    pageLock.value = false
  emit('changeDefaultLock', false)
}

const toggle_page_lock = () => {
  if (!pageLock.value)
    pageLock.value = true
}
</script>

<template>
  <LockCover :is-locked="pageLock || props.defaultLock" @validate="handle_validate" />
  <NavigateTray :fns="props.tray" @toggle-lock="toggle_page_lock" />
</template>

<style lang="less" scoped>
@import "../assets/style/theme/default-vars.less";
</style>
