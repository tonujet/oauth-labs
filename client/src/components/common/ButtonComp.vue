<script setup lang="ts">
defineProps<{
  disabled: boolean
}>()
</script>

<template>
  <button
    :disabled="disabled"
    :class="{ button: true, active_button: !disabled, disable_button: disabled }"
  >
    <slot></slot>
  </button>
</template>

<style scoped>
.button {
  padding: 10px 30px;

  box-shadow: 0 4px 5px #29232e;
  border-radius: 7px;
  font-size: 20px;
}

.active_button {
  background: #856354;
}

.disable_button {
  background: #564138;
}

.active_button:hover {
  color: lightgray;
}

.active_button:active {
  transform: translate(0, 1px);
  box-shadow: 0 2px 5px #29232e;
}
</style>
