import html2canvas from "html2canvas";

type DebouncedFunction<F extends (...args: any[]) => void> = (this: ThisParameterType<F>, ...args: Parameters<F>) => void;

export function debounce<F extends (...args: any[]) => void>(func: F, delay: number=300): DebouncedFunction<F> {
  let timerId: ReturnType<typeof setTimeout> | null;
  
  return function(this: ThisParameterType<F>, ...args: Parameters<F>) {
    if (timerId) {
      clearTimeout(timerId);
    }
    
    timerId = setTimeout(() => {
      func.apply(this, args);
      timerId = null;
    }, delay);
  } as DebouncedFunction<F>;
}



export function toImage(element:HTMLElement,windowHeight:number,opt?:any):Promise<HTMLCanvasElement>{
  const scrollTop=element.scrollTop;
  const all=document.createElement('canvas');
  all.width=element.clientWidth;
  all.height=element.scrollHeight;
  const ctx=all.getContext('2d');
  const partCount=all.height/windowHeight+Math.ceil(all.height/windowHeight%1);
  return new Promise(async (resolve,reject)=>{
    try{
      for(let i=0;i<=partCount;i++){
        let top=i*windowHeight;
        let offset=all.height-top;
        element.scrollTop=top;
        let canvas=await html2canvas(element,{
          backgroundColor: null,
          useCORS: true,
          y:offset<windowHeight?windowHeight-offset:0,
          height:offset<windowHeight?offset:windowHeight,
        });
        const tempCtx=canvas.getContext('2d');
        var heightPos=0;
        if(i==partCount){
          heightPos=all.height-i*windowHeight;
        }
        const imgData=tempCtx?.getImageData(0,heightPos,canvas.width,canvas.height);
        if(imgData){
          ctx?.putImageData(imgData,0,top);
        }
      }
      
      //const imgData = canvas.toDataURL('image/jpeg', 1.0);
      element.scrollTop=scrollTop;
      resolve(all);
    }catch(e){
      reject(e);
    }
    
  })
}