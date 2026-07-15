<script setup lang="ts">
import { workflowStages, type WorkflowStageId } from '../Shared/shell-model'

defineProps<{ activeStage: WorkflowStageId }>()
</script>

<template>
  <header class="workspace-header">
    <ol
      class="progress"
      aria-label="任务阶段"
    >
      <li
        v-for="(stage, index) in workflowStages"
        :key="stage.id"
        :class="{ on: stage.id === activeStage }"
      >
        {{ index + 1 }} {{ stage.label }}
      </li>
    </ol>
    <button disabled>
      设置
    </button>
  </header>
</template>
