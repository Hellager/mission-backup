export interface MissionConfig {
  id: string
  name: string
  target_type: string
  from_path: string
  to_path: string
  ignore_enable: boolean
  ignore_method: string
  ignores: Array<string>
  compress_enable: boolean
  compress_format: string
  cron_enable: boolean
  cron_expression: string
  monitor_enable: boolean
  restrict_save_days_enable: boolean
  save_days: number
  restrict_save_size_enable: boolean
  save_size: number
  backup_method: string
}

export interface MissionInfo {
  status: string
  next_run_time: string
  last_trigger_time: string
}

export interface Mission {
  config: MissionConfig
  info: MissionInfo
}
