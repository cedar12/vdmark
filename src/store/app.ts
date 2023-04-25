import { defineStore } from 'pinia';
import { type } from '@tauri-apps/api/os';
import { P } from '@tauri-apps/api/event-2a9960e7';

interface Path{
  path:string,
  fileName:string,
  openTime:string,
}

interface AppStore{
  showConfig:boolean,
  showAbout:boolean,
  showWorkspace:boolean,
  osType:"Linux" | "Darwin" | "Windows_NT",
  pin:boolean,
  theme:"classic"|"dark"|undefined,
  paths:Array<Path>
}

export const useAppStore = defineStore('app', {
  state: ():AppStore => {
    return {
      showConfig:false,
      showAbout:false,
      showWorkspace:false,
      osType:'Darwin',
      pin:false,
      theme:'classic',
      paths:[],
    }
  },
  actions:{
    addPath(path:Path){
      const newPaths=this.paths.filter(p=>path.path!==p.path);
      newPaths.unshift(path);
      this.paths=newPaths;
    }
  },
  persist: {
		enabled: true,
		strategies: [
			{
				key: 'RDmarkApp',
				storage: localStorage
			},
		]
	}
})