<script setup lang="ts">
withDefaults(defineProps<{
  size?: number
  label?: string
}>(), {
  size: 40,
  label: '',
})
</script>

<template>
  <svg
    class="brand-mark"
    :width="size"
    :height="size"
    viewBox="0 0 64 64"
    :aria-label="label || undefined"
    :aria-hidden="label ? undefined : 'true'"
    :role="label ? 'img' : undefined"
  >
    <rect
      x="2"
      y="2"
      width="60"
      height="60"
      rx="15"
      class="brand-mark__surface"
    />
    <rect
      x="2.75"
      y="2.75"
      width="58.5"
      height="58.5"
      rx="14.25"
      fill="none"
      class="brand-mark__border"
      stroke-width="1.5"
    />
    <path
      d="M48.3 19.1C44.5 14.1 38.6 11 31.8 11C20 11 10.5 20.4 10.5 32C10.5 43.6 20 53 31.8 53C38.6 53 44.5 49.9 48.3 44.9"
      fill="none"
      class="brand-mark__shadow"
      stroke-width="9"
      stroke-linecap="square"
      transform="translate(1.5 1.5)"
    />
    <path
      d="M48.3 19.1C44.5 14.1 38.6 11 31.8 11C20 11 10.5 20.4 10.5 32C10.5 43.6 20 53 31.8 53C38.6 53 44.5 49.9 48.3 44.9"
      fill="none"
      class="brand-mark__letter"
      stroke-width="9"
      stroke-linecap="square"
    />
    <path
      d="M43.7 21.6L51.5 15.4M43.7 42.4L51.5 48.6"
      fill="none"
      class="brand-mark__terminal"
      stroke-width="4"
      stroke-linecap="square"
    />
  </svg>
</template>
