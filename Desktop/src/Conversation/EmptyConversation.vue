<script setup lang="ts">
import BrandMark from '../Shared/BrandMark.vue'
import type { HostMode, WorkflowSnapshot } from '../Shared/shell-model'

defineProps<{
  suggestions: readonly string[]
  hostMode: HostMode
  snapshot: WorkflowSnapshot | null
}>()
</script>

<template>
  <section
    v-if="snapshot === null"
    class="empty-conversation"
    aria-labelledby="empty-title"
  >
    <div class="hero-lockup">
      <BrandMark :size="86" />
      <span class="section-label">COPINER / LOCAL HARNESS</span>
    </div>
    <h1 id="empty-title">
      让工作从一句话开始
    </h1>
    <p
      v-if="hostMode === 'ready'"
      class="hero-copy"
    >
      当前版本先把目标、计划、批准和执行过程清楚地跑一遍。
    </p>
    <p
      v-else-if="hostMode === 'browser-preview'"
      class="hero-copy"
    >
      当前是视觉预览。通过 <code>make dev</code> 启动 Tauri 后即可运行本地闭环。
    </p>
    <p
      v-else-if="hostMode === 'error'"
      class="hero-copy"
    >
      本地宿主连接失败，请重启桌面应用。
    </p>
    <p
      v-else
      class="hero-copy"
    >
      正在确认本地运行时状态……
    </p>

    <div class="principle-grid">
      <div>
        <span>01</span>
        <b>本地运行</b>
        <small>任务状态仅存在当前进程</small>
      </div>
      <div>
        <span>02</span>
        <b>批准后执行</b>
        <small>没有批准就不能进入 Execute</small>
      </div>
      <div>
        <span>03</span>
        <b>过程可回看</b>
        <small>每次状态推进都有事件记录</small>
      </div>
    </div>

    <div class="future-scenarios">
      <span class="section-label">NEXT SCENARIOS</span>
      <div>
        <button
          v-for="suggestion in suggestions"
          :key="suggestion"
          disabled
        >
          {{ suggestion }}
          <small>即将开放</small>
        </button>
      </div>
    </div>
  </section>

  <section
    v-else
    class="run-view"
    aria-labelledby="task-title"
  >
    <div class="run-hero">
      <div class="run-identity">
        <BrandMark :size="52" />
        <div>
          <span class="section-label">ACTIVE LOCAL RUN</span>
          <h1 id="task-title">
            {{ snapshot.taskTitle }}
          </h1>
        </div>
      </div>
      <div class="run-state">
        <span
          class="status-light"
          :class="{ complete: snapshot.completed }"
          aria-hidden="true"
        />
        <div>
          <small>当前状态</small>
          <b>{{ snapshot.completed ? '闭环完成' : snapshot.state.label }}</b>
        </div>
      </div>
    </div>

    <div
      v-if="snapshot.completionSummary"
      class="completion-card"
      role="status"
    >
      <span
        class="completion-icon"
        aria-hidden="true"
      >✓</span>
      <div>
        <span class="section-label">RUN COMPLETE</span>
        <b>本地闭环已完成</b>
        <p>{{ snapshot.completionSummary }}</p>
      </div>
    </div>

    <section class="event-panel">
      <div class="panel-heading">
        <div>
          <span class="section-label">EVENT STREAM</span>
          <h2>运行过程</h2>
        </div>
        <span>{{ snapshot.events.length }} EVENTS</span>
      </div>
      <ol
        class="event-stream"
        aria-label="Harness 事件记录"
      >
        <li
          v-for="event in snapshot.events"
          :key="event.sequence"
        >
          <span class="event-index">{{ String(event.sequence).padStart(2, '0') }}</span>
          <div>
            <b>{{ event.actionLabel }}</b>
            <small>{{ event.state.label }}</small>
          </div>
          <span class="event-state">RECORDED</span>
        </li>
      </ol>
    </section>
  </section>
</template>
