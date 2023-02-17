<script lang="ts" setup>
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { marked } from 'marked'
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

watch(props, () => {
  dialog_visiable.value = props.isShow

  if (props.message !== undefined) {
    md_message.value = marked.parse(props.message)

    setTimeout(() => {
      const list_obj = document.getElementsByTagName('ul')[0]
      if (list_obj != null)
        list_obj.style.listStyle = 'disc'
    }, 0)
  }
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
    <div class="list" v-html="md_message" />
    <template #footer>
      <DialogFooter @cancel="close_dialog" @confirm="confirm_dialog" />
    </template>
  </el-dialog>
</template>

<style lang="less">
</style>
