import { defineStore } from 'pinia';
import { type } from '@tauri-apps/api/os';
import { P } from '@tauri-apps/api/event-2a9960e7';

interface AppStore{
  showConfig:boolean,
  showAbout:boolean,
  showWorkspace:boolean,
  osType:"Linux" | "Darwin" | "Windows_NT",
  pin:boolean,
  theme:"classic"|"dark"|undefined,
  
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