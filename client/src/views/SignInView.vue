<script setup lang="ts">
import InputComp from '@/components/common/InputComp.vue'
import ButtonComp from '@/components/common/ButtonComp.vue'
import { reactive } from 'vue'
import { userStore } from '@/stores/UserStore'
import { storeToRefs } from 'pinia'
import SpinnerComp from '@/components/common/SpinnerComp.vue'
import type { LoginDto } from '@/common/interface/auth'
import router from '@/router'
import LinkComp from '@/components/common/LinkComp.vue'

const loginDto = reactive<LoginDto>({ login: '', password: '' })
const store = userStore()

const { is_error, loading, logged } = storeToRefs(store)
is_error.value = false
const { login } = store

if (logged.value) router.push('/')


</script>

<template>
  <div class="page" @keyup.enter="login(loginDto)">
    <div class="form">
      <div class="form_title">Login</div>
      <div class="form_info">
        <SpinnerComp v-if="loading" class="form_spinner" />
        <div class="form_error" v-else-if="!loading && is_error">Wrong email or password</div>
      </div>
      <div class="form_inputs">
        <InputComp class="form_input" v-model="loginDto.login" placeholder="Email"/>
        <InputComp class="form_input" v-model="loginDto.password" placeholder="Password" type="password" />
      </div>
      <div class="form_button">
        <ButtonComp @click="login(loginDto)"> Submit</ButtonComp>
      </div>
      <LinkComp to="/sign-up">Sign up</LinkComp>
    </div>
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
  padding: 20px 60px;
  background: #46383c;
  display: flex;
  flex-direction: column;
  box-shadow: 0px 5px 5px black;
  border-radius: 5px;
  position: relative;
}

.form_title {
  text-align: center;
  margin: 20px 15px 130px;
  font-size: 50px;
  color: white;
}

.form_info {
  position: absolute;
  align-self: center;
  text-align: center;
  top: 140px;
}

.form_error {
  background: #882a2a;
  padding: 10px;
  color: lightgray;
  box-shadow: 0 5px 5px #29232e;
  border-radius: 10px;
}

.form_spinner {
}

.form_inputs {
  display: flex;
  flex-direction: column;
}

.form_input {
  width: 350px;
  margin: 10px 0;
}

.form_button {
  text-align: center;
  margin: 20px 20px 0;
}


</style>
