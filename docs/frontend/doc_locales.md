# Locales

## 功能

软件语言国际化，使用 json 文件作为原始文本，目前支持 中/英 双语



## 函数

```typescript
import { createI18n } from 'vue-i18n';
import enUS from './langs/en_US.json';
import zhCN from './langs/zh_CN.json';

type MessageSchema = typeof zhCN
// type MessageSchema = typeof enUS

// type doesn't match error in ts: https://github.com/intlify/vue-i18n-next/issues/672
const i18n = createI18n<[MessageSchema], 'zh-CN' | 'en-US'>({
    legacy: false,
    locale: 'zh-CN',
    fallbackLocale: 'en-US',
    messages: {
        'zh-CN': zhCN,
        'en-US': enUS
    }
})

```



