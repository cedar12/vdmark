<template>
  <div class="layout">
    <Titlebar/>
    <slot></slot>
  </div>
</template>
<script setup lang="ts">
import Titlebar from './Titlebar/index.vue';
</script>