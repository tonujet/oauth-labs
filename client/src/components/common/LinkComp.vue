<script setup lang="ts">
const props = defineProps<{ to: string }>()

</script>

<template>
  <RouterLink class="link" :to=to>
    <slot></slot>
  </RouterLink>
</template>

<style scoped>

.link {
  color: #ff4c00;
  text-decoration: None;
  text-align: right;
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