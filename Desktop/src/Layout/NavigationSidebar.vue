<script setup lang="ts">
import BrandMark from '../Shared/BrandMark.vue'

defineProps<{ connectionLabel: string; canReset: boolean }>()
defineEmits<{ reset: [] }>()
</script>

<template>
  <aside class="navigation-sidebar">
    <div class="brand-lockup">
      <BrandMark
        :size="48"
        label="Copiner"
      />
      <div class="brand-wordmark">
        <b>Copiner</b>
        <small>local agent workspace</small>
      </div>
    </div>

    <section
      class="workspace-card"
      aria-label="当前工作区"
    >
      <span class="section-label">WORKSPACE</span>
      <div>
        <b>本地工作区</b>
        <span>PRIVATE</span>
      </div>
    </section>

    <button
      class="new-task"
      :disabled="!canReset"
      @click="$emit('reset')"
    >
      <span aria-hidden="true">＋</span>
      新建任务
    </button>

    <slot />

    <div class="runtime-card">
      <span
        class="runtime-dot"
        aria-hidden="true"
      />
      <div>
        <b>{{ connectionLabel }}</b>
        <small>当前运行仅保存在本地内存</small>
      </div>
    </div>
  </aside>
</template>
