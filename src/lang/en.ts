import { invoke } from "@tauri-apps/api/tauri";
import { Menu } from "./menu";

const menu:Menu=await invoke('get_lang',{lang:'en'});

const en = {
  base:'base',
  lang:'language',
  image:'image',
  theme:'theme',
  addr:'address',
  messages: {
    'setting': 'setting',
    'file': 'file',
    'table': 'table',
  },
  menu:menu,
  pin: 'Always On Top',
}
export default en
