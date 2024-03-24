<script setup lang="ts">
defineProps<{ to: string }>()
</script>

<template>
  <RouterLink class="link" :to="to">
    <slot></slot>
  </RouterLink>
</template>

<style scoped>
.link {
  color: #e5dada;
  cursor: pointer;
  font-size: 18px;
}

.link:hover {
  color: #c76136;
}

.link:active {
  color: #fc4a00;
}
</style>
