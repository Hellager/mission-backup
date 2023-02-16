<script lang="ts" setup>
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import DialogFooter from './DialogFooter.vue'

const props = defineProps({
  isShow: Boolean,
  title: String,
})

const emit = defineEmits(['close'])

const { t, locale } = useI18n({ useScope: 'global' })

const dialog_visiable = ref(props.isShow)

watch(props, () => {
  dialog_visiable.value = props.isShow
})

const close_dialog = () => {
  dialog_visiable.value = false
  emit('close')
}

const confirm_dialog = () => {
  console.log('confirm clicked')
  emit('close')
}
</script>

<template>
  <el-dialog v-model="dialog_visiable" :show-close="false" :title="props.title">
    <div>
      {{ "Template Dialog" }}
    </div>
    <template #footer>
      <DialogFooter @cancel="close_dialog" @confirm="confirm_dialog" />
    </template>
  </el-dialog>
</template>

<style lang="less">
</style>
