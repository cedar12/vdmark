<template>
  <div class="about-container" @click="showAbout=false">
    <div class="dialog" @click.stop="">
      <h1><img src="../../assets/32x32.png"/><span>{{ state.appName }}</span></h1>
      <ul>
        <li>app version: {{ state.version }}</li>
        <li>tauri version: {{ state.tauriVersion }}</li>
        <li>package version: {{ state.packageVersion }}</li>
        <li>build time: {{ new Date(state.buildTime).toLocaleString() }}</li>
        <li>author: cedar12.zxd@qq.com</li>
        <li>license: MIT</li>
        <li>source: <a :href="sourceHref" @click="toSource">cedar12/rdmark</a></li>
      </ul>
    </div>
  </div>
</template>
<script setup lang="ts">
import {onMounted, reactive} from 'vue';
import {storeToRefs} from 'pinia';
import {useAppStore} from '../../store/app';
import './index.scss';
import { getName,getTauriVersion,getVersion } from '@tauri-apps/api/app';
import { open } from '@tauri-apps/api/shell';


const appStore=useAppStore();
const {showAbout} = storeToRefs(appStore);

const sourceHref="https://github.com/cedar12/rdmark";

const state=reactive({
  appName:'rdmark',
  tauriVersion:'',
  version:'',
  packageVersion:window.__PACKAGE_VERSION__,
  buildTime:window.__BUILD_TIME__,
})

onMounted(async ()=>{
  state.appName = await getName();
  state.tauriVersion = await getTauriVersion();
  state.version = await getVersion();
  
})

const toSource=(e:MouseEvent)=>{
  e.preventDefault();
  e.stopPropagation();
  open(sourceHref);
}

</script>