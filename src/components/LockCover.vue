<template>
    <transition
        name="lock_cover_transition"
        enter-active-class="animate__animated animate__slideInLeft"
        leave-active-class="animate__animated animate__slideOutLeft"
    >
        <div class="cover" v-if="locked" >
            <div class="input">
                <el-input
                    v-model="input" 
                    type="password"
                    :placeholder="t('component.inputPassword')"
                    @change="validate_unlock"
                    show-password/>
            </div>

        </div>
    </transition>    
</template>
<script setup lang="ts">
import { ref, watch } from 'vue'
// import NavigateBar from '../components/NavigateBar.vue';
import { useI18n } from 'vue-i18n';
import { TauriCommand, execute_rust_command } from '../utils';
// import { ElMessage } from 'element-plus';

const { t, locale } = useI18n({ useScope: 'global' });

const props = defineProps({
    isLocked: { type: Boolean, required: true },
});

const emit = defineEmits<{
    (event: 'validate', res: boolean): void
}>()

const input = ref('');
const locked = ref(props.isLocked);

watch(
    props,
    (newProps) => {
        locked.value = newProps.isLocked;        
    }
)

async function validate_unlock() {
    const res = await execute_rust_command(TauriCommand.COMMAND_UNLOCK, input.value);
    if (res) {
        emit('validate', true);
        input.value = '';
        return true;
    } else {
        ElMessage.error({
            showClose: true,
            message: t('error.verifyUnlockFailed'),
            center: true
        })
        input.value = '';
        return false;
    }
}


</script>
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
}

.cover {
    position: absolute;
    display: flex;
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