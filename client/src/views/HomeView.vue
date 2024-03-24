<script setup lang="ts">
import { userStore } from '@/stores/userStore'
import { storeToRefs } from 'pinia'
import { ref } from 'vue'
import { fetchUserInfo } from '@/service/auth'
import SpinnerComp from '@/components/common/SpinnerComp.vue'
import { routes } from '@/common/enum/routes'

const { user, logged } = storeToRefs(userStore())
const { logout } = userStore()

const guest_token = ref('')
const creation_date = ref('')
const loading = ref(false)

const get_creation_date = () => {
  loading.value = true
  return fetchUserInfo(guest_token.value)
    .then((user) => {
      creation_date.value = new Date(user.created_at).toDateString()
    })
    .catch(() => {
      creation_date.value = 'Invalid token :('
    })
    .finally(() => (loading.value = false))
}
</script>

<template>
  <div class="page" tabindex="0" @keyup.enter="get_creation_date()">
    <div class="block">
      <div class="block_panel">
        <div class="block_user" v-if="logged">Hello {{ user?.nickname }} ({{ user?.email }})</div>
        <div class="block_guest" v-else>
          <div class="block_greeting">Hello Guest</div>
          <div class="block_desc">Try guess the correct token :)</div>
          <div>
            <div class="input_label">Access token</div>
            <div class="input_block">
              <input v-model="guest_token" class="input" type="text" />
            </div>
          </div>
          <div class="input_button_block" @keyup.enter.prevent="get_creation_date()">
            <button @click="get_creation_date()" class="input_button">
              Get account creation date
            </button>
          </div>
        </div>
      </div>
      <div class="block_status">
        <div v-if="logged" class="status_user">
          <div class="status_tag user_tag">User</div>
          <div class="user_created_at">
            {{ user ? new Date(user.created_at).toDateString() : '' }}
          </div>
          <div>
            <span class="block_link" @click="logout()">Logout</span>
          </div>
        </div>
        <div v-else class="status_guest">
          <div class="status_tag guest_tag">Stranger</div>
          <div class="user_created_at_block">
            <SpinnerComp class="user_created_at_spinner" v-if="loading" />
            <div class="user_created_at" v-else>{{ creation_date }}</div>
          </div>
          <div class="status_links">
            <RouterLink class="block_link" :to="routes.USER.LOGIN">Sign-in</RouterLink>
            <RouterLink class="block_link" :to="routes.USER.REGISTER">Sign-up</RouterLink>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.page {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 87vh;
  color: #fff;
  outline: none;
}

.block {
  background: #46383c;
  padding: 20px;
  box-shadow: 0px 5px 5px black;
  border-radius: 5px;
}

.block_desc {
  margin-bottom: 60px;
}

.block_panel {
  padding: 100px 70px 70px;
  font-size: 2em;
}

.block_status {
  font-size: 20px;
}

.status_user,
.status_guest {
  display: flex;
  justify-content: space-between;
  align-items: end;
  height: 50px;
}

.block_link {
  font-size: 20px;
  color: #fff;
  cursor: pointer;
  transition: 0.1s;
  display: block;
  padding: 5px 3px 0;
}

.block_link:hover {
  color: #e00;
}

.block_link:active {
  color: #360000;
}

.status_links {
  margin-left: 10px;
}

.status_tag {
  border-radius: 3px;
  box-shadow: 0 1px 2px black;
  color: #fff;
  font-weight: 400;
  padding: 3px 15px;
}

.guest_tag {
  background: #a20000;
}

.user_tag {
  background: green;
}

.input_label {
  font-size: 20px;
  margin-bottom: 3px;
  color: #c98f73;
  text-align: center;
}

.input {
  padding: 10px 15px;
  font-size: 20px;
  width: 100%;
  border-radius: 3px;
  box-shadow: 0 1px 3px black;
}

.input_block {
  margin-bottom: 10px;
}

.input_button {
  display: block;
  padding: 7px;
  font-size: 18px;
  cursor: pointer;
  color: black;
  margin: 0 auto;
  background: #c98f73;
  border-radius: 5px;
  box-shadow: 0 1px 3px black;
  width: 70%;
  transition: 0.2s;
}

.input_button:hover {
  background: #885f4d;
}

.input_button:active {
  background: #fff;
}

.user_created_at {
  color: #ff7300;
}

.user_created_at_block {
}
</style>
