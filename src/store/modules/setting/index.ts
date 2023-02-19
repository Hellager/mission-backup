import { defineStore } from 'pinia'
import pinia from '../../index'
import type { Setting } from './types'

export const useSettingStore = defineStore(
  'setting',
  {
    state: () => ({
      is_initialized: false,
      is_page_initialized: false,

      is_auto_start: false,
      is_light_theme: true,
      is_close_to_tray: false,
      language: 'zh-CN',
      is_password_protected: true,
      monitor_delay: 3,
      is_notify_when_create_backup_success: true,
      is_notify_when_create_backup_failed: true,
      // is_webdav_enable: false,
      // is_webdav_available: false,
      // webdav_server: '',
      // webdav_username: '',
      // webdav_password: '',
      // is_samba_enable: false,
      // is_sambe_available: false,
      // samba_server: '',
      // samba_username: '',
      // samba_password: '',
      // is_ftp_enable: false,
      // is_ftp_available: false,
      // ftp_server: '',
      // ftp_username: '',
      // ftp_password: '',
      software_version: '',
    }),
    getters: {},
    actions: {
      initialize_settings(data: Setting) {
        this.is_auto_start = data.is_auto_start
        this.is_light_theme = data.is_light_theme
        this.is_close_to_tray = data.is_close_to_tray
        this.language = data.language
        this.is_password_protected = data.is_password_protected
        this.software_version = data.software_version
        this.is_notify_when_create_backup_success = data.is_notify_when_create_backup_success
        this.is_notify_when_create_backup_failed = data.is_notify_when_create_backup_failed
      },

      update_auto_start(data: boolean) {
        this.is_auto_start = data
      },

      update_theme(data: boolean) {
        this.is_light_theme = data
      },

      update_language(data: string) {
        this.language = data
      },

      update_close_to_tray(data: boolean) {
        this.is_close_to_tray = data
      },

      update_password_protect(data: boolean) {
        this.is_password_protected = data
      },

      update_monitor_delay(data: number) {
        this.monitor_delay = data
      },

      update_version(data: string) {
        this.software_version = data
      },

      update_page_initialized_status(status: boolean) {
        this.is_page_initialized = status
      },

      update_notify_when_create_backup_success(data: boolean) {
        this.is_notify_when_create_backup_success = data
      },

      update_notify_when_create_backup_failed(data: boolean) {
        this.is_notify_when_create_backup_success = data
      },

      // toggleChangeLanguage(lang: string) {
      //     this.language = lang;
      //     console.log('change language to ' + this.language);
      // },

      // toggleTheme(is_light: boolean) {
      //     this.is_light_theme = is_light;
      //     console.log("change theme to " + this.is_light_theme);
      // },

      // toggleUpdateDispatch(new_setting: Partial<Setting>) {
      //     console.log("new setting");
      //     console.log(new_setting);
      // },
    },
  },
)

export function useSettingOutsideStore() {
  return useSettingStore(pinia)
}
