<template>
  <div class="about-container" @click="showAbout = false">
    <div class="dialog" @click.stop="">
      <h2><img src="../../assets/32x32.png" /><span>{{ state.buildInfo?.PROJECT_NAME }}</span></h2>
      <ul>
        <li>app version: {{ state.version }}</li>
        <li>tauri version: {{ state.tauriVersion }}</li>
        <li>branch: {{ state.buildInfo?.BRANCH }}</li>
        <li>hash: {{ state.buildInfo?.SHORT_COMMIT }}</li>
        <li>channel: {{ state.buildInfo?.BUILD_RUST_CHANNEL }}</li>
        <li>build time: {{ state.buildInfo?.BUILD_TIME }}</li>
        <li>author: cedar12.zxd@qq.com</li>
        <li>license: MIT</li>
        <li>source: <a :href="sourceHref" @click="toSource">cedar12/vdmark</a></li>
      </ul>
    </div>
  </div>
</template>
<script setup lang="ts">
import { onMounted, reactive } from 'vue';
import { storeToRefs } from 'pinia';
import { useAppStore } from '../../store/app';
import './index.scss';
import { getName, getTauriVersion, getVersion } from '@tauri-apps/api/app';
import { open } from '@tauri-apps/api/shell';
import { invoke } from '@tauri-apps/api';

interface BuildInfo {
  BRANCH:string
  BUILD_OS:string
  BUILD_RUST_CHANNEL:string
  BUILD_TIME:
  string
  CARGO_VERSION:
  string
  COMMIT_AUTHOR:
  string
  COMMIT_DATE
  :
  string
  COMMIT_EMAIL
  :
  string
  COMMIT_HASH
  :
  string
  PKG_VERSION
  :
  string
  PROJECT_NAME
  :
  string
  RUST_CHANNEL
  :
  string
  RUST_VERSION
  :
  string
  SHORT_COMMIT: string
}

const appStore = useAppStore();
const { showAbout } = storeToRefs(appStore);

const sourceHref = "https://github.com/cedar12/vdmark";

const state:{
  tauriVersion:string,
  version:string,
  buildInfo:BuildInfo|null,
} = reactive({
  tauriVersion: '',
  version: '',
  packageVersion: '',
  buildInfo: null,
})

onMounted(async () => {
  state.appName = await getName();
  state.tauriVersion = await getTauriVersion();
  state.version = await getVersion();
  const builds: BuildInfo = await invoke('build_info');
  console.log(builds);
  state.buildInfo = builds;
})

const toSource = (e: MouseEvent) => {
  e.preventDefault();
  e.stopPropagation();
  open(sourceHref);
}

</script>