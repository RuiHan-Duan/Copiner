<script setup lang="ts">
import { computed } from 'vue'

import BrandMark from '../Shared/BrandMark.vue'
import { workflowStages, type WorkflowStageId } from '../Shared/shell-model'

const props = defineProps<{ activeStage: WorkflowStageId; stateLabel: string }>()
const activeIndex = computed(() => workflowStages.findIndex(stage => stage.id === props.activeStage))
</script>

<template>
  <header class="workspace-header">
    <div class="header-context">
      <BrandMark
        class="mobile-brand"
        :size="36"
      />
      <div>
        <span class="section-label">CURRENT WORKFLOW</span>
        <b>{{ stateLabel }}</b>
      </div>
    </div>

    <ol
      class="progress"
      aria-label="任务阶段"
    >
      <li
        v-for="(stage, index) in workflowStages"
        :key="stage.id"
        :class="{
          on: stage.id === activeStage,
          done: index < activeIndex,
        }"
      >
        <span>{{ String(index + 1).padStart(2, '0') }}</span>
        {{ stage.label }}
      </li>
    </ol>
  </header>
</template>
