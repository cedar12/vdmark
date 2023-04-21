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

const {value,isChanged}=storeToRefs(editorStore);

watch(()=>editorStore.source,v=>{
  if(v){
    vditor.value?.setValue(v);
  }
  
})

const vditor = ref<Vditor | null>(null);

onMounted(() => {
  vditor.value = new Vditor('vditor', {
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
      vditor.value!.setValue(editorStore.source||'');
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
});
</script>