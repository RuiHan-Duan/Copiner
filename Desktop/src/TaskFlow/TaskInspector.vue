<script setup lang="ts">
import type { WorkflowSnapshot } from '../Shared/shell-model'

defineProps<{
  snapshot: WorkflowSnapshot | null
  busy: boolean
  errorMessage: string
}>()
defineEmits<{ advance: [action: string] }>()
</script>

<template>
  <aside class="task-inspector">
    <div class="inspector-heading">
      <div>
        <span class="section-label">RUN CONTROL</span>
        <h2>任务控制</h2>
      </div>
      <span
        class="control-state"
        :class="{ active: snapshot && !snapshot.completed }"
      >{{ snapshot ? (snapshot.completed ? 'DONE' : 'ACTIVE') : 'IDLE' }}</span>
    </div>

    <section class="action-card">
      <span class="section-label">NEXT ACTION</span>
      <template v-if="snapshot?.nextAction">
        <b>{{ snapshot.nextAction.label }}</b>
        <p v-if="snapshot.nextAction.requiresApproval">
          此动作只批准推进当前本地内存状态。
        </p>
        <p v-else>
          由 CoreEngine 校验当前状态后推进。
        </p>
        <button
          class="advance"
          :class="{ approval: snapshot.nextAction.requiresApproval }"
          :disabled="busy"
          @click="$emit('advance', snapshot.nextAction.id)"
        >
          {{ busy ? '处理中…' : snapshot.nextAction.label }}
          <span aria-hidden="true">→</span>
        </button>
      </template>
      <template v-else>
        <b>{{ snapshot?.completed ? '运行已完成' : '等待任务' }}</b>
        <p>{{ snapshot?.completed ? '可以新建任务重新开始。' : '输入任务后，这里会显示唯一合法动作。' }}</p>
      </template>
    </section>

    <section class="todo-panel">
      <div class="panel-heading compact">
        <h3>Todo</h3>
        <span>{{ snapshot ? `${Math.min(snapshot.events.length - 1, 3)}/3` : '0/3' }}</span>
      </div>
      <ul class="task-list">
        <li :class="{ done: snapshot && snapshot.events.length > 1 }">
          <span aria-hidden="true">{{ snapshot && snapshot.events.length > 1 ? '✓' : '1' }}</span>
          确认目标与演示边界
        </li>
        <li :class="{ done: snapshot && snapshot.events.length > 4 }">
          <span aria-hidden="true">{{ snapshot && snapshot.events.length > 4 ? '✓' : '2' }}</span>
          通过动作级批准
        </li>
        <li :class="{ done: snapshot?.completed }">
          <span aria-hidden="true">{{ snapshot?.completed ? '✓' : '3' }}</span>
          验收并归档运行摘要
        </li>
      </ul>
    </section>

    <p
      v-if="errorMessage"
      class="command-error"
      role="alert"
    >
      {{ errorMessage }}
    </p>

    <div class="guard">
      <span aria-hidden="true">✓</span>
      <div>
        <b>安全护栏已启用</b>
        <small>无模型、无工具、无外部副作用</small>
      </div>
    </div>
  </aside>
</template>
