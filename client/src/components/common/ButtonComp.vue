<script setup lang="ts"></script>

<template>
  <button>
    <slot> </slot>
  </button>
</template>

<style scoped>
button {
  padding: 10px 30px;
  background: #856354;
  box-shadow: 0 4px 5px #29232e;
  border-radius: 7px;
  font-size: 20px;
}

button:hover {
  color: lightgray;
}

button:active {
  transform: translate(0, 1px);
  box-shadow: 0 2px 5px #29232e;
}
</style>
