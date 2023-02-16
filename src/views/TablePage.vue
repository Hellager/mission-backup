<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { Clock, Delete, Edit, Loading, VideoPause, VideoPlay, View, Warning } from '@element-plus/icons-vue'
import { storeToRefs } from 'pinia'
import router from '../router'
import Lock from '../components/Lock.vue'
import { useMissionStore, useSettingStore } from '../store/index'
import { TauriCommand, execute_rust_command } from '../utils'
// import { ElMessage } from 'element-plus';

const default_lock = ref(false)

const route = useRoute()

const { t, locale } = useI18n({
  useScope: 'global',
  inheritLocale: true,
})

const missionStore = useMissionStore()
const settingStore = useSettingStore()
const { is_page_initialized } = storeToRefs(settingStore)
const { mission_list, current_mission } = storeToRefs(missionStore)

const indexMethod = (index: number) => {
  return (index + 1)
}

onMounted(() => {
  current_mission.value = mission_list.value.length > 0 ? undefined : mission_list.value[0]

  if (is_page_initialized.value === false) {
    default_lock.value = true
    settingStore.update_page_initialized_status(true)
  }
  else {
    default_lock.value = false
  }

  if (route.query.mode === 'after_set')
    default_lock.value = false
})

const toggle_row_click = (row: any) => {
  current_mission.value = row
}

const toggle_row_db_click = (row: any) => {
  current_mission.value = row
  router.push({ path: '/config', query: { mode: 'edit' } })
}

const toggle_change_default_lock = (res: boolean) => {
  default_lock.value = res
}

async function toggle_edit_click(id: string) {
  missionStore.update_current_mission(id)
  router.push({ path: '/config', query: { mode: 'edit' } })
}

async function toggle_stop_click(id: string, status: string) {
  if (status !== 'pausing') {
    await execute_rust_command(TauriCommand.COMMAND_STOP_MISSION, id)
      .then((res) => {
        if (res.data) {
          missionStore.stop_mission(id)
        }
        else {
          ElMessage.error({
            showClose: true,
            message: t('error.stopMissionFailed'),
            center: true,
          })
        }
      })
  }
  else {
    await execute_rust_command(TauriCommand.COMMAND_START_MISSION, id)
      .then((res) => {
        if (res.data) {
          missionStore.start_mission(id)
        }
        else {
          ElMessage.error({
            showClose: true,
            message: t('error.startMissionFailed'),
            center: true,
          })
        }
      })
  }
}

async function toggle_delete_dlick(id: string) {
  const current_mission_status = missionStore.get_mission_status(id)
  let delete_command = TauriCommand.COMMAND_DELETE_MISSION
  if (current_mission_status === 'unavailable')
    delete_command = TauriCommand.COMMAND_FORCE_DELETE_MISSION

  await execute_rust_command(delete_command, id)
    .then((res) => {
      if (res.data) {
        missionStore.delete_mission(id)
      }
      else {
        ElMessage.error({
          showClose: true,
          message: t('error.deleteMissionFailed'),
          center: true,
        })
      }
    })
}
</script>

<template>
  <div class="container">
    <div class="content">
      <el-table
        :data="mission_list"
        stripe fit
        style="width: 100%"
        height="100vh"
        table-layout="fixed"
        @row-click="toggle_row_click"
        @row-dblclick="toggle_row_db_click"
      >
        <el-table-column type="index" :index="indexMethod" :label="t('table.index')" :width="60" style="text-align: center" />
        <el-table-column :label="t('table.name')">
          <template #default="props">
            <el-tooltip placement="top" effect="light" :show-after="1000">
              <template #content>
                {{ props.row.config.name }}
              </template>
              <div class="missionName">
                {{ props.row.config.name }}
              </div>
            </el-tooltip>
          </template>
        </el-table-column>
        <el-table-column :label="t('table.status')">
          <template #default="props">
            <div class="statusContainer">
              <el-icon v-if="props.row.info.status === 'executing'" class="loading">
                <Loading />
              </el-icon>
              <el-icon v-if="props.row.info.status === 'timing'" class="status">
                <Clock />
              </el-icon>
              <el-icon v-if="props.row.info.status === 'monitoring'" class="status">
                <View />
              </el-icon>
              <el-icon v-if="props.row.info.status === 'pausing'" class="status">
                <VideoPause />
              </el-icon>
              <el-icon v-if="props.row.info.status === 'unavailable'" class="status">
                <Warning />
              </el-icon>
              <el-tooltip placement="bottom" effect="light" :show-after="1000" :disabled="props.row.info.status === 'unavailable'">
                <template #content>
                  {{ props.row.config.cron_enable
                    ? `${t('table.nextTrigger')} ${props.row.info.next_run_time === "" ? t('error.cronNotTriggerYet') : props.row.info.next_run_time}`
                    : `${t('table.lastTrigger')} ${props.row.info.last_trigger_time === "" ? t('error.cronNotTriggerYet') : props.props.row.info.last_trigger_time}` }}
                </template>
                <span class="statusText">{{ t(`table.${props.row.info.status}`) }}</span>
              </el-tooltip>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('table.actions')">
          <template #default="props">
            <el-button
              type="warning"
              circle
              :icon="props.row.info.status !== 'pausing' ? VideoPause : VideoPlay"
              :disabled="props.row.info.status === 'unavailable'"
              @click="toggle_stop_click(props.row.config.id, props.row.info.status)"
            />
            <el-button type="primary" :icon="Edit" circle @click="toggle_edit_click(props.row.config.id)" />
            <el-button type="danger" :icon="Delete" circle @click="toggle_delete_dlick(props.row.config.id)" />
          </template>
        </el-table-column>

        <template #empty>
          <el-empty :description="t('table.empty')" />
        </template>
      </el-table>
    </div>

    <Lock :tray="['lock', 'edit', 'add', 'setting', 'statistic']" :default-lock="default_lock" @changeDefaultLock="toggle_change_default_lock" />
  </div>
</template>

<style lang="less" scoped>
@import "../assets/style/theme/default-vars.less";
.container {
    width: 100%;
    min-height: 100vh;
    margin-top: @title-bar-height;
    color: var(--el-color-primary);
    background-color: var(--el-bg-color);
}

.title {
    text-align: center;
}

.content {

}

.action {
    text-align: center;
}

.missionName {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.statusContainer {
  display: flex;
  flex-direction: row;
  align-items: center;

  .statusText {
    margin-left: 3px;
  }
}

.loading {
  animation: rotation 2s infinite linear;
}

@keyframes rotation {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(359deg);
  }
}
</style>
