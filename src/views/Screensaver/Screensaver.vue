<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { useScreensaverStore } from '../../store'

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 * Used for routing within the application.
 */
const router = useRouter()

/**
 * Initializes the screensaver store.
 */
const store = useScreensaverStore()

/**
 * Represents the input password.
 */
const inputPwd = ref('')

/**
 * Handles the confirmation of the input password by attempting to unlock the screensaver.
 * Displays an error message if the password is incorrect, otherwise navigates back and updates the lock status.
 */
async function onInputConfirmed() {
  const res = await store.tryUnlock(inputPwd.value)
  if (!res) {
    ElMessage.error('Wrong password')
    inputPwd.value = ''
  }
  else {
    router.back()
    store.updateLockStatus(false)
  }
}
</script>

<template>
  <transition
    name="lock_cover_transition"
    enter-active-class="animate__animated animate__slideInLeft"
    leave-active-class="animate__animated animate__slideOutLeft"
  >
    <div v-if="true" class="screensaver">
      <div class="screensaver__input">
        <el-form @submit.prevent>
          <el-input
            v-model="inputPwd"
            type="password"
            :placeholder="t('screensaver.inputPwd')"
            show-password
          />
        </el-form>
      </div>

      <div class="screensaver__confirm">
        <el-button type="primary" class="screensaver__confirm" @click="onInputConfirmed">
          {{ t("common.confirm") }}
        </el-button>
      </div>
    </div>
  </transition>
</template>

<style scoped lang="less">
.screensaver {
    position: absolute;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    left: 0;
    top: 25px;
    width: 100%;
    height: 100%;
    z-index: 100;
    -webkit-backdrop-filter: blur(10px);
    backdrop-filter: blur(10px);
}

.screensaver__confirm {
    margin-left: 3px;
}
</style>
