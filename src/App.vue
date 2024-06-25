<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { onMounted } from 'vue'
import { Command, execute } from './utils/cmd'
import SideMenu from './components/SideMenu.vue'

/**
 * Initializes the handler by executing the init command and setting the status.
 */
async function initApp() {
  await execute(Command.InitApp)
}

onMounted(() => {
  document.addEventListener('DOMContentLoaded', async () => {
    await initApp()
  })
})
</script>

<template>
  <div class="main">
    <el-config-provider>
      <el-container>
        <el-container>
          <el-aside class="container__aside">
            <SideMenu />
          </el-aside>
          <el-main class="container__main">
            <!-- <router-view /> -->
            <router-view v-slot="{ Component, route }">
              <transition :name="(route.meta.transition as string)">
                <component :is="Component" :key="route.path" />
              </transition>
            </router-view>
          </el-main>
        </el-container>
      </el-container>
    </el-config-provider>
  </div>
</template>

<style lang="less">
@import "./assets/style/reset.css";

html, body {
  &::-webkit-scrollbar {
    display: none;
    -ms-overflow-style: none;
  }
}

.container__aside {
  width: 140px;
}

.container__aside::-webkit-scrollbar {
  width: 0;  /* Remove scrollbar space */
  background: transparent;  /* Optional: just make scrollbar invisible */
}

.container__main {
  width: 100%;
  padding: 0;
  overflow-x: hidden;
}
</style>
