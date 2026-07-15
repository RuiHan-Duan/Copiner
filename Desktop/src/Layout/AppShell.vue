<script setup lang="ts">
import EmptyConversation from '../Conversation/EmptyConversation.vue'
import MessageComposer from '../Conversation/MessageComposer.vue'
import TaskInspector from '../TaskFlow/TaskInspector.vue'
import type { ShellViewModel } from '../Shared/shell-model'
import HarnessNavigation from '../Harness/HarnessNavigation.vue'
import NavigationSidebar from './NavigationSidebar.vue'
import WorkspaceHeader from './WorkspaceHeader.vue'

defineProps<{ model: ShellViewModel }>()
</script>

<template>
  <main class="shell">
    <NavigationSidebar :connection-label="model.connectionLabel">
      <HarnessNavigation />
    </NavigationSidebar>
    <section class="workspace">
      <WorkspaceHeader :active-stage="model.activeStage" />
      <EmptyConversation :suggestions="model.suggestions" />
      <MessageComposer :provider-configured="model.providerConfigured" />
    </section>
    <TaskInspector />
  </main>
</template>
