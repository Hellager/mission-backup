import { createI18n } from 'vue-i18n'
import enUS from './langs/en_US.json'
import zhCN from './langs/zh_CN.json'

type MessageSchema = typeof zhCN
// type MessageSchema = typeof enUS

// type doesn't match error in ts: https://github.com/intlify/vue-i18n-next/issues/672
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
