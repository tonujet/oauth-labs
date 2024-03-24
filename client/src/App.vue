<script setup lang="ts">
import { RouterView } from 'vue-router'
import { onBeforeMount } from 'vue'
import { userStore } from '@/stores/userStore'
import HeaderComp from '@/components/main/HeaderComp.vue'
import FooterComp from '@/components/main/FooterComp.vue'

const { refresh } = userStore()

onBeforeMount(() => {
  refresh()
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
      <FooterComp />
    </footer>
  </div>
</template>

<style>
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
