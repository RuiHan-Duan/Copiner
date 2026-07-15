<script setup lang="ts">
defineProps<{ suggestions: readonly string[] }>()
</script>

<template>
  <section
    class="empty-conversation"
    aria-labelledby="empty-title"
  >
    <div
      class="pet"
      aria-hidden="true"
    >
      ◇
    </div>
    <h1 id="empty-title">
      今天想一起完成什么？
    </h1>
    <p>我会先澄清目标、列出 Todo 和 Plan；得到你的确认后才执行。</p>
    <div
      class="suggestions"
      aria-label="任务建议"
    >
      <button
        v-for="suggestion in suggestions"
        :key="suggestion"
        disabled
      >
        {{ suggestion }}
      </button>
    </div>
  </section>
</template>
