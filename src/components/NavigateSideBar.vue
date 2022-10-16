<script setup lang="ts">
import type { Component } from 'vue'
import { ref } from 'vue'
import {
  DownOutlined,
  EditOutlined,
  LeftOutlined,
  LockOutlined,
  PlusOutlined,
  RightOutlined,
  UpOutlined,
} from '@vicons/antd'
import router from '../router'

const props = defineProps({
  position: { type: String, required: true },
  toPath: { type: String, required: true },
  isSpecial: { type: String },
})

const isShow = ref(false)

const position = `arrow_${props.position}`
const animate_positon: string = props.position === undefined ? 'left' : props.position
const toPath: string = props.toPath === undefined ? '/' : props.toPath

const class_navigate = 'navigate'
const class_trigger = 'trigger'

let enter_transition = `animate__animated animate__slideIn${animate_positon.replace(/^\S/, s => s.toUpperCase())}`
let leave_transition = `animate__animated animate__slideOut${animate_positon.replace(/^\S/, s => s.toUpperCase())}`
if (animate_positon === 'up' || animate_positon === 'down') {
  enter_transition = `animate__animated animate__slideIn${animate_positon === 'up' ? 'Down' : 'Up'}`
  leave_transition = `animate__animated animate__slideOut${animate_positon === 'up' ? 'Up' : 'Down'}`
}

const jump_to = () => {
  if (isShow.value)
    router.push({ path: toPath })
}

const toggle_click = () => {
  if (props.isSpecial !== 'password')
    jump_to()
}

const toggle_enter = () => {
  isShow.value = true
}

const toggle_leave = () => {
  isShow.value = false
}
</script>

<template>
  <div class="navigateBar">
    <div
      :class="[class_navigate, position, class_trigger]"
      @mouseenter="toggle_enter"
      @mouseleave="toggle_leave"
      @click="toggle_click"
    />
    <transition
      name="navigate_bar_transition"
      :enter-active-class="enter_transition"
      :leave-active-class="leave_transition"
    >
      <div v-if="isShow" :class="[class_navigate, position]">
        <el-icon :size="30">
          <LockOutlined v-if="props.isSpecial === 'password'" />
          <PlusOutlined v-else-if="props.isSpecial === 'add'" />
          <EditOutlined v-else-if="props.isSpecial === 'edit'" />
          <LeftOutlined v-else-if="props.position === 'left'" />
          <RightOutlined v-else-if="props.position === 'right'" />
          <DownOutlined v-else-if="props.position === 'up'" />
          <UpOutlined v-else />
          <!-- <component :is="props.icon" /> -->
        </el-icon>
      </div>
    </transition>
  </div>
</template>

<style lang="less" scoped>
.navigate {
    display: flex;
    margin: auto;
    justify-content: center;
    align-items: center;
    background-color: rgba(245, 245, 245, 0.3);
    z-index: 5;
}

.trigger {
    z-index: 10;
    opacity: 0;
}

.arrow_left {
    position: absolute;
    left: 0;
    width: 5%;
    height: 100%;
    top: 0;
}

.arrow_right {
    position: absolute;
    right: 0;
    width: 5%;
    height: 100%;
    top: 0;
}

.arrow_up {
    position: absolute;
    top: 0;
    width: 100%;
    min-height: 5%;
}

.arrow_down {
    position: absolute;
    bottom: 0;
    width: 100%;
    min-height: 5%;
}
</style>
