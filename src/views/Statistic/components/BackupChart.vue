<script setup lang="ts">
import * as echarts from 'echarts/core'
import type {
  GridComponentOption,
  TooltipComponentOption,
} from 'echarts/components'
import {
  GridComponent,
  TooltipComponent,
} from 'echarts/components'
import type { LineSeriesOption } from 'echarts/charts'
import { LineChart } from 'echarts/charts'
import { UniversalTransition } from 'echarts/features'
import { CanvasRenderer } from 'echarts/renderers'
import type { PropType } from 'vue'
import { onMounted, ref, watch } from 'vue'
import { map as lmap } from 'lodash'

/**
 * Defines props for the chart component.
 */
const props = defineProps({
  data: { type: Object as PropType<ChartData[]>, required: true },
})

/**
 * Initializes ECharts components and plugins.
 */
echarts.use([
  GridComponent,
  TooltipComponent,
  LineChart,
  CanvasRenderer,
  UniversalTransition,
])

/**
 * Represents the type of ECharts option.
 */
type EChartsOption = echarts.ComposeOption<
  GridComponentOption | LineSeriesOption | TooltipComponentOption
>

/**
 * Represents the structure of chart data.
 */
interface ChartData {
  mission: string
  createAt: string
  cnt: number
}

/**
 * Represents the reference to the ECharts instance.
 */
const chart = ref<echarts.ECharts>()

/**
 * Watches for changes in props and updates the chart data accordingly.
 */
watch(props, (newProps) => {
  updateChartData(newProps.data)
})

/**
 * Initializes the ECharts chart.
 */
function initChart() {
  const chartDom = document.getElementById('backup_chart')!
  const myChart = echarts.init(chartDom)

  chart.value = myChart

  window.addEventListener('resize', () => {
    if (chart.value !== undefined)
      chart.value.resize()
  })
}

/**
 * Updates the chart data based on the provided data.
 * @param data The chart data to be displayed.
 */
function updateChartData(data: ChartData[]) {
  // const xData = lmap(data, 'createAt')
  // const yData = lmap(data, 'cnt')
  // console.log(xData)
  // console.log(yData)

  const option: EChartsOption = {
    xAxis: {
      type: 'category',
      data: lmap(data, 'createAt'),
    },
    yAxis: {
      type: 'value',
    },
    tooltip: {
      trigger: 'axis',
    //   formatter: function(params: any) {
    //     return '<div>' + params[0].value + '</div>'
    //   }
    },
    series: [
      {
        data: lmap(data, 'cnt'),
        type: 'line',
        smooth: true,
      },
    ],

  }

  if (chart.value) {
    // console.log('set chart data')
    option && chart.value.setOption(option)
  }
}

/**
 * Initializes the chart on component mount.
 */
onMounted(() => {
  initChart()
})
</script>

<template>
  <div class="container">
    <div id="backup_chart" class="container__chart" />
  </div>
</template>

<style lang="less">
.container {
    width: 100%;
    height: 100%;
}

.container__chart {
    width: 100%;
    height: 100%;
}
</style>
