<template>
    <div class="config-container" @click="showConfig=false">
        <div class="dialog">
            config
            <div>
                <label>{{ $t('lang') }}</label>
                <select :value="value" @change="switchLocale">
                    <option value="zh">中文</option>
                    <option value="en">English</option>
                </select>
            </div>
        </div>       
    </div>
</template>
<script setup lang="ts">
import './index.scss';
import {storeToRefs} from 'pinia';
import {useAppStore} from '../../store/app';
import {ref} from 'vue';
import { useI18n } from "vue-i18n";
import { invoke } from '@tauri-apps/api/tauri';
const { locale } = useI18n();

const appStore=useAppStore();

const {showConfig} = storeToRefs(appStore);

const value=ref<string>('zh');

const switchLocale=(e:any)=>{
  const lang=e.target.value;
  locale.value = lang;
  localStorage.setItem("lang", lang);
  value.value=lang;
  invoke('update_menu_lang',{lang:lang});
}

value.value=localStorage.getItem('lang')||'zhCN';
</script>