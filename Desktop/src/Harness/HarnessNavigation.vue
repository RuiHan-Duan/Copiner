<script setup lang="ts">
const items = ['当前会话', '任务归档', 'Skills', '记忆体']
</script>

<template>
  <nav aria-label="主要导航">
    <span
      v-for="(item, index) in items"
      :key="item"
      :class="{ active: index === 0 }"
    >
      {{ item }}
    </span>
  </nav>
</template>
