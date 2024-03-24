<script setup lang="ts">
import { useRoute } from 'vue-router'
import SpinnerComp from '@/components/common/SpinnerComp.vue'
import { AuthTypes } from '@/common/enum/auth'
import { onMounted, ref } from 'vue'
import { userStore } from '@/stores/userStore'
import { storeToRefs } from 'pinia'
import router from '@/router'
import { routes } from '@/common/enum/routes'

const store = userStore()
const { auth2TypesToFetchLogin } = store
const { logged } = storeToRefs(store)

if (logged.value) router.push(routes.USER.HOME)

const route = useRoute()
const error = ref(false)
const message = ref('')

onMounted(async () => {
  try {
    const type = route.query.state
    if (typeof type !== 'string') {
      throw new Error('Wrong redirect')
    }

    const value = AuthTypes[type as keyof typeof AuthTypes]

    if (!value) {
      throw Error('Wrong state parameter in redirect')
    }

    const login = auth2TypesToFetchLogin[value]
    await login()
  } catch (e) {
    error.value = true
    if (e instanceof Error) message.value = e.message
  }
})
</script>

<template>
  <div class="page">
    <SpinnerComp v-if="!error" />
    <div class="error" v-else>
      {{ message }}. Pls return to
      <RouterLink class="link" :to="routes.USER.HOME">home</RouterLink>
      page
    </div>
  </div>
</template>

<style scoped>
.page {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 87vh;
}

.error {
  background: #882a2a;
  font-size: 20px;
  color: #fff;
  box-shadow: 0 5px 5px #000;
  border-radius: 3px;
  padding: 20px 30px;
}

.link {
  color: orange;
}

.link:hover {
  color: #fff;
}

.link:active {
  color: #000;
}
</style>
