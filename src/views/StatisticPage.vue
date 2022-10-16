<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { storeToRefs } from 'pinia'
import * as echarts from 'echarts/core'
import type {
  GridComponentOption,
  LegendComponentOption,
  ToolboxComponentOption,
  TooltipComponentOption,
} from 'echarts/components'
import {
  GridComponent,
  LegendComponent,
  ToolboxComponent,
  TooltipComponent,
} from 'echarts/components'
import type {
  BarSeriesOption,
  LineSeriesOption,
} from 'echarts/charts'
import {
  BarChart,
  LineChart,
} from 'echarts/charts'
import {
  UniversalTransition,
} from 'echarts/features'
import {
  CanvasRenderer,
} from 'echarts/renderers'
import { TauriCommand, execute_rust_command } from '../utils'
import { useMissionStore, useSettingStore } from '../store/index'
import PageHeader from '../components/PageHeader.vue'
import Lock from '../components/Lock.vue'

echarts.use(
  [ToolboxComponent, TooltipComponent, GridComponent, LegendComponent, BarChart, LineChart, CanvasRenderer, UniversalTransition],
)

type EChartsOption = echarts.ComposeOption<
    ToolboxComponentOption | TooltipComponentOption | GridComponentOption | LegendComponentOption | BarSeriesOption | LineSeriesOption
>

let statistic_chart

const { t, locale } = useI18n({
  useScope: 'global',
  inheritLocale: true,
})

const globalSetting = useSettingStore()
const { is_light_theme } = storeToRefs(globalSetting)
const missionStore = useMissionStore()
const { mission_list, current_mission } = storeToRefs(missionStore)

const selected_mission = ref('')
const selected_mission_status = ref('')
const selected_query_time_type = ref('date')
const selected_time = ref('')
const datetime_picker_format = ref('YYYY/MM/DD')
const datetime_picker_value_format = ref('YYYY-MM-DD')

const query_time_type_option = [
  {
    value: 'date',
    label: t('statistic.timeDate'),
  },
  {
    value: 'week',
    label: t('statistic.timeWeek'),
  },
  {
    value: 'month',
    label: t('statistic.timeMonth'),
  },
  {
    value: 'year',
    label: t('statistic.timeYear'),
  },
]

function form_x_axis(from: number, to: number, step: number): Array<number> | Array<string> {
  // force from value < to value
  if (from - to > 0)
    [from, to] = [to, from]

  return [...Array((to - from) / step + 1)].map((e, i) => from + i * step)
}

function form_x_axis_with_datetime(from: string, length: number): Array<string> {
  // from should be in format YYYY-MM-DD or YYYY-MM
  const xAxis = []
  const month = from.slice(5, 7)
  let day = from.slice(8)
  if (day === '')
    day = '01'

  for (let i = 0; i < length; i++) {
    const combined_datetime = `${month}/${(parseInt(day) + i).toString().padStart(2, '0')}`
    xAxis.push(combined_datetime)
  }

  return xAxis
}

function change_date_picker_format(val: any) {
  const current_time_type = selected_query_time_type.value
  switch (current_time_type) {
    case 'date':
      datetime_picker_format.value = 'YYYY/MM/DD'
      datetime_picker_value_format.value = 'YYYY-MM-DD'
      break

    case 'week':
      datetime_picker_format.value = '[Week] ww'
      datetime_picker_value_format.value = 'YYYY-MM-DD'
      break

    case 'month':
      datetime_picker_format.value = 'YYYY-MM'
      datetime_picker_value_format.value = 'YYYY-MM'
      break

    case 'year':
      datetime_picker_format.value = 'YYYY'
      datetime_picker_value_format.value = 'YYYY'
      break

    default:
      datetime_picker_format.value = 'YYYY-MM-DD'
      datetime_picker_value_format.value = 'YYYY-MM-DD'
      break
  }
}

async function change_current_mission(val: any) {
  selected_mission_status.value = missionStore.get_mission_status(selected_mission.value)

  if (selected_time.value !== '')
    console.log(`query mission: ${val} time: ${selected_time.value}`)
}

const colors = ['#5470C6', '#91CC75']

const option = ref({
  color: colors,
  darkMode: 'auto',
  tooltip: {
    trigger: 'axis',
    axisPointer: {
      type: 'cross',
    },
  },
  legend: {
    data: [
      {
        name: t('statistic.size'),
        textStyle: { color: colors[0] },
      },
      {
        name: t('statistic.count'),
        textStyle: { color: colors[1] },
      },
    ],
  },
  xAxis: [
    {
      type: 'category',
      axisTick: {
        alignWithLabel: true,
      },
      // prettier-ignore
      data: form_x_axis(2, 24, 2),
    },
  ],
  yAxis: [
    {
      type: 'value',
      name: t('statistic.size'),
      position: 'right',
      alignTicks: true,
      axisLine: {
        show: true,
        lineStyle: {
          color: colors[0],
        },
      },
      minInterval: 5,
      axisLabel: {
        formatter: '{value} MB',
      },
    },
    {
      type: 'value',
      name: t('statistic.count'),
      position: 'left',
      alignTicks: true,
      axisLine: {
        show: true,
        lineStyle: {
          color: colors[1],
        },
      },
      minInterval: 1,
      axisLabel: {
        formatter: '{value}',
      },
    },
  ],
  series: [
    {
      name: t('statistic.size'),
      type: 'bar',
      smooth: true,
      data: new Array(12).fill(0),
    },
    {
      name: t('statistic.count'),
      type: 'line',
      yAxisIndex: 1,
      smooth: true,
      data: new Array(12).fill(0),
    },
  ],
})

async function select_date(val: any) {
  if (selected_mission.value !== '') {
    await execute_rust_command(TauriCommand.COMMAND_GET_MISSION_BACKUPS_STATUS,
      selected_mission.value,
      selected_query_time_type.value,
      val,
      'Mib')
      .then((res) => {
        if (res.code === 200) {
          console.log(res.data)
          let updated_xAxis = []
          const value_length = res.data.count.length
          switch (selected_query_time_type.value) {
            case 'date':
              updated_xAxis = form_x_axis(2, 24, 2)
              break

            case 'week':
              updated_xAxis = form_x_axis_with_datetime(val, 7)
              break

            case 'month':
              updated_xAxis = form_x_axis_with_datetime(val, value_length)
              break

            case 'year':
              updated_xAxis = form_x_axis(1, 12, 1)
              break

            default:
              updated_xAxis = form_x_axis(2, 24, 2)
              break
          }

          update_chart_option(updated_xAxis, res.data.count, res.data.size)
        }
      })
  }
}

function update_chart_option(xAxis: number[] | string[], count: number[], size: number[]) {
  const xAxis_value_obj = [{
    type: 'category',
    axisTick: {
      alignWithLabel: true,
    },
    // prettier-ignore
    data: xAxis,
  }]

  const series_data_obj = [
    {
      name: t('statistic.size'),
      type: 'bar',
      smooth: true,
      data: size,
    },
    {
      name: t('statistic.count'),
      type: 'line',
      yAxisIndex: 1,
      smooth: true,
      data: count,
    },
  ]

  option.value.xAxis = xAxis_value_obj
  option.value.series = series_data_obj
}
</script>

<template>
  <div class="container">
    <PageHeader :title="t('component.pageHeaderStatisticPage')" to="/" />
    <div class="select">
      <el-select v-model="selected_mission" clearable :placeholder="t('statistic.chooseMission')" @change="change_current_mission">
        <el-option
          v-for="item in mission_list"
          :key="item.config.id"
          :label="item.config.name"
          :value="item.config.id"
        />
      </el-select>
      <el-select
        v-model="selected_query_time_type"
        clearable
        :disabled="selected_mission === '' || selected_mission_status === 'unavailable'"
        :placeholder="t('statistic.chooseType')"
        @change="change_date_picker_format"
      >
        <el-option
          v-for="item in query_time_type_option"
          :key="item.value"
          :label="item.label"
          :value="item.value"
        />
      </el-select>
      <el-date-picker
        v-model="selected_time"
        :disabled="selected_mission === '' || selected_mission_status === 'unavailable'"
        :type="selected_query_time_type"
        :format="datetime_picker_format"
        :value-format="datetime_picker_value_format"
        :placeholder="t('statistic.pickTime')"
        @change="select_date"
      />
    </div>
    <v-chart id="statistic" class="chart" :option="option" :autoresize="true" theme="light" />

    <Lock :tray="['lock', 'home']" />
  </div>
</template>

<style lang="less" scoped>
  @import "../assets/style/theme/default-vars.less";
.container {
    width: 100%;
    min-height: 100vh;
    padding-top: @title-bar-height;
    color: var(--el-color-primary);
    background-color: rgba(255, 255, 255, 0);
}

.select {
    padding-top: @title-bar-height;
    display: flex;
    flex-direction: row;
    margin-left: 10px;
    margin-right: 10px;
}

.action {
    padding-top: 20px;
    text-align: center;
}

.chart {
    width: 100%;
    height: 88vh;
}
</style>
