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
import {useI18n} from 'vue-i18n';

const {locale,t}=useI18n();

const editorStore=useEditorStore();
const appStore=useAppStore();

const {value,isChanged,mode,typewriteEnable,counterEnable}=storeToRefs(editorStore);
const {theme,showWorkspace}=storeToRefs(appStore);

watch(()=>editorStore.source,v=>{
  // if(v){
  //   // vditor.value?.setValue(v);
    
  // }
  vditor.value?.destroy();
  initEditor(value.value);
})

watch(()=>mode.value,v=>{
  vditor.value?.destroy();
  initEditor(value.value);
})


watch(()=>locale.value,v=>{
  vditor.value?.destroy();
  initEditor(value.value);
})
watch(()=>typewriteEnable.value,v=>{
  vditor.value?.destroy();
  initEditor(value.value);
})
watch(()=>counterEnable.value,v=>{
  vditor.value?.destroy();
  initEditor(value.value);
})

watch(()=>theme.value,(t:any)=>{
  vditor.value?.setTheme(t);
})

const vditor = ref<Vditor | null>(null);

const toolbar=[
{
      hotkey: '⇧⌘F',
      name: 'doc-list',
      tipPosition: 'ne',
      tip: t('docList'),
      className: 'right',
      icon: '<?xml version="1.0" encoding="UTF-8"?><svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M42 4H6V14H42V4Z" fill="none"  stroke-width="4" stroke-linejoin="round"/><path d="M42 19H6V29H42V19Z" fill="none"  stroke-width="4" stroke-linejoin="round"/><path d="M42 34H6V44H42V34Z" fill="none"  stroke-width="4" stroke-linejoin="round"/><path d="M21 9H27"  stroke-width="4" stroke-linecap="round"/><path d="M21 24H27"  stroke-width="4" stroke-linecap="round"/><path d="M21 39H27"  stroke-width="4" stroke-linecap="round"/></svg>',
      click () {
        showWorkspace.value=!showWorkspace.value;
      },
    },
'headings','bold' , 'italic' , 'strike'  , 'line' , 'quote'  , 'check', 'link' , 'table' , '|','indent','outdent'  , 'insert-after' , 'insert-before', 'outline' ,
{
  name: 'more',
  toolbar: [
    'undo' , 'redo'  ,
    'emoji',
    'list' , 'ordered-list',
    'code' , 'inline-code',
    'edit-mode',
    'both',
    'fullscreen',
    'preview',
    'help',
  ],
},
]

const initEditor=async (defaultValue:string|null)=>{
  let lang:any=locale.value==='en'?`${locale.value}_US`:`${locale.value}_CN`;
  let editorTheme=theme.value;
  if(!editorTheme){
    let t=await appWindow.theme();
    editorTheme=t==='light'?'classic':'dark';
  }
  vditor.value = new Vditor('vditor', {
    mode: mode.value,
    theme:editorTheme,
    typewriterMode:typewriteEnable.value,
    lang:lang,
    counter:{
      enable:counterEnable.value,
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
    toolbar: toolbar,
    value:defaultValue||'',
    after: () => {
      // vditor.value is a instance of Vditor now and thus can be safely used here
      // vditor.value!.setValue(defaultValue||'');
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
        // console.log(bom);
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
        if(path.toLowerCase().endsWith('.md')){
          await editorStore.openPath(path);
        }else if(path.toLowerCase().endsWith('.jpg')||path.toLowerCase().endsWith('.png'||path.toLowerCase().endsWith('gif')||path.toLowerCase().endsWith('ico'))){
          let ret:PicgoResp=await invoke('update_picgo',{path});
          if(ret.success){
            vditor.value?.insertValue(`![](${ret.result[0]})`,true);
          }
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
  document.body?.setAttribute('theme',theme.value||'');
  initEditor(editorStore.source);
  window.addEventListener('resize',resize);
  window.addEventListener('keydown',pasteFileEvent);
  onDropFile();
});

onBeforeUnmount(()=>{
  if(unlistenDropFileEvent){
    unlistenDropFileEvent();
  }
  vditor.value?.destroy();
  window.removeEventListener('resize',resize);
  window.removeEventListener('keydown',pasteFileEvent);
})
</script>