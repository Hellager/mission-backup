<template>
    <LockCover :isLocked="pageLock" @validate="handle_validate" />
    <NavigateTray :fns="props.tray"  @toggleLock="toggle_page_lock"/>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import LockCover from '../components/LockCover.vue';
import NavigateTray from '../components/NavigateTray.vue';

const props = defineProps({
    tray: { type: Array, required: true },
})

const pageLock = ref(false);

const handle_validate = (res: boolean) => {
    if (res) {
        pageLock.value = false; 
    }
}

const toggle_page_lock = () => {
    if (!pageLock.value) {
       pageLock.value = true; 
    }
}
</script>
<style lang="less" scoped>
@import "../assets/style/theme/default-vars.less";
</style>