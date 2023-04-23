import { defineStore } from 'pinia';
import { type } from '@tauri-apps/api/os';

interface AppStore{
  showConfig:boolean,
  osType:"Linux" | "Darwin" | "Windows_NT",
  pin:boolean,
}

export const useAppStore = defineStore('app', {
  state: ():AppStore => {
    return {
      showConfig:false,
      osType:'Darwin',
      pin:false,
    }
  },
})