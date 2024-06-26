<script setup lang="ts">
function generateError() {
  throw new Error('test error from Mission page')
}
</script>

<template>
  <div class="container">
    {{ "This is the mission view" }}
    <el-button @click="generateError">
      {{ "Error" }}
    </el-button>
  </div>
</template>

<style scoped>

</style>
