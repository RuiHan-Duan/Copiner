<script setup lang="ts">
import { ref } from 'vue'

import type { HostMode } from '../Shared/shell-model'

const props = defineProps<{
  hostMode: HostMode
  sessionActive: boolean
  busy: boolean
}>()
const emit = defineEmits<{ start: [taskTitle: string] }>()
const taskTitle = ref('')

function submit() {
  const value = taskTitle.value.trim()
  if (!value || props.hostMode !== 'ready' || props.sessionActive || props.busy) return
  emit('start', value)
  taskTitle.value = ''
}
</script>

<template>
  <form
    class="composer"
    @submit.prevent="submit"
  >
    <div class="composer-heading">
      <label for="task-title-input">描述一个任务</label>
      <span>LOCAL RUN</span>
    </div>
    <textarea
      id="task-title-input"
      v-model="taskTitle"
      maxlength="120"
      placeholder="例如：验证 Copiner Harness 闭环"
      :disabled="hostMode !== 'ready' || sessionActive || busy"
    />
    <div class="composer-footer">
      <small v-if="hostMode === 'ready' && !sessionActive">不会调用模型或工具，仅推进本地状态</small>
      <small v-else-if="sessionActive">当前任务运行中，请在 Run Control 推进</small>
      <small v-else>浏览器只提供视觉预览，完整运行请使用 Tauri</small>
      <button
        class="send"
        type="submit"
        :disabled="hostMode !== 'ready' || sessionActive || busy || !taskTitle.trim()"
      >
        开始运行
        <span aria-hidden="true">↗</span>
      </button>
    </div>
  </form>
</template>
