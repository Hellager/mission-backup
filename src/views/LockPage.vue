<script setup lang="ts">
import NavigateSideBar from '../components/NavigateSideBar.vue'
</script>

<template>
  <div class="container">
    <div class="title">
      This is the Lock Page
    </div>
    <!-- <div class="action">
            <el-button type="primary" round @click="jumpTo('/table')">
                to Table
            </el-button>
        </div>  -->

    <NavigateSideBar position="right" to-path="/table" />
  </div>
</template>

<style lang="less" scoped>
.container {
    width: 100%;
    min-height: 100vh;
    background-color: #e6fffb;
}

.title {
    padding-top: 20px;
    text-align: center;
}

.action {
    margin-top: 20px;
    text-align: center;
}
</style>
