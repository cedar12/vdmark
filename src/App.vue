<template>
  <Layout>
    <Editor/>
  </Layout>
</template>

<script setup lang="ts">
import Layout from './components/Layout/index.vue';
import Editor from './components/Editor/index.vue';
// 禁止右键和检查
//禁止F12
document.onkeydown = function (event: any) {
    var winEvent: any = window.event
    if (winEvent && winEvent.keyCode == 123) {
        event.keyCode = 0
        event.returnValue = false
    }
    if (winEvent && winEvent.keyCode == 13) {
        winEvent.keyCode = 505
    }
}
 
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