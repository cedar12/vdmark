<template>
  <div ref="editorRef" class="vdmark-editor-container"  />
</template>

<script setup lang="ts">
import { ref, onMounted ,watch, onBeforeUnmount} from 'vue';
import './index.scss';
import {useEditorStore} from '../../store/editor';
import {useAppStore} from '../../store/app';
import {storeToRefs} from 'pinia';
import { open } from '@tauri-apps/api/shell';
import { appWindow } from "@tauri-apps/api/window";
import { UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api';
import { readText } from '@tauri-apps/api/clipboard';
import {useI18n} from 'vue-i18n';
import { debounce } from '../../utils';
import Editor from './editor';

const editorRef=ref<HTMLElement>();
const editor=ref<Editor>();
const {locale,t}=useI18n();

const editorStore=useEditorStore();
const appStore=useAppStore();

const {value,isChanged,mode,typewriteEnable,counterEnable,lineNumberEnable,codeBlockEnable}=storeToRefs(editorStore);
const {theme,showWorkspace}=storeToRefs(appStore);

onMounted(()=>{
  if(editorRef.value){
    editor.value=new Editor(editorRef.value,{value:"# abc"});
  }
  
})
onBeforeUnmount(()=>{
  editor.value?.destroy();
})
</script>