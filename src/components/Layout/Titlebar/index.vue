<template>
<div data-tauri-drag-region class="titlebar-container"  v-if="osType==='Windows_NT'">
  <div data-tauri-drag-region class="drag-region">
    <div data-tauri-drag-region class="file-name" :title="path||''">
      {{ fileName }}{{ isChanged?' *':'' }}
    </div>
  </div>
  
  <Menu @click="onClickMenu" ></Menu>
  <div class="titlebar">
    <div class="titlebar-button" @click="onPin" :class="{pin:pin}" :title="$t('pin')">
      <Pin></Pin>
    </div>
    <div class="titlebar-button" id="titlebar-minimize" @click="appWindow.minimize()">
      <Minus></Minus>
    </div>
    <div class="titlebar-button" id="titlebar-maximize" @click="appWindow.toggleMaximize()">
      <Browser></Browser>
    </div>
    <div class="titlebar-button" id="titlebar-close" @click="appWindow.close()">
      <Close></Close>
    </div>
  </div>
  
</div>
</template>
<script setup lang="ts">
import {appWindow} from '@tauri-apps/api/window';
import {open,save} from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";
import './index.scss';
import {useEditorStore} from '../../../store/editor';
import {storeToRefs} from 'pinia';
import Menu from './Menu.vue';
import { emit, listen, UnlistenFn } from '@tauri-apps/api/event';
import { useAppStore } from '../../../store/app';
import { onBeforeUnmount, onMounted } from 'vue';
import {Pin,Close,Minus,Browser} from '@icon-park/vue-next';
import {useI18n} from 'vue-i18n';

const {t} = useI18n();
const editorStore=useEditorStore();
const appStore=useAppStore();


const {fileName,path,isChanged,source,value,mode}=storeToRefs(editorStore);
const {showConfig,showAbout,pin,osType}=storeToRefs(appStore);

const onPin=async ()=>{
  pin.value=!pin.value;
  await invoke('pin',{pin:pin.value});
}

const openFileDialog=async ()=>{
  // const path:string = await invoke("open_file_dialog", {});
  const path=await open({
    multiple:false,
    filters:[{
      name:"Markdown",
      extensions:['md']
    }]
  })
  if(path&&typeof path === 'string'){
    console.log('打开文件',path);
    await editorStore.openPath(path);
  }
}
const saveFile=async ()=>{
  if(path.value){
    // console.log(editorStore.value);
    let err:string=await invoke('save_file',{path:editorStore.path,content:editorStore.value});
    if(!err){
      isChanged.value=false;
    }else{
      console.error(err);
    }
  }else{
    saveAsFile(undefined);
  }
}

const saveAsFile=async (isNew:boolean|undefined)=>{
  const filePath = await save({
    filters: [{
      name: 'Markdown',
      extensions: ['md']
    }],
    title:isNew?t('menu.file.new'):t('menu.file.saveas')
  });
  if(filePath){
    let err:string=await invoke('save_file',{path:filePath,content:editorStore.value});
    if(!err){
      isChanged.value=false;
      editorStore.openPath(filePath);
    }else{
      console.error(err);
    }
  }
  
}
const newFile=async ()=>{
  source.value=null;
  value.value=null;
  await saveAsFile(true);
}

const onClickMenu=async (key:string)=>{
  
  switch(key){
    case 'file.new':
      await newFile();
      break;
    case 'file.open':
      await openFileDialog();
      break;
    case 'file.save':
      await saveFile();
      break;
    case 'file.saveas':
      await saveAsFile(undefined);
      break;
    case 'file.config':
      showConfig.value=!showConfig.value;
      break;
    case 'view.wysiwyg':
      mode.value='wysiwyg';
      break;
    case 'view.ir':
      mode.value='ir';
      break;
    case 'view.sv':
      mode.value='sv';
      break;
    case 'view.top':
      await onPin();
      break;
    case 'help.about':
      showAbout.value=!showAbout.value;
      break;
  }
}


onMounted(async ()=>{
  appWindow.onMenuClicked(menuId=>{
    onClickMenu(menuId.payload);
  })
})

onBeforeUnmount(()=>{
  
})
</script>