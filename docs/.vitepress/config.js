export default {
  title: '有备',
  description: '简单好用的本地备份软件',
  themeConfig: {
    logo: '/icon.svg',
    sidebar: [
      {
        text: '项目简介',
        items: [
          { text: '下载', link: '/general/download' },
          { text: '使用', link: '/general/usage' },
          { text: '开发', link: '/general/develop' },
        ],
      },
      {
        text: '前端文档',
        items: [
          { text: 'Component', link: '/frontend/doc_component' },
          { text: 'Command', link: '/frontend/doc_command' },
          { text: 'Locales', link: '/frontend/doc_locales' },
          { text: 'Views', link: '/frontend/doc_views' },
          { text: 'Router', link: '/frontend/doc_router' },
          { text: 'Store', link: '/frontend/doc_store' },
          { text: 'App', link: '/frontend/doc_app_frontend' },
        ],
      },
      {
        text: '后端文档',
        items: [
          { text: 'Command', link: '/backend/doc_command' },
          { text: 'Handler', link: '/backend/doc_handler' },
          { text: 'Logger', link: '/backend/doc_logger' },
          { text: 'Plugin', link: '/backend/doc_plugin' },
          { text: 'Types', link: '/backend/doc_types' },
          { text: 'App', link: '/backend/doc_app_backend' },
          { text: 'Compressor', link: '/backend/doc_compressor' },
          { text: 'Explorer', link: '/backend/doc_explorer' },
          { text: 'Collector', link: '/backend/doc_collector' },
        ],
      },
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Hellager' },
    ],
    footer: {
      message: 'Released under the Apache License.',
      copyright: 'Copyright © 2022 present by Steins',
    },

  },
}
