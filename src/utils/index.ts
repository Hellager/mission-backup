import { invoke } from '@tauri-apps/api/tauri'

enum TauriCommand {
  COMMAND_SHOW_MAIN_WINDOW,
  COMMAND_CLOSE_SPLASHSCREEN,
  COMMAND_INITIALIZE_DATA,
  COMMAND_IS_INITIALIZED,
  COMMAND_IS_PASSWORD_SET,
  COMMAND_INITIALIZE_PROGRAM_STATUS,
  COMMAND_EXIT_PROGRAM,
  COMMAND_CLOSE_TO_TRAY,
  COMMAND_START_TIMING_SAVE_DATA,
  COMMAND_TIMING_SAVE_DATA,
  COMMAND_CHANGE_SETTING_IS_AUTO_START,
  COMMAND_CHANGE_SETTING_IS_LIGHT_THEME,
  COMMAND_CHANGE_SETTING_IS_CLOSE_TO_TRAY,
  COMMAND_CHANGE_WINDOW_STATE_BY_SYSTEM_TRAY,
  COMMAND_CHANGE_SETTING_LANGUAGE,
  COMMAND_CHANGE_SETTING_IS_PASSWORD_PROTECTED,
  COMMAND_CHANGE_SETTING_PASSWORD,
  COMMAND_CHANGE_SETTING_MONITOR_DELAY,
  COMMAND_UNLOCK,
  COMMAND_CREATE_MISSION,
  COMMAND_START_MISSION,
  COMMAND_STOP_MISSION,
  COMMAND_EDIT_MISSION,
  COMMAND_DELETE_MISSION,
  COMMAND_FORCE_DELETE_MISSION,
  COMMAND_CHECK_PATH_VALID,
  COMMAND_GET_DROP_PATH_INFO,
  COMMAND_UPDATE_LIST_INFO,
  COMMAND_GET_MISSION_BACKUPS_STATUS,
  COMMAND_OPEN_URL,
  COMMAND_CHANGE_IS_NOTIFY_WHEN_CREATE_BACKUP_SUCCESS,
  COMMAND_CHANGE_IS_NOTIFY_WHEN_CREATE_BACKUP_FAILED,
}

// tauri command 参数只能是驼峰命名
async function execute_rust_command(command: Number, data?: any, additional?: any, additional2?: any, additional3?: any) {
  let result: any = false

  // console.error('id  ' + data + " date_type " + additional + " time " + additional2 + " unit " + additional3);

  switch (command) {
    case TauriCommand.COMMAND_SHOW_MAIN_WINDOW:
      result = await invoke('show_mainwindow')
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_CLOSE_SPLASHSCREEN:
      result = await invoke('close_splashscreen')
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_INITIALIZE_DATA:
      result = await invoke('initialize_data')
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_IS_INITIALIZED:
      result = await invoke('is_program_initialized')
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_IS_PASSWORD_SET:
      result = await invoke('is_password_set')
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_INITIALIZE_PROGRAM_STATUS:
      result = await invoke('initialize_program_status')
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_EXIT_PROGRAM:
      await invoke('exit_program')
      break

    case TauriCommand.COMMAND_CLOSE_TO_TRAY:
      await invoke('close_to_tray')
      break

    case TauriCommand.COMMAND_START_TIMING_SAVE_DATA:
      await invoke('start_timing_save_data')
      break

    case TauriCommand.COMMAND_TIMING_SAVE_DATA:
      await invoke('timing_save_data')
      break

    case TauriCommand.COMMAND_CHANGE_SETTING_IS_AUTO_START:
      result = await invoke('change_setting_is_auto_start', { autoStart: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_CHANGE_SETTING_IS_LIGHT_THEME:
      result = await invoke('change_setting_is_light_theme', { isLight: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_CHANGE_SETTING_IS_CLOSE_TO_TRAY:
      result = await invoke('change_setting_is_close_to_tray', { closeToTray: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_CHANGE_WINDOW_STATE_BY_SYSTEM_TRAY:
      result = await invoke('change_window_state_by_system_tray')
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_CHANGE_SETTING_LANGUAGE:
      result = await invoke('change_setting_language', { lang: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_CHANGE_SETTING_IS_PASSWORD_PROTECTED:
      result = await invoke('change_setting_is_password_protect', { passwordProtect: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_CHANGE_SETTING_PASSWORD:
      result = await invoke('change_setting_password', { oldPassword: data, newPassword: additional })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_CHANGE_SETTING_MONITOR_DELAY:
      result = await invoke('change_monitor_delay', { delay: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_UNLOCK:
      result = await invoke('unlock', { password: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_CREATE_MISSION:
      result = await invoke('create_mission', { config: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_START_MISSION:
      result = await invoke('start_mission', { id: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_STOP_MISSION:
      result = await invoke('stop_mission', { id: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_EDIT_MISSION:
      result = await invoke('edit_mission', { id: data, config: additional })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_DELETE_MISSION:
      result = await invoke('delete_mission', { id: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_FORCE_DELETE_MISSION:
      result = await invoke('force_delete_mission', { id: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_CHECK_PATH_VALID:
      result = await invoke('check_path_valid', { path: data, expected: additional })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_GET_DROP_PATH_INFO:
      result = await invoke('get_drop_path_info', { path: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_UPDATE_LIST_INFO:
      result = await invoke('update_list_info', { list: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_GET_MISSION_BACKUPS_STATUS:
      result = await invoke('get_mission_backups_status', { id: data, dateType: additional, startDatetime: additional2, sizeUnit: additional3 })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_OPEN_URL:
      result = await invoke('open_url', { url: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_CHANGE_IS_NOTIFY_WHEN_CREATE_BACKUP_SUCCESS:
      result = await invoke('change_setting_is_notify_when_create_backup_success', { isNotify: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    case TauriCommand.COMMAND_CHANGE_IS_NOTIFY_WHEN_CREATE_BACKUP_FAILED:
      result = await invoke('change_setting_is_notify_when_create_backup_failed', { isNotify: data })
        .then((res) => { return res })
        .catch(err => console.error(err))
      break

    default:
      console.error('No command matches')
      break
  }

  return result
}

export {
  TauriCommand,
  execute_rust_command,
}
