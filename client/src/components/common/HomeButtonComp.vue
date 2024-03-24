<script setup lang="ts">
import { routes } from '@/common/enum/routes'
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const isHovered = ref(false)
const isClicked = ref(false)
const router = useRouter()

const onMouseUp = () => {
  isClicked.value = false
  router.push(routes.USER.HOME)
}
</script>

<template>
  <div
    class="link"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
    @mouseup="onMouseUp"
    @mousedown="isClicked = true"
    @mouseout="isClicked = false"
    @click="console.log('click')"
  >
    <v-icon class="icon" name="oi-home" scale="3" v-if="!isHovered && !isClicked" />
    <v-icon class="icon" name="ri-home-5-fill" scale="3" v-else-if="isHovered && !isClicked" />
    <v-icon class="icon" name="ri-home-5-fill" scale="3" fill="#ff7300" v-else />
  </div>
</template>

<style scoped>
.link {
  text-decoration: None;
  display: block;
  padding: 10px;
  color: #fff;
}
</style>
