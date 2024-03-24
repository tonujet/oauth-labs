<script setup lang="ts">
import InputComp from '@/components/common/InputComp.vue'
import ButtonComp from '@/components/common/ButtonComp.vue'
import { reactive } from 'vue'
import { userStore } from '@/stores/userStore'
import { storeToRefs } from 'pinia'
import SpinnerComp from '@/components/common/SpinnerComp.vue'
import type { LoginDto } from '@/common/interface/auth'
import LinkComp from '@/components/common/LinkComp.vue'
import Auth0Button from '@/components/oauth2/Auth0Button.vue'
import { routes } from '@/common/enum/routes'

const loginDto = reactive<LoginDto>({ login: '', password: '' })
const store = userStore()

const { isError, loading } = storeToRefs(store)
const { login } = store
</script>

<template>
  <div class="page" @keyup.enter="login(loginDto)">
    <form class="form" @submit.prevent>
      <div class="form_title">Login</div>
      <div class="form_info">
        <SpinnerComp v-if="loading" class="form_spinner" />
        <div class="form_error" v-else-if="!loading && isError">Wrong email or password</div>
      </div>
      <div class="form_inputs">
        <InputComp
          class="form_input"
          v-model="loginDto.login"
          placeholder="Email"
          name="email"
          autocomplete="email"
          :disabled="loading"
        />
        <InputComp
          class="form_input"
          v-model="loginDto.password"
          placeholder="Password"
          name="password"
          type="password"
          autocomplete="current-password"
          :disabled="loading"
        />
      </div>
      <div class="form_register">
        <span class="register_title">Don't have an account?</span>
        <LinkComp :to="routes.USER.REGISTER">Sign up</LinkComp>
      </div>
      <div class="form_button">
        <ButtonComp @click="login(loginDto)" :disabled="loading"> Submit</ButtonComp>
      </div>
      <div class="form_bottom">
        <Auth0Button class="form_auth0_button" />
      </div>
    </form>
  </div>
</template>

<style scoped>
.page {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 85vh;
  outline: None;
}

.form {
  padding: 20px 50px;
  background: #46383c;
  display: flex;
  flex-direction: column;
  box-shadow: 0 5px 5px black;
  border-radius: 5px;
  position: relative;
}

.form_title {
  text-align: center;
  margin: 20px 15px 100px;
  font-size: 50px;
  color: white;
}

.form_info {
  position: absolute;
  align-self: center;
  text-align: center;
  top: 129px;
}

.form_error {
  background: #882a2a;
  padding: 10px;
  color: lightgray;
  box-shadow: 0 5px 5px #29232e;
  border-radius: 10px;
}

.form_inputs {
  display: flex;
  flex-direction: column;
}

.form_input {
  width: 350px;
  margin: 12px 0;
}

.form_input:last-child {
  margin-bottom: 5px;
}

.form_button {
  text-align: center;
  margin: 0 20px 70px;
}

.form_bottom {
  display: flex;
  justify-content: center;
}

.form_register {
  display: block;
  color: #ded4d4;
  text-align: right;
  margin-right: 10px;
  margin-bottom: 20px;
}

.register_title {
  margin-right: 10px;
}
</style>
