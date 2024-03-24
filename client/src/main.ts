import { createApp } from 'vue'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import './common/icons'

import App from './App.vue'
import router from './router'
import { OhVueIcon } from 'oh-vue-icons'

const app = createApp(App)
const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)

app.use(pinia)
app.use(router)

app.component('v-icon', OhVueIcon)
app.mount('#app')
