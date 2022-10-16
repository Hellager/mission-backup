import { createPinia } from 'pinia'
import { useSettingStore } from './modules/setting'
import { useMissionStore } from './modules/mission'
import { useStatisticStore } from './modules/statistic'

const pinia = createPinia()

export { useSettingStore, useMissionStore, useStatisticStore }
export default pinia
