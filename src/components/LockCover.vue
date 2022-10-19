<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
// import NavigateBar from '../components/NavigateBar.vue';
import { useI18n } from 'vue-i18n'
import router from '../router'
import { TauriCommand, execute_rust_command } from '../utils'
const props = defineProps({
  isLocked: { type: Boolean, required: true },
  isSetMode: { type: Boolean },
})

const emit = defineEmits<{
  (event: 'validate', res: boolean): void
}>()

// import { ElMessage } from 'element-plus';

const { t, locale } = useI18n({ useScope: 'global' })

const input = ref('')
const locked = ref(props.isLocked)
const placeholder_message = ref(t('component.inputPassword'))

watch(
  props,
  (newProps) => {
    locked.value = newProps.isLocked
    placeholder_message.value = newProps.isSetMode ? t('component.setPassword') : t('component.inputPassword')
  },
)

onMounted(() => {
  placeholder_message.value = props.isSetMode ? t('component.setPassword') : t('component.inputPassword')
})

async function validate_unlock() {
  if (input.value.trim() === '') {
    ElMessage.error({
      showClose: true,
      message: t('error.emptyPassword'),
      center: true,
    })
    input.value = ''
    return false
  }

  if (props.isSetMode) {
    await execute_rust_command(TauriCommand.COMMAND_CHANGE_SETTING_PASSWORD, 'not set yet', input.value)
      .then((res) => {
        if (res) {
          router.push({ path: '/', query: { mode: 'after_set' } })
          return true
        }
        else {
          ElMessage.error({
            showClose: true,
            message: t('error.verifyUnlockFailed'),
            center: true,
          })
          input.value = ''
          return false
        }
      })
  }
  else {
    const res = await execute_rust_command(TauriCommand.COMMAND_UNLOCK, input.value)
    if (res) {
      emit('validate', true)
      input.value = ''
      return true
    }
    else {
      ElMessage.error({
        showClose: true,
        message: t('error.verifyUnlockFailed'),
        center: true,
      })
      input.value = ''
      return false
    }
  }
}
</script>

<template>
  <transition
    name="lock_cover_transition"
    enter-active-class="animate__animated animate__slideInLeft"
    leave-active-class="animate__animated animate__slideOutLeft"
  >
    <div v-if="locked" class="cover">
      <div class="input">
        <el-input
          v-model="input"
          type="password"
          :placeholder="placeholder_message"
          show-password
          @change="validate_unlock"
        />
      </div>
      <el-button type="primary" class="confirm" @change="validate_unlock">
        {{ t("general.confirm") }}
      </el-button>
    </div>
  </transition>
</template>

<style lang="less" scoped>
@import "../assets/style/theme/default-vars.less";
.LockCover {
    display: flex;
    margin: auto;
    justify-content: center;
    align-items: center;
    color: var(--el-color-primary);
    background-color: var(--el-bg-color);
}

.input {
    width: 30%;
    height: 10%;
    margin-right: -5%;
}

.confirm {
  margin-left: 6%;
  margin-top: -6px;
}

.cover {
    position: absolute;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    z-index: 100;
    -webkit-backdrop-filter: blur(10px);
    backdrop-filter: blur(10px);
}
</style>
