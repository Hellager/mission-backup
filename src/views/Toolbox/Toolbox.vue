<script lang="ts" setup>
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { DialogMode } from '../../types'
import CronTools from './components/CrontabTool.vue'

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

const showDialogs = ref<boolean[]>(Array.from({ length: 4 }, () => false))

function handleShowDialogAction(toolIdx: number) {
  showDialogs.value[toolIdx] = true
}

async function handleDialogAction(toolIdx: number) {
  showDialogs.value[toolIdx] = false
}
</script>

<template>
  <div class="toolbox">
    <el-row>
      <el-col :span="12">
        <el-card shadow="hover" @click="handleShowDialogAction(0)">
          {{ t('toolbox.crontab.title') }}
        </el-card>
      </el-col>
    </el-row>
    <CronTools :visiable="showDialogs[0]" :mode="DialogMode.Create" expression="0 0/30 * * * *" @hide="handleDialogAction(0)" />
  </div>
</template>

<style lang="less" scoped>

</style>
