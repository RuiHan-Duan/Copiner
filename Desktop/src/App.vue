<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'

import AppShell from './Layout/AppShell.vue'
import {
  advanceLocalHarness,
  commandErrorMessage,
  getLocalHarnessSnapshot,
  getRuntimeStatus,
  hasDesktopHost,
  resetLocalHarness,
  startLocalHarness,
} from './Shared/local-harness-client'
import { emptyShellModel, type HostMode, type WorkflowSnapshot } from './Shared/shell-model'

const hostMode = ref<HostMode>('checking')
const snapshot = ref<WorkflowSnapshot | null>(null)
const busy = ref(false)
const errorMessage = ref('')

const model = computed(() => ({
  ...emptyShellModel,
  activeStage: snapshot.value?.completed
    ? 'archive' as const
    : snapshot.value?.state.stage ?? 'clarify' as const,
  connectionLabel: hostMode.value === 'ready'
    ? '本地 Harness · 未连接模型'
    : hostMode.value === 'browser-preview'
      ? '浏览器预览 · 需 Tauri'
      : hostMode.value === 'error'
        ? '本地宿主不可用'
        : '正在检查本地 Harness',
}))

onMounted(async () => {
  if (!hasDesktopHost()) {
    hostMode.value = 'browser-preview'
    return
  }

  try {
    await getRuntimeStatus()
    snapshot.value = await getLocalHarnessSnapshot()
    hostMode.value = 'ready'
  } catch (error) {
    hostMode.value = 'error'
    errorMessage.value = commandErrorMessage(error)
  }
})

async function startTask(taskTitle: string) {
  await runCommand(async () => {
    snapshot.value = await startLocalHarness(taskTitle)
  })
}

async function advanceTask(action: string) {
  await runCommand(async () => {
    snapshot.value = await advanceLocalHarness(action)
  })
}

async function resetTask() {
  await runCommand(async () => {
    await resetLocalHarness()
    snapshot.value = null
  })
}

async function runCommand(command: () => Promise<void>) {
  busy.value = true
  errorMessage.value = ''
  try {
    await command()
  } catch (error) {
    errorMessage.value = commandErrorMessage(error)
  } finally {
    busy.value = false
  }
}
</script>

<template>
  <AppShell
    :model="model"
    :host-mode="hostMode"
    :snapshot="snapshot"
    :busy="busy"
    :error-message="errorMessage"
    @start="startTask"
    @advance="advanceTask"
    @reset="resetTask"
  />
</template>
