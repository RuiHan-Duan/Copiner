<script setup lang="ts">
import AppShell from './Layout/AppShell.vue'
import { emptyShellModel } from './Shared/shell-model'
</script>

<template>
  <AppShell :model="emptyShellModel" />
</template>
