<template>
  <div class="layout">
    <Titlebar/>
    <slot></slot>
    <Config v-if="showConfig"/> 
  </div>
</template>
<script setup lang="ts">
import Titlebar from './Titlebar/index.vue';
import Config from '../Config/index.vue';
import {useAppStore} from '../../store/app';
import {storeToRefs} from 'pinia';

const appStore=useAppStore();
const {showConfig} = storeToRefs(appStore);
</script>