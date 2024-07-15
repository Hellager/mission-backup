<script lang="ts" setup>
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { homeDir } from '@tauri-apps/api/path'
import { open } from '@tauri-apps/api/dialog'
import { Command, execute } from '../../../utils/cmd'
import type { MigratedData } from '../../../utils/cmd/types'

interface MigrateTableData {
  key: string
  label: string
  count: number
}

/**
 * Props definition for the dialog component.
 */
const props = defineProps({
  visiable: { type: Boolean, required: true },
})

const emit = defineEmits<{
  (event: 'hide', value: boolean): void
}>()

const migratedData = ref<MigratedData>()
const tableVisable = ref<boolean>(false)
const migrateTableData = ref<MigrateTableData[]>([])

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
  tableVisable.value = false
})

async function selectFile() {
  const selected = await open({
    directory: false,
    multiple: false,
    defaultPath: await homeDir(),
  })

  if (selected !== null) {
    const data = await execute(Command.MigrateFromOld, Array.isArray(selected) ? selected[0] : selected)
    migratedData.value = data
    await buileTableData(data)
  }
}

async function buileTableData(raw: MigratedData) {
  migrateTableData.value = [
    {
      key: 'config',
      label: t('toolbox.migrate.config'),
      count: 1,
    },
    {
      key: 'mission',
      label: t('toolbox.migrate.missionRelated'),
      count: (raw.backups.length + raw.ignores.length + raw.missions.length + raw.procedures.length),
    },
  ]
}

/**
 * Handle dialog cancel click event.
 */
async function onDialogCancelClicked() {
  emit('hide', true)
}

/**
 * Handle dialog confirm click event.
 */
async function onDialogConfirmClicked() {
  // store.tryClose(closeOption.value, remember.value)
  emit('hide', true)
}
</script>

<template>
  <div class="migration">
    <el-dialog
      v-model="visiable"
      :title="t('toolbox.migrate.title')"
      class="migration__dialog"
    >
      <div class="migration__dialog__select">
        <span>
          {{ t('toolbox.migrate.selectFile') }}
        </span>

        <el-button type="primary" @click="selectFile">
          {{ t('toolbox.migrate.select') }}
        </el-button>
      </div>

      <el-table :data="migrateTableData">
        <el-table-column type="expand">
          <template #default="props">
            <div>
              {{ props }}
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="t('toolbox.migrate.data')" prop="key" />
        <el-table-column :label="t('toolbox.migrate.count')" prop="key" />
        <el-table-column :label="t('toolbox.migrate.action')">
          <template #default="">
            <div>
              <el-button>
                {{ t('toolbox.migrate.migrate') }}
              </el-button>
            </div>
          </template>
        </el-table-column>
      </el-table>

      <template #footer>
        <div class="dialog-footer">
          <el-button @click="onDialogCancelClicked">
            {{ t("common.cancel") }}
          </el-button>
          <el-button type="primary" @click="onDialogConfirmClicked">
            {{ t('toolbox.migrate.migrate') }}
          </el-button>
        </div>
      </template>
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

.migration__dialog__select {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-evenly;
}
</style>
