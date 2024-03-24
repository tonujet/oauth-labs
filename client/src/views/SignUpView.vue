<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { userStore } from '@/stores/userStore'
import SpinnerComp from '@/components/common/SpinnerComp.vue'
import ButtonComp from '@/components/common/ButtonComp.vue'
import InputComp from '@/components/common/InputComp.vue'
import { reactive } from 'vue'
import type { RegisterDto } from '@/common/interface/auth'
import LinkComp from '@/components/common/LinkComp.vue'
import { routes } from '@/common/enum/routes'
import Auth0Button from '@/components/oauth2/Auth0Button.vue'

const store = userStore()

const { isError, loading } = storeToRefs(store)
const { register } = store
const registerDto = reactive<RegisterDto>({ email: '', password: '', username: '' })
</script>

<template>
  <div class="page" @keyup.enter="register(registerDto)">
    <form class="form" @submit.prevent>
      <div class="form_title">Registration</div>
      <div class="form_info">
        <SpinnerComp v-if="loading" class="form_spinner" />
        <div class="form_error" v-else-if="!loading && isError">Wrong inputs</div>
      </div>
      <div class="form_inputs">
        <InputComp
          class="form_input"
          v-model="registerDto.email"
          placeholder="Email"
          name="email"
          autocomplete="none"
          :disabled="loading"
        />
        <InputComp
          class="form_input"
          v-model="registerDto.username"
          placeholder="Username"
          name="username"
          autocomplete="none"
          :disabled="loading"
        />
        <InputComp
          class="form_input"
          v-model="registerDto.password"
          placeholder="Password"
          name="password"
          type="password"
          autocomplete="none"
          :disabled="loading"
        />
      </div>
      <div class="form_register">
        <span class="register_title">Have an account?</span>
        <LinkComp :to="routes.USER.LOGIN">Sign in</LinkComp>
      </div>
      <div class="form_button">
        <ButtonComp @click="register(registerDto)" :disabled="loading"> Submit </ButtonComp>
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
  margin: 20px 15px 110px;
  font-size: 50px;
  color: white;
}

.form_inputs {
  display: flex;
  flex-direction: column;
}

.form_input {
  width: 350px;
  margin: 10px 0;
}

.form_input:last-child {
  margin-bottom: 5px;
}

.form_button {
  text-align: center;
  margin: 0 20px 70px;
}

.form_info {
  position: absolute;
  align-self: center;
  text-align: center;
  top: 133px;
}

.form_error {
  background: #882a2a;
  padding: 10px;
  color: lightgray;
  box-shadow: 0 5px 5px #29232e;
  border-radius: 10px;
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
