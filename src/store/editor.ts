import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api/tauri";
import { type } from '@tauri-apps/api/os';

const osType = await type();

interface EditorStore{
  /**
   * 文件路径
   */
  path:string|null,
  /**
   * 文件名称
   */
  fileName:string|null,
  /**
   * 文件原始内容
   */
  source:string|null,
  /**
   * 编辑器的值
   */
  value:string|null,
  /**
   * 内容是否变动
   */
  isChanged:boolean,
}

export const useEditorStore = defineStore('editor', {
  state: ():EditorStore => {
    return {
      path:null,
      fileName:null,
      source:null,
      value:null,
      isChanged:false,
     }
  },
  actions: {
    async openPath(path:string){
      this.path=path;
      let obj=path.lastIndexOf(osType==='Windows_NT'?'\\':'/');

      this.fileName=path.substr(obj+1);
      if(path){
        const content:string = await invoke("read_file", {path});
        this.source=content;
        this.value=content;
        this.isChanged=false;
      }
    }
  },
})