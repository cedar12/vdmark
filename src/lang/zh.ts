import { invoke } from "@tauri-apps/api/tauri";
import { Menu } from "./menu";

const menu:Menu=await invoke('get_lang',{lang:'zh'});

const zhCN = {
  base:'基础',
  editor:'编辑器',
  typewriterMode:'打字机模式',
  lang:'语言',
  image:'图片',
  theme:'主题',
  addr:'地址',
  counter:'计数器',
  comment:'评论模式',
  messages: {
    'setting': '设置',
    'file': '文件',
    'table': '表格',
  },
  menu:menu,
  pin:'固定最上层窗口',
}
export default zhCN
