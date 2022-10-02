<template>
    <div class="navigateTray">
        <div
            class="navigate position_down trigger"
            @mouseenter="toggle_enter"
            @mouseleave="toggle_leave"
            v-if="!isShow"></div>
        <transition
            name="navigate_tray_transition"
            enter-active-class="animate__animated animate__slideInUp"
            leave-active-class="animate__animated animate__slideOutDown">
            <div
                class="navigate position_down tray"
                v-if="isShow" 
                @mouseenter="toggle_enter"
                @mouseleave="toggle_leave">
                    <el-button-group>
                        <el-button v-if="isShowButtons.lock" :icon="LockOutlined" @click="toggle_click('lock')" :disabled="!is_password_protected" />                   
                        <el-button v-if="isShowButtons.edit" :icon="EditOutlined" @click="toggle_click('edit')" :disabled="mission_list.length == 0 || current_mission == null"/>
                        <el-button v-if="isShowButtons.add" :icon="PlusOutlined" @click="toggle_click('add')"/>
                        <el-button v-if="isShowButtons.home" :icon="HomeOutlined" @click="toggle_click('home')"/>
                        <el-button v-if="isShowButtons.setting" :icon="SettingOutlined" @click="toggle_click('setting')"/>
                        <el-button v-if="isShowButtons.statistic" :icon="LineChartOutlined" @click="toggle_click('statistic')"/>
                    </el-button-group>
            </div>
        </transition>        
    </div>
</template>
<script setup lang="ts">
import type { Component } from 'vue';
import { ref } from 'vue';
import router from '../router';
import {
    LockOutlined,
    PlusOutlined,
    EditOutlined,
    SettingOutlined,
    LineChartOutlined,
    HomeOutlined
} from '@vicons/antd';
import { storeToRefs } from 'pinia';
import { useSettingStore, useMissionStore } from '../store/index';

const globalSetting = useSettingStore();
const missionStore = useMissionStore();
const { is_password_protected } = storeToRefs(globalSetting);
const { mission_list, current_mission } = storeToRefs(missionStore);

const props = defineProps({
    fns: { type: Array, required: true },
})

const emit = defineEmits(['toggleLock'])

const isShow = ref(false);

const isShowButtons = ref({
    lock: props.fns === undefined? false : props.fns.includes('lock'),
    edit: props.fns === undefined? false : props.fns.includes('edit'),
    add: props.fns === undefined? false : props.fns.includes('add'),
    home: props.fns === undefined? false : props.fns.includes('home'),
    setting: props.fns === undefined? false : props.fns.includes('setting'),
    statistic: props.fns === undefined? false : props.fns.includes('statistic')
})

const toggle_enter = () => {
    isShow.value = true;
}

const toggle_leave = () => {
    isShow.value = false;
}

const jump_to = (to: string, params?: string) => {
    if (isShow) {
        if (params) {
            router.push({ path: to, query: { mode: params} })       
        } else {
            router.push({ path: to })
        }          
    }
}

const toggle_click = (button: string) => {
    switch(button) {
        case 'lock': {
            emit('toggleLock', true);
        } break;
        case 'edit': {
            jump_to('/config', 'edit');
        } break;
        case 'add': {
            jump_to('/config', 'add');
        } break;
        case 'home': {
            jump_to('/');
        } break;
        case 'setting': {
            jump_to('/setting');
        } break;
        case 'statistic': {
            jump_to('/statistic');
        } break;
    }
}



</script>
<style lang="less" scoped>
@import "../assets/style/theme/default-vars.less";
.navigate {
    display: flex;
    margin: auto;
    justify-content: center;
    align-items: center;
    z-index: 5;
    color: var(--el-color-primary);
}

.trigger {
    z-index: 10;
    opacity: 0;
}

.position_down {
    position: fixed;
    bottom: 0;
    width: 100%;
    min-height: 8%;
}    

.tray {
    display: flex;
    flex-direction: row;
    flex-wrap: nowrap;
    z-index: 20;
    background-color: transparent;

    :deep(.el-button-group .el-button) {
        background-color: transparent;
    }
}
</style>