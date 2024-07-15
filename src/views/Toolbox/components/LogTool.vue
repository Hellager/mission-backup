<script lang="ts" setup>
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { ElMessage, ElMessageBox } from 'element-plus'
import type { Action } from 'element-plus'
import { Command, execute } from '../../../utils/cmd'
import type { DBInfo } from '../../../utils/cmd/types'

/**
 * Props definition for the dialog component.
 */
const props = defineProps({
  visiable: { type: Boolean, required: true },
})

const emit = defineEmits<{
  (event: 'hide', value: boolean): void
}>()

const infoData = ref<DBInfo>()

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 * Computed property for managing dialog visibility and behavior.
 */
const visiable = computed({
  get: () => {
    return props.visiable
  },
  set: (value: boolean) => emit('hide', value),
})

watch(visiable, async (_new, _old) => {
  const info = await execute(Command.QueryDBInfo)
  infoData.value = info
})

async function onCheckLogClick() {
  const info = await execute(Command.QueryLogInfo)
  if (info.path === 'term')
    ElMessage.warning('Terminal logging')
  else
    await execute(Command.ShowInExplorer, info.path)
}

async function onCleanLogClick() {
  const info = await execute(Command.QueryLogInfo)
  ElMessageBox.confirm(
        `${t('toolbox.log.preCleanHint')}, ${info.size} ${t('toolbox.log.sufCleanHint')}`,
        t('common.confirm'),
        {
          distinguishCancelAndClose: true,
          confirmButtonText: t('common.confirm'),
          cancelButtonText: t('common.cancel'),
        },
  ).then(async () => {
    await execute(Command.CleanAppLog)
      .then((res: number) => {
        ElMessage.success(`${res} ${t('toolbox.log.cleanedHint')}`)
      })
  })
    .catch((action: Action) => {
      if (action === 'cancel') {
        // do nothing
      }
    })
}
</script>

<template>
  <div class="tool__log">
    <el-dialog
      v-model="visiable"
      :title="t('toolbox.log.title')"
      class="tool__log__dialog"
    >
      <el-row>
        <el-col :span="12">
          <div class="tool__log__check">
            <el-button type="primary" @click="onCheckLogClick">
              {{ t('toolbox.log.check') }}
            </el-button>
          </div>
        </el-col>

        <el-col :span="12">
          <div class="tool__log__clean">
            <el-button type="warning" @click="onCleanLogClick">
              {{ t('toolbox.log.clean') }}
            </el-button>
          </div>
        </el-col>
      </el-row>
    </el-dialog>
  </div>
</template>

<style lang="less" scoped>
.tool__log {
  :deep(.el-overlay-dialog::-webkit-scrollbar) {
    width: 0;  /* Remove scrollbar space */
    background: transparent;  /* Optional: just make scrollbar invisible */
  }
}
</style>
