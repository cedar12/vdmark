<template>
  <div class="layout">
    <Titlebar />
    <div class="layout-content" :class="{doc:showWorkspace}">
      <Workspace v-if="showWorkspace"></Workspace>
      <slot></slot>
    </div>
    <Config v-if="showConfig"></Config>
    <About v-if="showAbout"></About> 
  </div>
</template>
<script setup lang="ts">
import Titlebar from './Titlebar/index.vue';
import Config from '../Config/index.vue';
import About from '../About/index.vue';
import Workspace from '../Workspace/index.vue';
import {useAppStore} from '../../store/app';
import {storeToRefs} from 'pinia';
import { type } from '@tauri-apps/api/os';
import {onMounted} from 'vue';

const appStore=useAppStore();
const {showConfig,showAbout,showWorkspace,osType} = storeToRefs(appStore);


onMounted(async ()=>{
  osType.value=await type();
  console.log(osType.value);
})
</script>
<style lang="scss">
.layout{
  
  .layout-content{
    height: calc(100vh - var(--titlebarHeight));
    display: flex;
    flex-wrap: nowrap;
    position: relative;
    &.doc{
      .vditor-content{
        width: calc(100vw - var(--docListWidth));
      }
    }
    .workpsace-btn{
      position: absolute;
      top: 0;
      left: 0;
      .item{
        height: 36px;
        width: 36px;
      }
      
    }
  }
  
}
</style>