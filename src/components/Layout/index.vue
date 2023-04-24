<template>
  <div class="layout">
    <Titlebar />
    <slot></slot>
    <Config v-if="showConfig"/> 
    <About v-if="showAbout"/> 
  </div>
</template>
<script setup lang="ts">
import Titlebar from './Titlebar/index.vue';
import Config from '../Config/index.vue';
import About from '../About/index.vue';
import {useAppStore} from '../../store/app';
import {storeToRefs} from 'pinia';
import { type } from '@tauri-apps/api/os';
import {onMounted, ref} from 'vue';

const appStore=useAppStore();
const {showConfig,showAbout,osType} = storeToRefs(appStore);


onMounted(async ()=>{
  osType.value=await type();
  console.log(osType.value);
})
</script>