<script setup lang="ts">
import router from '@/router'
import { routes } from '@/common/enum/routes'
</script>

<template>
  <button @click="router.push(routes.OAUTH2.AUTH0.REDIRECT)">
    <v-icon class="icon" name="co-auth0" scale="1.8" />
    <span class="label">Continue with auth0</span>
  </button>
</template>

<style scoped>
button {
  padding: 5px 13px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-radius: 5px;
  box-shadow: 0 3px 6px black;
  cursor: pointer;
}

.icon {
  margin-right: 10px;
}

.label {
  font-size: 22px;
}

button:hover {
  background: #cec8c8;
}

button:active {
  transform: translateY(1px);
  box-shadow: 0 2px 5px black;
}
</style>
