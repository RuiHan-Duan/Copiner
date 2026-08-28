<script setup lang="ts">
const items = [
  { code: '01', label: '当前运行' },
  { code: '02', label: '任务归档' },
  { code: 'SK', label: 'Skills' },
  { code: 'ME', label: '记忆体' },
]
</script>

<template>
  <nav aria-label="主要导航">
    <span class="section-label">NAVIGATION</span>
    <span
      v-for="(item, index) in items"
      :key="item.label"
      class="nav-item"
      :class="{ active: index === 0 }"
    >
      <span class="nav-code">{{ item.code }}</span>
      {{ item.label }}
      <span
        v-if="index === 0"
        class="nav-current"
        aria-hidden="true"
      />
    </span>
  </nav>
</template>
