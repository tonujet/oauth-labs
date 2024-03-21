<script setup lang="ts">
import { userStore } from '@/stores/UserStore'
import { storeToRefs } from 'pinia'
import { ref } from 'vue'
import { fetch_user_info } from '@/service/auth'
import SpinnerComp from '@/components/common/SpinnerComp.vue'

const { user, logged } = storeToRefs(userStore())
const { logout } = userStore()

const guest_token = ref('')

const creation_date = ref('')
const loading_date = ref(false)
const get_creation_date = () => {
  loading_date.value = true
  return fetch_user_info(guest_token.value)
    .then((user) => {
      creation_date.value = new Date(user.created_at).toDateString()
    })
    .catch((e) => {
      creation_date.value = 'Invalid token :('
      console.log(e)
    }).finally(() => loading_date.value = false)
}
</script>

<template>
  <div class="page" tabindex="0" @keyup.enter="get_creation_date()">
    <div class="block">
      <div class="block_panel">
        <div class="block_user" v-if="logged">Hello {{ user ? user.name : 'unknown' }}</div>
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
            <SpinnerComp class="user_created_at_spinner" v-if="loading_date" />
            <div class="user_created_at" v-else>{{ creation_date }}</div>
          </div>
          <div>
            <RouterLink class="block_link" to="/sign-in">Sign-in</RouterLink>
            /
            <RouterLink class="block_link" to="/sign-up">Sign-up</RouterLink>
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
  margin-bottom: 70px;
}

.block_panel {
  padding: 100px 100px 80px;
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
  color: #ff4c00;
  text-decoration: None;
  cursor: pointer;
}

.block_link:hover {
  color: #c76136;
}

.block_link:active {
  color: #80665b;
}

.status_tag {
  padding: 2px 5px;
  border-radius: 3px;
  box-shadow: 0 1px 2px black;
  color: #fff;
  font-weight: 400;
}

.guest_tag {
  background: red;
}

.user_tag {
  background: green;
}

.input_label {
  font-size: 18px;
  color: #ff7300;
}

.input {
  padding: 6px;
  font-size: 16px;
  width: 100%;
  border-radius: 3px;

  box-shadow: 0 1px 3px black;
}

.input_block {
  margin-bottom: 10px;
  margin-top: -8px;
}

.input_button {
  display: block;
  padding: 5px 10px;
  font-size: 18px;
  cursor: pointer;
  color: black;
  background: #ff7300;
  border-radius: 5px;
  box-shadow: 0 1px 3px black;
  width: 100%;
}

.input_button:hover {
  background: #ff9100;
}

.input_button:active {
  background: #ff7300;
}

.user_created_at {
  color: #ff7300;
}

.user_created_at_block {
  margin-left: 40px;
}

</style>
