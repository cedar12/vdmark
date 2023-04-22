import { defineStore } from 'pinia';

interface AppStore{
  showConfig:boolean,
}

export const useAppStore = defineStore('app', {
  state: ():AppStore => {
    return {
      showConfig:false,
    }
  },
  actions: {
  },
})