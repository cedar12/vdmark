// import { createApp } from 'vue'
// import App from '../App.vue'
import { createI18n, I18n } from 'vue-i18n'
import en from './en'
import zhCN from './zh'
 
// const app = createApp(App)
const i18n = createI18n({
  legacy: false, 
  locale: localStorage.getItem('lang') || "zhCN", // 初始化显示中文
  messages: {en,zhCN} // 这里就是你有几种语言，对象里面就有几个
})
 
// export default function (app: { use: (arg0: I18n<{ en: any; zhCN: any }, {}, {}, string, false>) => void }) {
//   app.use(i18n)
// }

export default i18n;