<script lang="ts" setup>
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { readText, writeText } from '@tauri-apps/api/clipboard'
import {
  GithubFilled,
  MailOutlined,
  TwitterOutlined,
} from '@vicons/antd'
import { TauriCommand, execute_rust_command } from '../../utils'

const REPO_ADDRESS = 'https://github.com/Hellager/mission-backup'
const EMAIL_ADDRESS = 'steinsgu@gmail.com'
const TWITTER_ADDRESS = 'https://twitter.com/steinssgu'

const { t, locale } = useI18n({ useScope: 'global' })

async function open_an_url(url: string) {
  await execute_rust_command(TauriCommand.COMMAND_OPEN_URL, url)
}

async function copy_to_clipboard(data: string) {
  await writeText(data)

  const current_clipboard = await readText()

  if (data === current_clipboard) {
    ElMessage({
      message: t('setting.copySuccess'),
      showClose: true,
      type: 'success',
    })
  }
}
</script>

<template>
  <div>
    <el-button-group class="contact">
      <el-tooltip :content="t('setting.toGithubRepo')" placement="top" :show-after="1000">
        <el-button type="" text :icon="GithubFilled" @click="open_an_url(REPO_ADDRESS)" />
      </el-tooltip>

      <el-tooltip :content="t('setting.withhEmail')" placement="top" :show-after="1000">
        <el-button type="warning" text :icon="MailOutlined" @click="copy_to_clipboard(EMAIL_ADDRESS)" />
      </el-tooltip>

      <el-tooltip :content="t('setting.toTwitter')" placement="top" :show-after="1000">
        <el-button type="primary" text :icon="TwitterOutlined" @click="open_an_url(TWITTER_ADDRESS)" />
      </el-tooltip>
    </el-button-group>
  </div>
</template>

<style lang="less">
</style>
