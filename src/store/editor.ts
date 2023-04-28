import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api/tauri";
import { type } from '@tauri-apps/api/os';
import { appWindow } from '@tauri-apps/api/window';

const osType = await type();


interface Path{
  path:string,
  fileName:string,
  openTime:string,
}


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

  /**
   * 模式
   */
  mode:'wysiwyg'|'ir'|'sv',
  /**
   * 启用打字机模式
   */
  typewriteEnable:boolean,
  /**
   *  启用计数器
   */
  counterEnable:boolean,
  codeBlockEnable:boolean,
  lineNumberEnable:boolean,
  paths:Array<Path>,
  autoSaveEnable:boolean,
  rustEnable:boolean,
}

export const useEditorStore = defineStore('editor', {
  state: ():EditorStore => {
    return {
      path:null,
      fileName:null,
      source:null,
      value:null,
      isChanged:false,
      mode:'ir',
      typewriteEnable:false,
      counterEnable:true,
      codeBlockEnable:true,
      lineNumberEnable:false,
      paths:[],
      autoSaveEnable:false,
      rustEnable:false,
     }
  },
  persist: {
		enabled: true,
		strategies: [
			{
				key: 'RDmarkEditor',
				storage: localStorage,
        // paths:['mode','typewriteEnable','paths']
			},
		]
	},
  actions: {
    async openPath(path:string){
      this.path=path;
      
      
      if(path){
        let obj=path.lastIndexOf(osType==='Windows_NT'?'\\':'/');

        this.fileName=path.substr(obj+1);
        appWindow.setTitle(this.fileName);
        this.addPath({
          path:path,
          fileName:this.fileName,
          openTime:new Date().toLocaleString(),
        })
        try{
          const content:string = await invoke("read_file", {path});
          this.source=content;
          this.value=content;
          this.isChanged=false;
        }catch(e){
          console.error(e);
        }
        
      }
    },
    addPath(path:Path){
      const newPaths=this.paths.filter(p=>path.path!==p.path);
      newPaths.unshift(path);
      console.log(newPaths);
      this.paths=newPaths;
    }
  },
})