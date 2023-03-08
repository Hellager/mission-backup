<script setup lang="ts">
import { onMounted, reactive, ref, toRefs } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import type { FormInstance } from 'element-plus'
import cronstrue from 'cronstrue/i18n'

import { open } from '@tauri-apps/api/dialog'
import { homeDir } from '@tauri-apps/api/path'
import { emit, listen, once } from '@tauri-apps/api/event'

import { storeToRefs } from 'pinia'
import { useI18n } from 'vue-i18n'
import { useMissionStore } from '../store/index'
import PageHeader from '../components/PageHeader.vue'
import Lock from '../components/Lock.vue'

import { TauriCommand, execute_rust_command } from '../utils'

import type { Mission, MissionConfig } from '../store/modules/mission/types'
// import { ElMessage } from 'element-plus';
/**
 * i18n related
 */
const { t, locale } = useI18n({ useScope: 'global' })

/**
 * load pinia shedule store
 */
const missionStore = useMissionStore()

const {
  mission_list,
  current_mission,
  current_drop_mission_path,
} = storeToRefs(missionStore)

const missionFormRef = ref<FormInstance>()

const form = reactive({
  id: '',
  name: '',
  target_type: 'file',
  from_path: '',
  to_path: '',
  ignore_enable: false,
  ignore_method: '.gitignore',
  ignores: [] as string[],
  compress_enable: false,
  compress_format: 'zip',
  cron_enable: false,
  cron_expression: '',
  monitor_enable: true,
  restrict_save_days_enable: false,
  save_days: 3,
  restrict_save_size_enable: false,
  save_size: 25,
  backup_method: 'full',
})

/**
 * if in mode edit, load current mission
 */
const route = useRoute()
const router = useRouter()

const load_current_mission = (data: Mission) => {
  // form = {...data.config};
  form.id = data.config.id
  form.name = data.config.name
  form.target_type = data.config.target_type
  form.from_path = data.config.from_path
  form.to_path = data.config.to_path
  form.ignore_enable = data.config.ignore_enable
  form.ignore_method = data.config.ignore_method
  form.ignores = data.config.ignores
  form.compress_enable = data.config.compress_enable
  form.compress_format = data.config.compress_format
  form.cron_enable = data.config.cron_enable
  form.cron_expression = data.config.cron_expression
  form.monitor_enable = data.config.monitor_enable
  form.restrict_save_days_enable = data.config.restrict_save_days_enable
  form.save_days = data.config.save_days
  form.restrict_save_size_enable = data.config.restrict_save_size_enable
  form.save_size = data.config.save_size
  form.backup_method = data.config.backup_method
}

async function load_drop_mission(path: string) {
  console.log('set form path')
  await execute_rust_command(TauriCommand.COMMAND_GET_DROP_PATH_INFO, path)
    .then((res) => {
      form.from_path = res.data.path
      form.target_type = res.data.meta_type
      form.name = res.data.name
      form.to_path = res.data.save_path
    })
}

const cancel = () => {
  router.push('/table')
}

onMounted(() => {
  if (route.query.mode === 'edit') {
    if (current_mission.value !== undefined)
      load_current_mission(current_mission.value)
    else
      router.push({ path: '/config', query: { mode: 'add' } })
  }
  else if (route.query.mode === 'drop') {
    if (current_drop_mission_path.value !== '' || current_drop_mission_path.value !== undefined)
      load_drop_mission(current_drop_mission_path.value)
    else
      router.push({ path: '/config', query: { mode: 'add' } })
  }
})

const compressFormatOption = [
  {
    value: 'zip',
    label: 'zip',
  },
  {
    value: 'tar',
    label: 'tar',
  },
  {
    value: 'tar.gz',
    label: 'tar.gz',
  },
  {
    value: 'tar.bz2',
    label: 'tar.bz2',
  },
  {
    value: 'tar.xz',
    label: 'tar.xz',
  },
]

const backupMethodOption = [
  {
    value: 'full',
    label: t('config.fullbackup'),
  },
  {
    value: 'incremental',
    label: t('config.incremental'),
  },
  // {
  //     value: 'differential',
  //     label: t('config.differential'),
  // },
]

const ignoresToolTip = ref('')

async function select_path(target: string, from: string) {
  if (from === 'mission') {
    const selected = await open({
      directory: (form.target_type !== 'file' || target === 'to'),
      multiple: false,
      defaultPath: await homeDir(),
    })

    if (selected !== null) {
      if (target === 'from')
        form.from_path = Array.isArray(selected) ? selected[0] : selected
      else if (target === 'to')
        form.to_path = Array.isArray(selected) ? selected[0] : selected
    }
  }
  else if (from === 'ignore_file') {
    const selected = await open({
      directory: false,
      multiple: true,
      defaultPath: await homeDir(),
    })

    if (selected !== null) {
      if (Array.isArray(selected)) {
        selected.forEach((element) => {
          form.ignores.push(element)
          ignoresToolTip.value += (`${element}; \n`)
        })
      }
      else {
        form.ignores.push(selected)
      }

      console.log(ignoresToolTip)
    }
  }
  else if (from === 'ignore_folder') {
    const selected = await open({
      directory: true,
      multiple: true,
      defaultPath: await homeDir(),
    })

    if (selected !== null) {
      if (Array.isArray(selected)) {
        selected.forEach((element) => {
          form.ignores.push(element)
          ignoresToolTip.value += (`${element}; \n`)
        })
      }
      else {
        form.ignores.push(selected)
      }

      console.log(ignoresToolTip)
    }
  }
}

/**
 * validate cron expression
 */
const cron_string = ref('')

const toggle_is_cron = (formEl: FormInstance | undefined) => {
  if (!form.cron_enable && formEl !== undefined) {
    formEl.clearValidate('cron_expression')
    form.cron_expression = ''
  }

  form.monitor_enable = !form.cron_enable
}

const toggle_is_monitor = () => {
  form.cron_enable = !form.monitor_enable
}

const toggle_is_save_days_restrict = (val: any) => {
  form.restrict_save_days_enable = val
}

const toggle_is_save_size_restrict = (val: any) => {
  form.restrict_save_size_enable = val
}

const validate_cron = (rule: any, value: any, callback: any) => {
  if (!form.cron_enable)
    callback()
  if (value === '') {
    cron_string.value = ''
    callback(new Error(t('config.inputCron')))
  }
  else {
    if (form.cron_expression !== '') {
      try {
        const cron_locale = locale.value === 'zh-CN' ? 'zh_CN' : 'en'
        cron_string.value = cronstrue.toString(value, { locale: cron_locale })
      }
      catch (err) {
        cron_string.value = ''
        console.log(err)
        callback(new Error(t('config.inputCron')))
      }
    }
    callback()
  }
}

const handle_change_save_days = (currentValue: number | typeof NaN, oldValue: number | typeof NaN) => {
  if (!isNaN(currentValue)) {
    form.save_days = currentValue
  }
  else {
    ElMessage({
      showClose: true,
      message: t('error.invalidNumber'),
      center: true,
    })
  }
}

const handle_change_save_size = (currentValue: number | typeof NaN, oldValue: number | typeof NaN) => {
  if (!isNaN(currentValue)) {
    form.save_size = currentValue
  }
  else {
    ElMessage({
      showClose: true,
      message: t('error.invalidNumber'),
      center: true,
    })
  }
}

/**
 * Form validate rules
 */
const rules = reactive({
  cron_expression: [{ validator: validate_cron, trigger: 'blur' }],
  name: [{ required: true, message: t('config.missionNameEmpty'), trigger: 'blur' }],
  target_type: [{ required: true, message: '', trigger: 'blur' }],
  from_path: [{ required: true, message: t('config.fromPathEmpty'), trigger: 'change' }],
  to_path: [{ required: true, message: t('config.toPathEmpty'), trigger: 'change' }],
  cron_enable: [{ required: true, message: '', trigger: 'blur' }],
  monitor_enable: [{ required: true, message: '', trigger: 'blur' }],
  backup_method: [{ required: true, message: '', trigger: 'blur' }],
})

async function submit_form(formEl: FormInstance | undefined) {
  if (!formEl)
    return
  formEl.validate((valid) => {
    if (valid) {
      check_valid()
        .then((res) => {
          if (res) {
            submit_to_backend()
              .then((submit_res) => {
                if (submit_res.code === 200) {
                  route.query.mode === 'add' || route.query.mode === 'drop'
                    ? missionStore.append_new_mission(submit_res.data)
                    : missionStore.edit_mission(form.id, submit_res.data)
                  router.push('/table')
                }
              })
          }
        })
    }
    else {
      ElMessage.error({
        showClose: true,
        message: t('config.validFormFailed'),
        center: true,
      })
      return false
    }
  })
}

async function check_exists_before_submit() {
  let isExists = false
  const error_list = []
  const check_list = mission_list.value
  for (let i = 0; i < check_list.length; i++) {
    const mission = check_list[i]
    if (mission.config.id === form.id && route.query.mode === 'edit')
      continue
    if (mission.config.name === form.name) {
      error_list.push({
        showClose: true,
        message: t('config.sameMissionName'),
        center: true,
      })
      isExists = true
    }
    else if (mission.config.from_path === form.from_path) {
      error_list.push({
        showClose: true,
        message: t('config.sameFromPath'),
        center: true,
      })
      isExists = true
    }
    else if (mission.config.to_path === form.to_path) {
      error_list.push({
        showClose: true,
        message: t('config.sameToPath'),
        center: true,
      })
      isExists = true
    }
  }

  error_list.forEach((element) => {
    ElMessage.error(element)
  })

  return isExists
}

async function check_path_valid() {
  const isFromPathValid = await execute_rust_command(TauriCommand.COMMAND_CHECK_PATH_VALID, form.from_path, form.target_type)
    .then((res) => {
      if (!res) {
        ElMessage.error({
          showClose: true,
          message: t('error.fromPathInvalid'),
          center: true,
        })
      }
      return res
    })
  const isToPathValid = await execute_rust_command(TauriCommand.COMMAND_CHECK_PATH_VALID, form.to_path, 'directory')
    .then((res) => {
      if (!res) {
        ElMessage.error({
          showClose: true,
          message: t('error.toPathInvalid'),
          center: true,
        })
      }
      return res
    })

  return (isFromPathValid && isToPathValid)
}

async function check_valid() {
  const isExists = await check_exists_before_submit()
  const isPathValid = await check_path_valid()

  return !isExists && isPathValid
}

async function submit_to_backend() {
  const configuration: MissionConfig = {
    ...form,
    id: route.query.mode === 'add' ? '' : form.id,
  }

  //     let configuration: MissionConfig = {
  //     id: route.query.mode === 'add' ? "" : form.id,
  //     name: form.name,
  //     target_type: form.target_type,
  //     from_path: form.from_path,
  //     to_path: form.to_path,
  //     ignore_enable: form.ignore_enable,
  //     ignore_method: form.ignore_method,
  //     ignores: form.ignores,
  //     compress_enable: form.compress_enable,
  //     compress_format: form.compress_format,
  //     cron_enable: form.cron_enable,
  //     cron_expression: form.cron_expression,
  //     monitor_enable: form.monitor_enable,
  //     restrict_save_days_enable: form.restrict_save_days_enable,
  //     save_days: form.save_days,
  //     restrict_save_size_enable: form.restrict_save_size_enable,
  //     save_size: form.save_size,
  //     backup_method: form.backup_method,
  // }

  // configuration.cron_expression = configuration.cron_expression.replace("?", "*");
  let data: any
  if (route.query.mode === 'add' || route.query.mode === 'drop')
    data = await execute_rust_command(TauriCommand.COMMAND_CREATE_MISSION, configuration)
  else if (route.query.mode === 'edit')
    data = await execute_rust_command(TauriCommand.COMMAND_EDIT_MISSION, form.id, configuration)

  return data
}
</script>

<template>
  <div class="container">
    <PageHeader :title="t('component.pageHeaderMissionConfig')" to="/" />
    <div class="configForm">
      <el-form
        ref="missionFormRef"
        :model="form"
        :rules="rules"
        label-width="132px"
      >
        <el-form-item :label="t('config.name')" prop="name">
          <div class="select">
            <el-input v-model="form.name" clearable />
          </div>
        </el-form-item>
        <el-form-item :label="t('config.type')" prop="target_type">
          <el-radio-group v-model="form.target_type" :disabled="form.from_path !== ''">
            <el-radio label="file" size="large">
              {{ t("config.file") }}
            </el-radio>
            <el-radio label="directory" size="large">
              {{ t("config.directory") }}
            </el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item :label="t('config.fromPath')" prop="from_path">
          <div class="select">
            <el-tooltip effect="light" :content="form.from_path" placement="top" :disabled="form.from_path === ''">
              <el-input v-model="form.from_path" clearable :disabled="route.query.mode === 'edit'" />
            </el-tooltip>
            <el-button type="primary" :disabled="route.query.mode === 'edit'" @click="select_path('from', 'mission')">
              {{ t("config.select") }}
            </el-button>
          </div>
        </el-form-item>
        <el-form-item :label="t('config.toPath')" prop="to_path">
          <div class="select">
            <el-tooltip effect="light" :content="form.to_path" placement="bottom" :disabled="form.to_path === ''">
              <el-input v-model="form.to_path" clearable />
            </el-tooltip>
            <el-button type="primary" @click="select_path('to', 'mission')">
              {{ t("config.select") }}
            </el-button>
          </div>
        </el-form-item>
        <el-form-item :label="t('config.ignoreEnable')">
          <el-switch v-model="form.ignore_enable" :disabled="form.target_type === 'file'" />
        </el-form-item>
        <transition name="fade" enter-active-class="animate__animated animate__fadeInUp" leave-active-class="animate__animated animate__fadeOutDown">
          <el-form-item v-if="form.ignore_enable" :label="t('config.ignoreMethod')">
            <el-radio-group v-model="form.ignore_method">
              <el-radio label="custom" size="large">
                {{ t('config.selectIgnore') }}
              </el-radio>
              <el-radio label=".gitignore" size="large">
                {{ t('config.hasGitignore') }}
              </el-radio>
            </el-radio-group>
          </el-form-item>
        </transition>
        <transition name="fade" enter-active-class="animate__animated animate__fadeInUp" leave-active-class="animate__animated animate__fadeOutDown">
          <el-form-item v-if="form.ignore_method === 'custom' && form.ignore_enable" :label="t('config.selectIgnore')">
            <div class="select">
              <el-tooltip :trigger-keys="[]" :disabled="form.ignore_method !== 'custom' || form.ignores.length === 0" placement="right" effect="light">
                <template #content>
                  <div class="ignoreToolTip">
                    {{ ignoresToolTip }}
                  </div>
                </template>
                <el-input v-model="form.ignores" :autosize="{ minRows: 1, maxRows: 3 }" type="textarea" />
              </el-tooltip>
              <el-button type="primary" @click="select_path('', 'ignore_file')">
                {{ t('config.selectIgnoreFiles') }}
              </el-button>
              <el-button type="primary" @click="select_path('', 'ignore_folder')">
                {{ t('config.selectIgnoreFolders') }}
              </el-button>
            </div>
          </el-form-item>
        </transition>
        <el-form-item :label="t('config.compressEnable')">
          <el-switch v-model="form.compress_enable" />
        </el-form-item>
        <transition name="fade" enter-active-class="animate__animated animate__fadeInUp" leave-active-class="animate__animated animate__fadeOutDown">
          <el-form-item v-if="form.compress_enable" :label="t('config.compressFormat')">
            <el-select v-model="form.compress_format" :disabled="!form.compress_enable" class="m-2" placeholder="zip" size="large">
              <el-option
                v-for="item in compressFormatOption"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
          </el-form-item>
        </transition>
        <el-form-item :label="t('config.cronEnable')" prop="cron_enable">
          <el-switch v-model="form.cron_enable" :disabled="form.monitor_enable" @change="toggle_is_cron(missionFormRef)" />
        </el-form-item>
        <transition name="fade" enter-active-class="animate__animated animate__fadeInUp" leave-active-class="animate__animated animate__fadeOutDown">
          <el-form-item v-if="form.cron_enable" :label="t('config.cronExpression')" prop="cron_expression">
            <el-tooltip :content="cron_string" :trigger-keys="[]" :disabled="!form.cron_enable || cron_string === ''" :show-after="2000" placement="top" effect="light">
              <el-input v-model.trim="form.cron_expression" class="select" :disabled="!form.cron_enable" :placeholder="t('config.inputCron')" />
            </el-tooltip>
          </el-form-item>
        </transition>
        <el-form-item :label="t('config.monitorEnable')" prop="monitor_enable">
          <el-switch v-model="form.monitor_enable" :disabled="form.cron_enable" @change="toggle_is_monitor" />
        </el-form-item>
        <el-form-item :label="t('config.saveDaysEnable')">
          <div class="restrict">
            <el-switch v-model="form.restrict_save_days_enable" @change="toggle_is_save_days_restrict" />
            <!-- <transition name="fade" enter-active-class="animate__animated animate__fadeInUp" leave-active-class="animate__animated animate__fadeOutDown"> -->
            <div class="restrict_item">
              <el-input-number v-if="form.restrict_save_days_enable" v-model="form.save_days" :min="1" :max="365" @change="handle_change_save_days" />
              <span v-if="form.restrict_save_days_enable" class="item_label">{{ t('config.days') }}</span>
            </div>
            <!-- </transition> -->
          </div>
        </el-form-item>
        <el-form-item :label="t('config.saveSizeEnalbe')">
          <div class="restrict">
            <el-switch v-model="form.restrict_save_size_enable" @change="toggle_is_save_size_restrict" />
            <!-- <transition appear name="fade" enter-active-class="animate__animated animate__fadeInRight" leave-active-class="animate__animated animate__fadeOutRight"> -->
            <div class="restrict_item">
              <el-input-number v-if="form.restrict_save_size_enable" v-model="form.save_size" :min="1" :max="1024" @change="handle_change_save_size" />
              <span v-if="form.restrict_save_size_enable" class="item_label">{{ t('config.size') }}</span>
            </div>
            <!-- </transition> -->
          </div>
        </el-form-item>
        <!-- <el-form-item :label="t('config.method')" prop="method">
                    <el-select v-model="form.backup_method" placeholder="full" size="large">
                        <el-option
                        v-for="item in backupMethodOption"
                        :key="item.value"
                        :label="item.label"
                        :value="item.value"
                        />
                    </el-select>
                </el-form-item> -->
        <el-form-item>
          <el-button type="primary" @click="submit_form(missionFormRef)">
            {{ t('config.confirm') }}
          </el-button>
          <el-button @click="cancel">
            {{ t('config.cancel') }}
          </el-button>
        </el-form-item>
      </el-form>
    </div>

    <Lock :tray="['lock', 'home']" />
  </div>
</template>

<style lang="less" scoped>
  @import "../assets/style/theme/default-vars.less";
.container {
    width: 100%;
    min-height: 100vh;
    padding-top: @title-bar-height;
    color: var(--el-color-primary);
    background-color: var(--el-bg-color);
    overflow-y: visiable;
    overflow-x: hidden;
    display:flex;
    flex-direction: column;
}

.configForm {
    margin-top: 32px;
    margin-bottom: 26px;
}

.title {
    padding-top: 20px;
    text-align: center;
}

.action {
    margin-top: 20px;
    text-align: center;
}

.select {
    display: flex;
    flex-direction: row;
    width: 72%;

    .el-input {
        width: 100%;
    }

    .el-button {
        margin-left: 20px;
    }
}

.restrict {
    display: flex;
    flex-direction: row;

    .restrict_item {
        display: flex;
        flex-direction: row;
        margin-left: 6vw;

        .item_label {
            margin-left: 12px;
        }
    }
}

.ignoreToolTip {
    width: 70%;
}
</style>
