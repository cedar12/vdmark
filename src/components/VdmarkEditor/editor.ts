import { invoke } from "@tauri-apps/api/tauri";

interface EditorOptions{
  editable?:boolean,
  value?:string,
}

const defaultOptions:EditorOptions={
  editable:true,
  value:''
}

export default class Editor{
  private el:HTMLElement;
  private pre:HTMLElement;
  private options:EditorOptions;
  constructor(el:HTMLElement,options?:EditorOptions){
    this.el=el;
    if(options){
      this.options=options;
    }else{
      this.options=defaultOptions;
    }
    this.pre=document.createElement('pre');
    this.pre.className='vdmark-editor';
    this.pre.setAttribute('contenteditable',`${this.options.editable||true}`);
    this.el.appendChild(this.pre);
    document.execCommand('defaultParagraphSeparator', false, 'p')
    this.parse();
    this.event();
  }

  private event(){
    this.pre.addEventListener('input',(e:InputEvent)=>{
      console.log(e);
      if(e.inputType==='insertParagraph'&&e.data===null){
        const selection = window.getSelection();
        const node=selection?.anchorNode;
        const el=node as HTMLElement;
        el.focus();
      }
      
    })
    this.pre.addEventListener('change',e=>{
      console.log(e);
    })
  }

  private async parse(){
    const html=await invoke('md2html',{md:this.options.value});
    if(html){
      this.pre.innerHTML=html as string;
      
    }
  }

  private parseHtml(html:string){
    return new Promise(async (resolve,reject)=>{
      const md=await invoke('html2md',{html});
      if(md){
        resolve(md);
      }else{
        reject(new Error(''));
      }
    })
    
  }


  

  public getSelection(){
    const selection = window.getSelection();
    console.log(selection);
    if (selection&&selection.type === "Range") {
      const range = selection.getRangeAt(0);
      const startNode = range.startContainer;
      const endNode = range.endContainer;
      const startOffset = range.startOffset;
      const endOffset = range.endOffset;
      console.log(range,startNode,endNode);
    }else{
      const node=selection?.anchorNode;
      let element=node?.parentElement;
      this.parseHtml(element?.parentElement?.innerHTML||'').then(res=>{
        if(element){
          // element.innerHTML=res as string;
        }
        
        // console.log(res);
      })
    }

  }

  public destroy(){

  }
}