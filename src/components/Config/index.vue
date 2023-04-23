<template>
    <div class="config-container" @click="showConfig=false">
        <div class="dialog" @click.stop="">
            <div class="outline">
                <div>
                    <a href="#theme">{{ $t('theme') }}</a>
                </div>
                <div>
                    <a href="#lang">{{ $t('lang') }}</a>
                </div>
                <div><a href="#image">{{ $t('image') }}</a></div>
            </div>
            <div class="content">
                <div>
                    <h2 id="theme">{{ $t('theme') }}</h2>
                    <select >
                        <option value="dark">dark</option>
                        <option value="classic">classic</option>
                        <option value="system">system</option>
                    </select>
                </div>
                <div>
                    <h2 id="theme">打字机</h2>
                    <label>是否启用打字机模式</label>
                    <input type="checkbox" />
                </div>
                <div>
                    <h2 id="lang">{{ $t('lang') }}</h2>
                    <select :value="value" @change="switchLocale">
                        <option value="zh">中文</option>
                        <option value="en">English</option>
                    </select>
                </div>
                <div>
                    <h2 id="image">{{ $t('image') }}</h2>
                    <div>
                        PicGo-Server
                    </div>
                    <div>
                        <label>{{ $t('addr') }}</label>
                        <input placeholder="127.0.0.1:36667"/>
                    </div>
                </div>
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
  if(appStore.osType!=='Windows_NT'){
    invoke('update_menu_lang',{lang:lang});
  }
}

value.value=localStorage.getItem('lang')||'zh';
</script>