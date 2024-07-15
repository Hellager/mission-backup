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

async function onCleanDatabaseClick() {
  const info = await execute(Command.QueryDBInfo)
  ElMessageBox.confirm(
    `${t('toolbox.database.preCleanHint')}, ${info.deleted} ${t('toolbox.database.sufCleanHint')}`,
    t('common.confirm'),
    {
      distinguishCancelAndClose: true,
      confirmButtonText: t('common.confirm'),
      cancelButtonText: t('common.cancel'),
    },
  ).then(async () => {
    await execute(Command.CleanDatabase)
      .then((res: DBInfo) => {
        ElMessage.success(`${res.deleted} ${t('toolbox.database.cleanedHint')}`)
      })
  })
    .catch((action: Action) => {
      if (action === 'cancel') {
        // do nothing
      }
    })
}

async function onCheckDatabaseClick() {
  const info = await execute(Command.QueryDBInfo)
  const path = info.path
  await execute(Command.ShowInExplorer, path)
}
</script>

<template>
  <div class="tool__database">
    <el-dialog
      v-model="visiable"
      :title="t('toolbox.database.title')"
      class="tool__database__dialog"
    >
      <el-row>
        <el-col :span="12">
          <div class="tool__database__check">
            <el-button type="primary" @click="onCheckDatabaseClick">
              {{ t('toolbox.database.check') }}
            </el-button>
          </div>
        </el-col>

        <el-col :span="12">
          <div class="tool__database__clean">
            <el-button type="warning" @click="onCleanDatabaseClick">
              {{ t('toolbox.database.clean') }}
            </el-button>
          </div>
        </el-col>
      </el-row>
    </el-dialog>
  </div>
</template>

<style lang="less" scoped>
.tool__database {
  :deep(.el-overlay-dialog::-webkit-scrollbar) {
    width: 0;  /* Remove scrollbar space */
    background: transparent;  /* Optional: just make scrollbar invisible */
  }
}
</style>
