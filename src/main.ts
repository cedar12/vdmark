import { createApp } from "vue";
import "./styles.scss";
import App from "./App.vue";
import {createPinia} from 'pinia';
import piniaPluginPersist from 'pinia-plugin-persist';
import i18n from './lang/index';

const pinia = createPinia()
pinia.use(piniaPluginPersist)

createApp(App).use(pinia).use(i18n).mount("#app");
