<script lang="ts" setup>
import { computed, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import cronstrue from 'cronstrue/i18n'
import { Cron } from 'croner'
import { ElMessage, dayjs } from 'element-plus'
import { writeText } from '@tauri-apps/api/clipboard'
import { Copy } from '../../../assets/icons'
import { DialogMode } from '../../../types'
import { subStringOccurrences } from '../../../utils/common'

/**
 * Props definition for the dialog component.
 */
const props = defineProps({
  visiable: { type: Boolean, required: true },
  mode: { type: Number, required: true },
  expression: { type: String, required: true },
})

const emit = defineEmits<{
  (event: 'hide', value: boolean): void
}>()

/**
 * Used for internationalization.
 */
const { t, locale } = useI18n({ useScope: 'global' })

const input = ref<string>('0 0/30 * * * *')
const hintCron = ref<string>('')
const hintNext = ref<string>('')
const showPartDoc = ref<string>('')

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
  if (props.mode === DialogMode.Create)
    input.value = '0 0/30 * * * *'
  else if (props.mode === DialogMode.Edit)
    input.value = props.expression

  showPartDoc.value = ''

  setTimeout(() => {
    const cronInput = document.getElementById('crontabInput')
    cronInput?.addEventListener('click', onCronInputPartClicked)
  }, 100)
})

const cronParts = [
  {
    label: t('toolbox.crontab.second'),
    value: 'second',
  },
  {
    label: t('toolbox.crontab.minute'),
    value: 'minute',
  },
  {
    label: t('toolbox.crontab.hour'),
    value: 'hour',
  },
  {
    label: t('toolbox.crontab.dayOfMonth'),
    value: 'dayOfMonth',
  },
  {
    label: t('toolbox.crontab.month'),
    value: 'month',
  },
  {
    label: t('toolbox.crontab.dayOfWeek'),
    value: 'dayOfWeek',
  },
]

const cronValues = [
  {
    label: '*',
    value: 'asterisk',
    group: 'general',
    desp: t('toolbox.crontab.asterisk'),
  },
  {
    label: ',',
    value: 'comma',
    group: 'general',
    desp: t('toolbox.crontab.comma'),
  },
  {
    label: '-',
    value: 'hyphen',
    group: 'general',
    desp: t('toolbox.crontab.hyphen'),
  },
  {
    label: '/',
    value: 'slash',
    group: 'general',
    desp: t('toolbox.crontab.slash'),
  },
  {
    label: t('toolbox.crontab.second'),
    value: 'second',
    group: 'second',
    min: 0,
    max: 59,
    desp: t('toolbox.crontab.valueRange'),
  },
  {
    label: t('toolbox.crontab.minute'),
    value: 'minute',
    group: 'minute',
    min: 0,
    max: 59,
    desp: t('toolbox.crontab.valueRange'),
  },
  {
    label: t('toolbox.crontab.hour'),
    value: 'hour',
    group: 'hour',
    min: 0,
    max: 23,
    desp: t('toolbox.crontab.valueRange'),
  },
  {
    label: t('toolbox.crontab.dayOfMonth'),
    value: 'dayOfMonth',
    group: 'day',
    min: 1,
    max: 31,
    desp: t('toolbox.crontab.valueRange'),
  },
  {
    label: t('toolbox.crontab.month'),
    value: 'month',
    group: 'month',
    min: 1,
    max: 12,
    desp: t('toolbox.crontab.valueRange'),
    alterMin: 'JAN',
    alterMax: 'DEC',
    alterDesp: t('toolbox.crontab.alterValues'),
  },
  {
    label: t('toolbox.crontab.dayOfWeek'),
    value: 'dayOfWeek',
    group: 'week',
    min: 0,
    max: 6,
    desp: t('toolbox.crontab.valueRange'),
    alterMin: 'SUN',
    alterMax: 'SAT',
    alterDesp: t('toolbox.crontab.alterValues'),
  },
]

function buildHint() {
  try {
    const cron_locale = locale.value === 'zh-CN' ? 'zh_CN' : 'en'
    const cronJob = Cron(input.value, { maxRuns: 1 }, () => {})
    const nextRuntime = dayjs(cronJob.nextRun()).format('YYYY-MM-DD HH:mm:ss')

    hintCron.value = cronstrue.toString(input.value, { locale: cron_locale })
    hintNext.value = nextRuntime
  }
  catch {
    ElMessage.error(t('error.invalidCron'))
    input.value = '0 0/30 * * * *'
  }
}

function onCronPartClick(part: string) {
  showPartDoc.value = part

  let cronPartIdx = 0
  cronParts.forEach((item, idx) => {
    const cronPartElId = `cronInputPart${item.value}`
    const cronPartEl = document.getElementById(cronPartElId) as HTMLButtonElement
    if (item.value === part) {
      cronPartEl.click()
      cronPartEl.style.color = '#f89898'
      cronPartIdx = idx
    }
    else {
      cronPartEl.style.color = '#409eff'
    }
  })

  const inputEl = document.getElementById('crontabInput') as HTMLInputElement
  const expression = inputEl.value
  const partsArr = expression.split(' ')
  let starIdx = 0
  const partLen = partsArr[cronPartIdx].length
  for (let i = 0; i < partsArr.length; i++) {
    if (i !== cronPartIdx)
      starIdx += partsArr[i].length
    else
      break
    starIdx += ' '.length
  }

  const stopIdx = starIdx + partLen
  inputEl.focus()
  inputEl.setSelectionRange(starIdx, stopIdx)
}

function onCronInputPartClicked(event: Event) {
  const el = event.target as HTMLInputElement
  const expression = el.value
  const starIdx = el.selectionStart
  if (starIdx != null) {
    const preString = expression.substring(0, starIdx)
    const nextString = expression.substring(starIdx)
    const lastSpace = preString.lastIndexOf(' ') + 1
    const nextSpace = nextString.indexOf(' ')

    let stopIdx = starIdx + nextSpace
    if (nextSpace === -1 || starIdx === expression.length)
      stopIdx = expression.length

    el.setSelectionRange(lastSpace, stopIdx)

    const cronPartIdx = 5 - subStringOccurrences(nextString, ' ')
    cronParts.forEach((item, index) => {
      const cronPartElId = `cronInputPart${item.value}`
      const cronPartEl = document.getElementById(cronPartElId) as HTMLButtonElement
      if (index === cronPartIdx) {
        cronPartEl.click()
        cronPartEl.style.color = '#f89898'
      }
      else {
        cronPartEl.style.color = '#409eff'
      }
    })
  }
}

onMounted(() => {
  buildHint()
})

async function onCopyCronExpressionClicked() {
  await writeText(input.value)

  ElMessage.success(t('info.valueCopied'))
}
</script>

<template>
  <div class="crontab">
    <el-dialog
      v-model="visiable"
      :title="t('toolbox.crontab.title')"
      class="crontab__dialog"
    >
      <div class="crontab__hint">
        <div class="crontab__hint__cron">
          {{ `"${hintCron}"` }}
        </div>

        <div class="crontab__hint__next">
          {{ `${t('toolbox.crontab.next')}: ${hintNext}` }}
        </div>
      </div>

      <div class="crontab__input">
        <div class="crontab__input__expression">
          <el-input id="crontabInput" v-model="input" size="large" @focus="buildHint()" @blur="buildHint()">
            <template #append>
              <el-button :icon="Copy" @click="onCopyCronExpressionClicked" />
            </template>
          </el-input>
        </div>

        <div class="crontab__input__label">
          <template v-for="item in cronParts" :key="item.value">
            <el-button :id="`cronInputPart${item.value}`" link type="primary" @click="onCronPartClick(item.value)">
              {{ item.label }}
            </el-button>
          </template>
        </div>
      </div>

      <table class="crontab__document">
        <tr v-for="item in cronValues" :key="item.value">
          <th v-if="item.group === 'general'" :key="item.value">
            {{ `${item.label}` }}
          </th>
          <td v-if="item.group === 'general'" :key="item.value">
            {{ `${item.desp}` }}
          </td>
        </tr>

        <tr v-for="item in cronValues" :key="item.value">
          <th v-if="item.value === showPartDoc" :key="item.value">
            {{ `${item.min}-${item.max}` }}
          </th>
          <td v-if="item.value === showPartDoc" :key="item.value">
            {{ `${item.desp}` }}
          </td>
        </tr>

        <tr v-for="item in cronValues" :key="item.value">
          <th v-if="(item.value === showPartDoc) && (item.alterDesp !== undefined)" :key="item.value">
            {{ `${item.alterMin}-${item.alterMax}` }}
          </th>
          <td v-if="(item.value === showPartDoc) && (item.alterDesp !== undefined)" :key="item.value">
            {{ `${item.alterDesp}` }}
          </td>
        </tr>
      </table>
    </el-dialog>
  </div>
</template>

<style lang="less" scoped>
.crontab {
  :deep(.el-overlay-dialog::-webkit-scrollbar) {
    width: 0;  /* Remove scrollbar space */
    background: transparent;  /* Optional: just make scrollbar invisible */
  }
}

.crontab__hint {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.crontab__hint__cron {
  font-size: x-large;
  color: var(--el-color-primary);
}

.crontab__hint__next {
  margin-top: 1em;
  font-size: small;
  color: var(--el-color-primary-dark-2);
}

.crontab__input {
  margin-top: 1em;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.crontab__input__expression {
  width: 80%;
  :deep(.el-input__inner) {
      text-align: center;
      font-size: large;
  }
}

.crontab__input__label {
  margin-top: 1em;
  display: flex;
  flex-direction: row;
  justify-content: space-evenly;

  // :deep(.el-button--primary:focus) {
  //   color: var(--el-color-danger-light-3);
  // }
}

.crontab__document {
  margin-top: 1em;
  // display: flex;
  // flex-direction: column;
  // align-items: center;
  margin-left: 3em;
  margin-right: auto;
  table-layout: auto;

  th {
    height: 1.5em;
    width: 5em;
  }

  td {
    height: 1.5em;
    width: 8em;
  }
}
</style>
