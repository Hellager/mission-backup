<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { storeToRefs } from 'pinia'
import { dayjs } from 'element-plus'
import { useMissionStore } from '../../store'
import { Command, execute } from '../../utils/cmd'
import BackupChart from './components/BackupChart.vue'
// import type { Backup } from '../../store/mission/types';

interface SelectOptions {
  value: string
  label: string
}

// interface StasticForm {
//     missionId: string,
//     time_start: string,
//     time_stop: string,
// }

interface ChartData {
  mission: string
  createAt: string
  cnt: number
}

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 * Initializes the mission store and sets up necessary variables and functions for managing notifications.
 */
const store = useMissionStore()
const { missions } = storeToRefs(store)

/**
 * Represents the options for missions in a select dropdown.
 */
const missionOptions = ref<SelectOptions[]>([])

/**
 * Represents the date range.
 */
const dateRange = ref<string>('')

/**
 * Represents the currently selected mission.
 */
const curMission = ref<string>('')

/**
 * Represents the chart data for mission backups.
 */
const missionBackups = ref<ChartData[]>([])

/**
 * Represents the start date for data filtering.
 */
const startDate = ref<string | undefined>('')

/**
 * Represents the end date for data filtering.
 */
const endDate = ref<string | undefined>('')

/**
 * Asynchronously builds the options for missions based on stored data.
 */
async function buildMissionOptions() {
  await store.syncRecords('procedure')

  const option_data = [] as SelectOptions[]
  for (const item of missions.value) {
    option_data.push({
      value: item.missionId,
      label: item.name,
    })
  }

  missionOptions.value = option_data
  // set default selected mission
  curMission.value = option_data[0].value

  queryChartData()
}

/**
 * Handles the change event for the date picker input.
 */
async function onPickerDataChange() {
  const start_date = Date.parse(dateRange.value[0]) / 1000
  let stop_date = Date.parse(dateRange.value[1]) / 1000

  if (start_date === stop_date)
    stop_date += 86400 // 24h

  startDate.value = dayjs.unix(start_date).format()
  endDate.value = dayjs.unix(stop_date).format()

  // console.log("new data range:")
  // console.log(dateRange.value);
  // console.log("start date:")
  // console.log(start_date);
  // console.log(start);
  // //console.log(dayjs(dateRange.value[0]))
  // //console.log(dayjs(Date.parse(dateRange.value[0])).local().format('YYYY-MM-DD HH:mm'))
  // console.log("end date:")
  // console.log(stop_date);
  // console.log(stop);
  queryChartData()
}

/**
 * Queries chart data based on selected mission and date range.
 */
async function queryChartData() {
  if (startDate.value === '' || startDate.value === 'Invalid Date')
    startDate.value = undefined

  if (endDate.value === '' || startDate.value === 'Invalid Date')
    endDate.value = undefined

  // console.log(`mission ${curMission.value}, start ${startDate.value}, end ${endDate.value}`)
  await execute(Command.QueryStatistic, curMission.value, startDate.value, endDate.value)
    .then((res: any) => {
      const backups = [] as ChartData[]
      let mission_name = ''
      for (const item of missionOptions.value) {
        if (item.value === curMission.value)
          mission_name = item.label
      }
      for (const [idx, item] of res.entries()) {
        backups.push({
          mission: mission_name,
          createAt: dayjs.utc(item.createAt.substring(0, 23)).local().format('YYYY-MM-DD HH:mm'),
          cnt: idx + 1,
        })
      }
      missionBackups.value = backups

      // console.log(backups)
    })

  // console.log("chart data:")
  // console.log(missionBackups.value);
}

/**
 * Initializes the component by building mission options.
 */
onMounted(async () => {
  await buildMissionOptions()
})
</script>

<template>
  <div class="statistic">
    <div class="statistic__select">
      <el-row>
        <el-col :span="6">
          <el-select
            v-model="curMission"
            :placeholder="t('statistic.selectMission')"
            :disabled="missions.length === 0"
            @focus="onPickerDataChange"
          >
            <el-option
              v-for="item in missionOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value"
            />
          </el-select>
        </el-col>

        <el-col :span="12">
          <el-date-picker
            v-model="dateRange"
            type="daterange"
            :disabled="missions.length === 0"
            :range-separator="t('statistic.dateSeperator')"
            :start-placeholder="t('statistic.startDate')"
            :end-placeholder="t('statistic.endDate')"
          />
        </el-col>
      </el-row>
    </div>

    <div class="statistic__chart">
      <BackupChart :data="missionBackups" />
    </div>
  </div>
</template>

<style scoped lang="less">
.statistic {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
}

.statistic__select {
    margin-left: 3px;
    margin-top: 3px;
}

.statistic__chart {
    width: 100%;
    height: 100%;
}
</style>
