<script lang="ts" setup>
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { DialogMode } from '../../types'
import CrontabTool from './components/CrontabTool.vue'
import DatabaseTool from './components/DatabaseTool.vue'
import LogTool from './components/LogTool.vue'
import MigrateTool from './components/MigrateTool.vue'

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

      <el-col :span="12">
        <el-card shadow="hover" @click="handleShowDialogAction(1)">
          {{ t('toolbox.database.title') }}
        </el-card>
      </el-col>
    </el-row>

    <el-row>
      <el-col :span="12">
        <el-card shadow="hover" @click="handleShowDialogAction(2)">
          {{ t('toolbox.log.title') }}
        </el-card>
      </el-col>
      <el-col :span="12">
        <el-card shadow="hover" @click="handleShowDialogAction(3)">
          {{ t('toolbox.migrate.title') }}
        </el-card>
      </el-col>
    </el-row>

    <CrontabTool :visiable="showDialogs[0]" :mode="DialogMode.Create" expression="0 0/30 * * * *" @hide="handleDialogAction(0)" />
    <DatabaseTool :visiable="showDialogs[1]" @hide="handleDialogAction(1)" />
    <LogTool :visiable="showDialogs[2]" @hide="handleDialogAction(2)" />
    <MigrateTool :visiable="showDialogs[3]" @hide="handleDialogAction(3)" />
  </div>
</template>

<style lang="less" scoped>

</style>
