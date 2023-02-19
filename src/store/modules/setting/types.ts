export interface Setting {
  is_auto_start: boolean
  is_light_theme: boolean
  is_close_to_tray: boolean
  language: string
  is_password_protected: boolean
  monitor_delay: number
  is_notify_when_create_backup_success: boolean
  is_notify_when_create_backup_failed: boolean
  // is_webdav_enable: boolean;
  // is_webdav_available: boolean;
  // webdav_host_address: string;
  // webdav_username: string;
  // webdav_password: string;
  // is_samba_enable: boolean;
  // is_sambe_available: boolean;
  // samba_host_address: string;
  // samba_username: string;
  // samba_password: string;
  // is_ftp_enable: boolean;
  // is_ftp_available: boolean;
  // ftp_host_address: string;
  // ftp_username: string;
  // ftp_password: string;
  // is_update_available: string;
  software_version: string
}

export interface Webdav {
  is_webdav_enable: boolean
  is_webdav_available: boolean
  webdav_server: string
  webdav_username: string
  webdav_password: string
}

export interface Samba {
  is_samba_enable: boolean
  is_sambe_available: boolean
  samba_server: string
  samba_username: string
  samba_password: string
}

export interface FTP {
  is_ftp_enable: boolean
  is_ftp_available: boolean
  ftp_server: string
  ftp_username: string
  ftp_password: string
}
