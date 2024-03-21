<script setup lang="ts">
import { RouterView } from 'vue-router'
import { onMounted } from 'vue'
import { userStore } from '@/stores/UserStore'
import HeaderComp from '@/components/main/HeaderComp.vue'

const { refresh } = userStore()

onMounted(async () => {
  await refresh()
})
</script>

<template>
  <div class="main-wrapper">
    <header>
      <HeaderComp />
    </header>
    <main class="main">
      <RouterView class="main_router" />
    </main>
    <footer>
      <!--            footer-->
    </footer>
  </div>
</template>

<style>
header,
footer {
  display: block;
}

.main-wrapper {
  background-color: #29232e;
  display: flex;
  flex-direction: column;
  min-height: 100%;
}

.main {
  flex: 1 1 auto;
}

.main_router {
  height: 100%;
}
</style>
