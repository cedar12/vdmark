<template>
  <div class="workspace-container">
    <div class="workspace-header">
      <input v-model="search" @change="onSearch" placeholder="查找文档"/>
    </div>
    <div class="workspace-content">
      <div class="workspace-item" v-for="item in list" :key="item.path" @click="openPath(item.path)" :title="item.path">
        <div class="item-name">{{ item.fileName }}</div>
        <div class="item-content">{{ item.openTime }}</div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { useEditorStore } from '../../store/editor';
import {storeToRefs} from 'pinia';
import {ref,watch} from 'vue';
import { dialog } from '@tauri-apps/api';
import {useI18n} from 'vue-i18n';

const {t} = useI18n();

const editorStore=useEditorStore();
const {paths}=storeToRefs(editorStore);

const search=ref<string>();
const list=ref<Array<any>>(paths.value);

watch(()=>paths.value,(v)=>{
  list.value=v;
})

const openPath=async (path:string)=>{
  if(editorStore.isChanged){
    const c=await dialog.confirm(editorStore.fileName+t('noSaveTip'),t('tip'));
    if(!c){
      return;
    }
  }
  editorStore.openPath(path);
}

const onSearch=()=>{
  if(search.value&&search.value!==''){
    list.value=list.value.filter(l=>l.fileName.indexOf(search.value)>-1);
  }else{
    list.value=paths.value;
  }
}
</script>
<style lang="scss">
.workspace-container{
  width: var(--docListWidth);
  background-color: var(--backgroundColor2);
  border-right: .1em solid var(--hoverBackgroundColor);
  height: 100%;
  
  box-sizing: border-box;
  .workspace-header{
    height: 36px;
    
    box-sizing: border-box;
    border-bottom: .1em solid var(--hoverBackgroundColor);
    padding: 12px;
    display: flex;
    justify-content: center;
    align-items: center;
    input{
      outline: none;
      border: .1em solid var(--hoverBackgroundColor);
      width: 100%;
      height: 100%;
      padding: 6px;
      background-color: var(--backgroundColor2);
    }
  }
  .workspace-content{
    height: calc(100% - 36px);
    overflow: auto;
    .workspace-item{
      border-bottom: .1em solid var(--hoverBackgroundColor);
      padding: 12px;
      cursor: pointer;
      .item-name{
        font-size: 16px;
        color: var(--textColor);
      }
      .item-content{
        font-size: 12px;
        color: darken(rgb(131, 131, 131),10%);
      }
      &:hover{
        background-color: var(--hoverBackgroundColor);
      }
    }
  }
  
}
</style>