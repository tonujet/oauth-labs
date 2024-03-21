<script setup lang="ts">

import { storeToRefs } from 'pinia'
import router from '@/router'
import { userStore } from '@/stores/UserStore'
import SpinnerComp from '@/components/common/SpinnerComp.vue'
import ButtonComp from '@/components/common/ButtonComp.vue'
import InputComp from '@/components/common/InputComp.vue'
import { reactive } from 'vue'
import type { RegisterDto } from '@/common/interface/auth'
import LinkComp from '@/components/common/LinkComp.vue'

const store = userStore()

const { is_error, loading, logged } = storeToRefs(store)
const { register } = store
is_error.value = false
const registerDto = reactive<RegisterDto>({ email: '', password: '', username: '' })

if (logged.value) router.push('/')


</script>

<template>
  <div class="page" @keyup.enter="register(registerDto)">
    <div class="form">
      <div class="form_title">Registration</div>
      <div class="form_info">
        <SpinnerComp v-if="loading" class="form_spinner" />
        <div class="form_error" v-else-if="!loading && is_error">Wrong inputs</div>
      </div>
      <div class="form_inputs">
        <InputComp class="form_input" v-model="registerDto.email" placeholder="Email" />
        <InputComp class="form_input" v-model="registerDto.username" placeholder="Username" />
        <InputComp class="form_input" v-model="registerDto.password" placeholder="Password" type="password" />
      </div>
      <div class="form_button">
        <ButtonComp @click="register(registerDto)"> Submit</ButtonComp>
      </div>
      <LinkComp to="/sign-in">Sign in</LinkComp>
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
  box-shadow: 0 5px 5px black;
  border-radius: 5px;
  position: relative;
}

.form_title {
  text-align: center;
  margin: 20px 15px 130px;
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

.form_button {
  text-align: center;
  margin: 20px 20px 5px;
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
</style>
