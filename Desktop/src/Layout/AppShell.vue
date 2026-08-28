<script setup lang="ts">
import EmptyConversation from '../Conversation/EmptyConversation.vue'
import MessageComposer from '../Conversation/MessageComposer.vue'
import TaskInspector from '../TaskFlow/TaskInspector.vue'
import type { HostMode, ShellViewModel, WorkflowSnapshot } from '../Shared/shell-model'
import HarnessNavigation from '../Harness/HarnessNavigation.vue'
import NavigationSidebar from './NavigationSidebar.vue'
import WorkspaceHeader from './WorkspaceHeader.vue'

defineProps<{
  model: ShellViewModel
  hostMode: HostMode
  snapshot: WorkflowSnapshot | null
  busy: boolean
  errorMessage: string
}>()

defineEmits<{
  start: [taskTitle: string]
  advance: [action: string]
  reset: []
}>()
</script>

<template>
  <main class="shell">
    <NavigationSidebar
      :connection-label="model.connectionLabel"
      :can-reset="hostMode === 'ready' && snapshot !== null"
      @reset="$emit('reset')"
    >
      <HarnessNavigation />
    </NavigationSidebar>
    <section class="workspace">
      <WorkspaceHeader
        :active-stage="model.activeStage"
        :state-label="snapshot?.state.label ?? '等待任务'"
      />
      <EmptyConversation
        :suggestions="model.suggestions"
        :host-mode="hostMode"
        :snapshot="snapshot"
      />
      <MessageComposer
        :host-mode="hostMode"
        :session-active="snapshot !== null"
        :busy="busy"
        @start="$emit('start', $event)"
      />
    </section>
    <TaskInspector
      :snapshot="snapshot"
      :busy="busy"
      :error-message="errorMessage"
      @advance="$emit('advance', $event)"
    />
  </main>
</template>
