import { createApp } from "vue";
import "./styles.scss";
import App from "./App.vue";
import {createPinia} from 'pinia';
import i18n from './lang/index';

createApp(App).use(createPinia()).use(i18n).mount("#app");
