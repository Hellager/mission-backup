import { createI18n } from 'vue-i18n'
import enUS from './langs/en_US.json'
import zhCN from './langs/zh_CN.json'

/**
 * Represents the message schema type based on the 'zhCN' constant.
 */
type MessageSchema = typeof zhCN

/**
 * Creates the i18n instance with message schema and language options.
 */
const i18n = createI18n<[MessageSchema], 'zh-CN' | 'en-US'>({
  legacy: false,
  locale: 'zh-CN',
  fallbackLocale: 'en-US',
  messages: {
    'zh-CN': zhCN,
    'en-US': enUS,
  },
})

export default i18n
