<script setup lang="ts">
defineProps<{ providerConfigured: boolean }>()
</script>

<template>
  <footer class="composer">
    <textarea
      aria-label="任务输入"
      placeholder="描述任务；涉及敏感数据时请先标注……"
      :disabled="!providerConfigured"
    />
    <button
      class="send"
      disabled
      aria-label="发送任务"
    >
      ↑
    </button>
    <small v-if="!providerConfigured">尚未配置 Provider，当前仅展示交互骨架</small>
  </footer>
</template>
