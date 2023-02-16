<script lang="ts" setup>
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import TurndownService from 'turndown'
import { TauriCommand, execute_rust_command } from '../../utils'
import DialogFooter from './DialogFooter.vue'

const props = defineProps({
  isShow: Boolean,
  title: String,
  message: String,
  releaseUrl: String,
})

const emit = defineEmits(['close'])

const { t, locale } = useI18n({ useScope: 'global' })

const dialog_visiable = ref(props.isShow)
const md_message = ref('')
const turndown_service = new TurndownService()

watch(props, () => {
  dialog_visiable.value = props.isShow
  md_message.value = ref(turndown_service.turndown(props.message))
})

const close_dialog = () => {
  dialog_visiable.value = false
  emit('close')
}

async function confirm_dialog() {
  await execute_rust_command(TauriCommand.COMMAND_OPEN_URL, props.releaseUrl)
  emit('close')
}
</script>

<template>
  <el-dialog v-model="dialog_visiable" :show-close="false" :title="props.title">
    <div :h-html="md_message">
      {{ md_message }}
    </div>
    <template #footer>
      <DialogFooter @cancel="close_dialog" @confirm="confirm_dialog" />
    </template>
  </el-dialog>
</template>

<style lang="less">
</style>
