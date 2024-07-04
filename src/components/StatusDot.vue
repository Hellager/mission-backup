<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'

const props = defineProps({
  status: { type: Number, required: true },
})

/**
 * Used for internationalization.
 */
const { t } = useI18n({ useScope: 'global' })

/**
 *  Contains CSS classes for different status dots.
 */
const dotTag = ['error', 'stopped', 'running', 'default']

/**
 * Contains descriptions for different status types.
 */
const descriptions = [t('status.error'), t('status.stopped'), t('status.running'), t('status.default')]

/**
 * Determines the text to display based on the status prop.
 */
const dottext = computed({
  get: () => {
    if (props.status + 1 > 3)
      return descriptions[3]
    else
      return descriptions[props.status + 1]
  },
  set: (value: string) => value,
})

onMounted(() => {
  if (props.status + 1 > 3)
    dottext.value = descriptions[3]
  else
    dottext.value = descriptions[props.status + 1]
})
</script>

<template>
  <div class="status">
    <span :class="`status__dot ${dotTag[$props.status + 1]}`" />
    <span class="description">{{ dottext }}</span>
  </div>
</template>

<style scoped lang="less">
.status {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    background-color: transparent;
}

.description {
    margin-left: 3px;
}

.status__dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    display: block;
}

.default {
    background-color: #FFFFFF
}

.running {
    background-color: #52c41a
}

.stopped {
    background-color: #faad14;
}

.error {
    background-color: #ff4d4f;
}
</style>
