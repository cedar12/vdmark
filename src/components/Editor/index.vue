<template>
  <div class="editor-container" id="vditor" />
</template>

<script setup lang="ts">
import { ref, onMounted ,watch, onBeforeUnmount} from 'vue';
import Vditor from 'vditor';
import 'vditor/dist/index.css';
import './index.scss';
import {useEditorStore} from '../../store/editor';
import {useAppStore} from '../../store/app';
import {storeToRefs} from 'pinia';
import { open } from '@tauri-apps/api/shell';
import { appWindow } from "@tauri-apps/api/window";
import { UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api';
import { readText } from '@tauri-apps/api/clipboard';

const editorStore=useEditorStore();
const appStore=useAppStore();

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
    counter:{
      enable:true,
    },
    height:window.innerHeight-(appStore.osType==='Windows_NT'?30:0),
    input:(v)=>{
      value.value=v;
      isChanged.value=true;
    },
    toolbarConfig:{
      hide:false,
      pin:false
    },
    toolbar: ['emoji','headings','bold' , 'italic' , 'strike' , '|' , 'line' , 'quote' , 'list' , 'ordered-list' , 'check' ,'outdent' ,'indent' , 'code' , 'inline-code' , 'insert-after' , 'insert-before' ,'undo' , 'redo'  , 'link' , 'table', 'edit-mode' ,'both' , 'preview'  , 'outline' , 'br'],
    after: () => {
      // vditor.value is a instance of Vditor now and thus can be safely used here
      vditor.value!.setValue(defaultValue||'');
      // const toolbar=vditor.value?.vditor.toolbar;
      // const pin=toolbar?.element;
      // if(pin){
      //   document.querySelector('.titlebar-container>.toolbar')?.appendChild(pin);
      // }
      vditor.value?.vditor.toolbar?.element?.setAttribute('data-tauri-drag-region','true');
      // console.log(toolbar);
      
    },
    link:{
      isOpen:false,
      click:(bom)=>{
        console.log(bom);
        const href=bom.textContent?.valueOf();
        if(href){
          if(!href.startsWith('#')){
            open(href);  
          }else{
            console.log(vditor.value?.getCommentIds())
          }
        }
        
      }
    },
    image:{
      preview(bom){
        console.log(bom);
      }
    }
  });
}


const resize=()=>{
  setTimeout(()=>{
    vditor.value?.destroy();
    initEditor(value.value);
  },300);
}


var unlistenDropFileEvent:UnlistenFn;
interface PicgoResp{
  success:boolean,result:Array<String>
}
const onDropFile=async ()=>{
  unlistenDropFileEvent = await appWindow.onFileDropEvent(async (event) => {
    if (event.payload.type === 'hover') {
      console.log('User hovering', event.payload.paths);
    } else if (event.payload.type === 'drop') {
      console.log('User dropped', event.payload.paths);
      event.payload.paths.forEach(async (path)=>{
        let ret:PicgoResp=await invoke('update_picgo',{path});
        if(ret.success){
          vditor.value?.insertValue(`![](${ret.result[0]})`,true);
        }

      })
      
    } else {
      console.log('File drop cancelled');
    }
  });

}

const pasteFileEvent=async (e:KeyboardEvent)=>{
  if(e.code==='KeyV'&&(appStore.osType==='Windows_NT'?e.ctrlKey:e.metaKey)){
    const text=await readText();
    console.log(text);
  }
};

onMounted(() => {
  initEditor(editorStore.source);
  window.addEventListener('resize',resize);
  window.addEventListener('keydown',pasteFileEvent);
  onDropFile();
});

onBeforeUnmount(()=>{
  unlistenDropFileEvent();
  vditor.value?.destroy();
  window.removeEventListener('resize',resize);
  window.removeEventListener('keydown',pasteFileEvent);
})
</script>