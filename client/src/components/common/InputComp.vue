<script setup lang="ts">
const props = defineProps(['modelValue'])
const emit = defineEmits(['update:modelValue'])
</script>

<template>
  <input
    type="text"
    :value="props.modelValue"
    @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
  />
</template>

<style scoped>
input {
  padding: 13px 20px;
  box-shadow: 0 7px 5px #29232e;
  border-radius: 5px;
  font-size: 20px;
}
</style>
