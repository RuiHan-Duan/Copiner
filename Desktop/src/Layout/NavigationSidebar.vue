<script setup lang="ts">
defineProps<{ connectionLabel: string }>()
</script>

<template>
  <aside class="navigation-sidebar">
    <div class="brand">
      <span
        class="coin"
        aria-hidden="true"
      >C</span>
      <div><b>Copiner</b><small>Local-first coworker</small></div>
    </div>
    <button
      class="new-task"
      disabled
    >
      ＋ 新任务
    </button>
    <slot />
    <p class="privacy">
      ● {{ connectionLabel }}
    </p>
  </aside>
</template>
