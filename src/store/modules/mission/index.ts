import { defineStore } from 'pinia'
import pinia from '../../index'
import type { Mission, MissionInfo } from './types'

export const useMissionStore = defineStore(
  'mission',
  {
    state: () => ({
      mission_list: [] as Mission[],
      current_mission: {} as Mission | undefined,
      current_drop_mission_path: '',
    }),
    getters: {},
    actions: {
      initialize_mission_list(list: Array<Mission>) {
        this.mission_list = list
      },

      update_current_mission(id: string) {
        this.mission_list.forEach((value) => {
          if (value.config.id === id)
            this.current_mission = value
        })
      },

      update_current_drop_mission_path(path: string) {
        this.current_drop_mission_path = path
      },

      append_new_mission(data: Mission) {
        this.mission_list.push(data)
        this.mission_list.forEach((item) => {
          if (item.config.id === data.config.id)
            item.info.next_run_time = data.info.next_run_time.slice(0, data.info.next_run_time.indexOf(' +'))
        })
      },

      edit_mission(id: string, new_config: Mission) {
        let target_index = 0
        this.mission_list.forEach((value, index) => {
          if (value.config.id === id)
            target_index = index
        })

        this.mission_list[target_index] = new_config
        this.mission_list[target_index].info.next_run_time = new_config.info.next_run_time.slice(0, new_config.info.next_run_time.indexOf(' +'))
      },

      start_mission(id: string) {
        this.mission_list.forEach((value) => {
          if (value.config.id === id)
            value.info.status = value.config.cron_enable ? 'timing' : 'monitoring'
        })
      },

      stop_mission(id: string) {
        this.mission_list.forEach((value) => {
          if (value.config.id === id)
            value.info.status = 'pausing'
        })
      },

      delete_mission(id: string) {
        this.mission_list.forEach((value, index) => {
          if (value.config.id === id)
            this.mission_list.splice(index, 1)
        })
      },

      update_mission_status(id: string, status: string) {
        this.mission_list.forEach((item) => {
          if (item.config.id === id)
            item.info.status = status
        })
      },

      update_mission_info(id: string, info: MissionInfo) {
        this.mission_list.forEach((item) => {
          if (item.config.id === id) {
            item.info.status = info.status
            item.info.last_trigger_time = info.last_trigger_time
            item.info.next_run_time = info.next_run_time.slice(0, info.next_run_time.indexOf(' +'))
          }
        })
      },

      get_mission_status(id: string) {
        let status = ''
        this.mission_list.forEach((item) => {
          if (item.config.id === id)
            status = item.info.status
        })

        return status
      },
    },
  },
)

export function useMissionoutsideStore() {
  return useMissionStore(pinia)
}
