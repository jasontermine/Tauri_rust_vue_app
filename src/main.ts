import 'vuetify/styles'
import { createApp } from "vue";
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import App from '@/App.vue'
import router from '@/router';
import { createPinia } from 'pinia';

const pinia = createPinia()
const app = createApp(App);

const vuetify = createVuetify({
  components,
  directives,
})

app.use(pinia)
app.use(router);
app.use(vuetify);
app.mount('#app');
