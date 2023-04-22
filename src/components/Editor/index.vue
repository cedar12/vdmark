<template>
  <div class="editor-container" id="vditor" />
</template>

<script setup lang="ts">
import { ref, onMounted ,watch} from 'vue';
import Vditor from 'vditor';
import 'vditor/dist/index.css';
import './index.scss';
import {useEditorStore} from '../../store/editor';
import {storeToRefs} from 'pinia';

const editorStore=useEditorStore();

const {value,isChanged,mode}=storeToRefs(editorStore);

watch(()=>editorStore.source,v=>{
  if(v){
    vditor.value?.setValue(v);
  }
})

watch(()=>mode.value,v=>{
  vditor.value?.destroy();
  initEditor(value.value);
})

const vditor = ref<Vditor | null>(null);

const initEditor=(defaultValue:string|null)=>{
  vditor.value = new Vditor('vditor', {
    mode: mode.value,
    height:window.innerHeight,
    input:(v)=>{
      value.value=v;
      isChanged.value=true;
    },
    toolbarConfig:{
      hide:false,
      pin:false
    },
    toolbar: ['emoji','headings','bold' , 'italic' , 'strike' , '|' , 'line' , 'quote' , 'list' , 'ordered-list' , 'check' ,'outdent' ,'indent' , 'code' , 'inline-code' , 'insert-after' , 'insert-before' ,'undo' , 'redo' , 'upload' , 'link' , 'table', 'edit-mode' ,'both' , 'preview'  , 'outline' , 'br'],
    after: () => {
      // vditor.value is a instance of Vditor now and thus can be safely used here
      vditor.value!.setValue(defaultValue||'');
      // const toolbar=vditor.value?.vditor.toolbar;
      // const pin=toolbar?.element;
      // if(pin){
      //   document.querySelector('.titlebar-container>.toolbar')?.appendChild(pin);
      // }
      
      console.log(toolbar);
    },
    link:{
      isOpen:true,
      click:(bom)=>{
      }
    },
  });
}


onMounted(() => {
  initEditor(editorStore.source);
});
</script>