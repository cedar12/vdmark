interface Menu{
  file:{
      name:string
      new:string
      open:string
      save:string
      saveas:string
      config:string
    },
    help:{
      name:string
      about:string
    },
    view: {
    name:string
    wysiwyg:string
    ir:string
    sv:string
  }
}

export {Menu}