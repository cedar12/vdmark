<template>
    <div class="config-container" @click="showConfig=false">
        <div class="dialog" @click.stop="">
            <div class="outline">
                <div class="outline-item">
                    <a href="#editor">{{ $t('editor') }}</a>
                </div>
                <div class="outline-item">
                    <a href="#codeBlock">{{ $t('codeBlock.name') }}</a>
                </div>
                <div class="outline-item">
                    <a href="#theme">{{ $t('theme') }}</a>
                </div>
                <div  class="outline-item">
                    <a href="#lang">{{ $t('lang') }}</a>
                </div>
                <div  class="outline-item"><a href="#image">{{ $t('image') }}</a></div>
            </div>
            <div class="content">

                <div>
                    
                    <h2 id="editor">{{ $t('editor') }}</h2>
                    <label>{{$t('autoSave')}}</label>
                    <input type="checkbox" v-model="autoSaveEnable"/>
                    <br/>
                    <label>{{$t('typewriterMode')}}</label>
                    <input type="checkbox" v-model="typewriteEnable"/>
                    <br/>
                    <label>{{$t('counter')}}</label>
                    <input type="checkbox" v-model="counterEnable"/>
                    <br/>
                    <label>{{$t('comment')}}</label>
                    <input type="checkbox" />
                    <span class="tip">仅支持 wysiwyg 模式</span>
                </div>

                <div>
                    <h2 id="codeBlock">{{ $t('codeBlock.name') }}</h2>
                    <label>{{$t('codeBlock.enable')}}</label>
                    <input type="checkbox" v-model="codeBlockEnable"/>
                    <br/>
                    <label>{{$t('codeBlock.lineNumber')}}</label>
                    <input type="checkbox" v-model="lineNumberEnable"/>
                    <br/>
                    <!-- <select name="" id="" >
                        <option>system</option>
                    </select> -->
                </div>
                
                <div>
                    <h2 id="theme">{{ $t('theme') }}</h2>
                    <select name="" id="" v-model="theme" @change="changeTheme">
                        <option value="classic">classic</option>
                        <option value="dark">dark</option>
                        <!-- <option>system</option> -->
                    </select>
                </div>
                <div>
                    <h2 id="lang">{{ $t('lang') }}</h2>
                    <select :value="value" @change="switchLocale">
                        <option value="zh">中文</option>
                        <option value="en">English</option>
                    </select>
                    <span class="tip" style="cursor: pointer;" @click="onRelaunch">{{$t('langTip')}}</span>
                </div>
                <div>
                    <h2 id="image">{{ $t('image') }}</h2>
                    <div>
                        <label>PicGo-Server</label>
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
import {useEditorStore} from '../../store/editor';
import {ref} from 'vue';
import { useI18n } from "vue-i18n";
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow } from '@tauri-apps/api/window';
import { relaunch } from '@tauri-apps/api/process';
import { confirm } from '@tauri-apps/api/dialog';
const { locale } = useI18n();

const appStore=useAppStore();
const editorStore=useEditorStore();

const {showConfig,theme} = storeToRefs(appStore);
const {typewriteEnable,counterEnable,codeBlockEnable,lineNumberEnable,autoSaveEnable} = storeToRefs(editorStore);

const value=ref<string>('zh');

const switchLocale=(e:any)=>{
  const lang=e.target.value;
  locale.value = lang;
  localStorage.setItem("lang", lang);
  value.value=lang;
  //if(appStore.osType!=='Windows_NT'){
    invoke('update_menu_lang',{lang:lang});
  //}
}

value.value=localStorage.getItem('lang')||'zh';

const onRelaunch=async ()=>{
    const confirmed = await confirm('是否重启应用?', '重启rdmark');
    if(confirmed){
        await relaunch();
    }
    
}

const changeTheme=async ()=>{
    if(theme.value){
        document.body?.setAttribute('theme',theme.value||'');
    }else{
        const theme=await appWindow.theme() as any;
        document.body?.setAttribute('theme',theme==='light'?'':theme);
    }
    
}
</script>