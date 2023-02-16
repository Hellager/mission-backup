<script lang="ts" setup>
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { storeToRefs } from 'pinia'
import { useMissionStore, useSettingStore } from '../../store/index'
import { TauriCommand, execute_rust_command } from '../../utils'
import DialogFooter from './DialogFooter.vue'

const props = defineProps({
  isShow: Boolean,
  title: String,
})

const emit = defineEmits(['close'])

const missionStore = useMissionStore()
const globalSetting = useSettingStore()
const { t, locale } = useI18n({ useScope: 'global' })

const { language, monitor_delay } = storeToRefs(globalSetting)
const { mission_list } = storeToRefs(missionStore)

const dialog_visiable = ref(props.isShow)
const input_delay_time = ref(monitor_delay.value)

watch(props, () => {
  dialog_visiable.value = props.isShow
})

const close_dialog = () => {
  dialog_visiable.value = false
  emit('close')
}

async function toggle_change_monitor_delay() {
  let any_trouble = false
  if (typeof input_delay_time.value != 'number') {
    ElMessage.error({
      showClose: true,
      message: t('error.dataFormatError'),
      center: true,
    })
    any_trouble = true
  }

  mission_list.value.forEach((item) => {
    if (item.config.monitor_enable && item.info.status === 'executing') {
      ElMessage.error({
        showClose: true,
        message: t('error.jobRunningError'),
        center: true,
      })
    }

    any_trouble = true
  })

  if (any_trouble) {
    emit('close')
    return
  }

  const res = await execute_rust_command(TauriCommand.COMMAND_CHANGE_SETTING_MONITOR_DELAY, input_delay_time.value)
  if (res) {
    globalSetting.update_monitor_delay(input_delay_time.value)
    dialog_visiable.value = false
    ElMessage.success({
      showClose: true,
      message: t('setting.changeDelaySuccess'),
      center: true,
    })
  }
  else {
    ElMessage.error({
      showClose: true,
      message: t('error.changeDelayFailed'),
      center: true,
    })
  }

  emit('close')
}
</script>

<template>
  <el-dialog v-model="dialog_visiable" :show-close="false" :title="props.title">
    <el-form class="setting-dialog" label-position="right" :label-width="language === 'zh-CN' ? 'auto' : '110px'">
      <el-form-item :label="t('setting.targetDelay')">
        <el-input-number v-model="input_delay_time" :min="1" :max="60" />
      </el-form-item>
    </el-form>
    <span class="delayTip">
      {{ t('general.validAfterRestart') }}
    </span>
    <template #footer>
      <DialogFooter @cancel="close_dialog" @confirm="toggle_change_monitor_delay" />
    </template>
  </el-dialog>
</template>

<style lang="less">
</style>
