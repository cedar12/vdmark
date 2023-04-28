<template>
  <Layout>
    <Editor v-if="!editorStore.rustEnable"/>
    <VdmarkEditor v-else></VdmarkEditor>
  </Layout>
</template>

<script setup lang="ts">
import Layout from './components/Layout/index.vue';
import Editor from './components/Editor/index.vue';
import VdmarkEditor from './components/VdmarkEditor/index.vue';
import {listen} from '@tauri-apps/api/event';
import { useI18n } from 'vue-i18n';
import {useEditorStore} from './store/editor';
import {storeToRefs} from 'pinia';

const editorStore=useEditorStore();


const {locale} = useI18n();



// 禁止右键和检查
//禁止F12
// document.onkeydown = function (event: any) {
//     var winEvent: any = window.event
//     if (winEvent && winEvent.keyCode == 123) {
//         event.keyCode = 0
//         event.returnValue = false
//     }
//     // if (winEvent && winEvent.keyCode == 13) {
//     //     winEvent.keyCode = 505
//     // }
// }

listen('lang',e=>{
    console.log('lang',e);
    const payload=e.payload as string;
    
    locale.value=payload;
})

 
//屏蔽右键菜单
document.oncontextmenu = function (event: any) {
    if (window.event) {
        event = window.event
    }
    try {
        var the = event.srcElement
        if (
            !(
                (the.tagName == 'INPUT' && the.type.toLowerCase() == 'text') ||
                the.tagName == 'TEXTAREA'
            )
        ) {
            return false
        }
        return true
    } catch (e) {
        return false
    }
}
</script>

<style></style>