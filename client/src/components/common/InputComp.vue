<script setup lang="ts">
defineProps<{
  disabled: boolean
  modelValue: string
}>()
const emit = defineEmits(['update:modelValue'])
</script>

<template>
  <input
    type="text"
    :disabled="disabled"
    :value="modelValue"
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
