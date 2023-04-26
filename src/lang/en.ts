import { invoke } from "@tauri-apps/api/tauri";
import { Menu } from "./menu";

const menu:Menu=await invoke('get_lang',{lang:'en'});

const en = {
  base:'base',
  editor:'Edtior',
  typewriterMode:'Typewriter Mode',
  lang:'Language',
  langTip:'Restart Rdmark',
  image:'Image',
  theme:'Theme',
  addr:'Address',
  counter:'Counter',
  comment:'Comment',
  docList:'Document List',
  autoSave:'Auto Save',
  noSaveTip:'Give up to save?',
  tip:'tip',
  codeBlock:{
    name:'Code Block',
    enable:'Code Highlighting',
    lineNumber:'Line Number',
  },
  messages: {
    'setting': 'setting',
    'file': 'file',
    'table': 'table',
  },
  menu:menu,
  pin: 'Always On Top',
}
export default en
