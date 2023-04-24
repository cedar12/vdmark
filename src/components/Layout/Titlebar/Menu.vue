<script setup lang="ts">
import {ref,reactive,defineEmits, onMounted, onBeforeUnmount} from 'vue';
import {useI18n} from 'vue-i18n';

const emit=defineEmits(['click']);
const {t,locale}=useI18n();

interface MenuItem{
  key:string,
  name:string,
  submenu:Array<SubMenuItem>
}
interface SubMenuItem{
  key:string,
  name:string,
  cmd:Array<string>,
}

const menus=reactive<Array<MenuItem>>([
  {
    key:'file',
    name:t('menu.file.name'),
    submenu:[
      {
        key:'file.new',
        name:t('menu.file.new'),
        cmd:['Ctrl','N'],
      },
      {
        key:'file.open',
        name:t('menu.file.open'),
        cmd:['Ctrl','Shift','O'],
      },
      {
        key:'file.save',
        name:t('menu.file.save'),
        cmd:['Ctrl','S'],
      },
      {
        key:'file.saveas',
        name:t('menu.file.saveas'),
        cmd:['Ctrl','Alt','S'],
      },
      {
        key:'file.config',
        name:t('menu.file.config'),
        cmd:[],
      }
    ]
  },
  {
    key:'view',
    name:t('menu.view.name'),
    submenu:[
      {
        key:'view.wysiwyg',
        name:t('menu.view.wysiwyg'),
        cmd:['Ctrl','Alt','7'],
      },
      {
        key:'view.ir',
        name:t('menu.view.ir'),
        cmd:['Ctrl','Alt','8'],
      },
      {
        key:'view.sv',
        name:t('menu.view.sv'),
        cmd:['Ctrl','Alt','9'],
      }
    ]
  },
  {
    key:'help',
    name:t('menu.help.name'),
    submenu:[
      {
        key:'help.about',
        name:t('menu.help.about'),
        cmd:[],
      }
    ]
  }
]);

const active=ref<string|null>(null);

const onClickMenu=(key:string)=>{
  active.value=null;
  emit('click',key);
}

const keydownMenu=(e:KeyboardEvent)=>{
  const isCtrl=e.ctrlKey;
  const isAlt=e.altKey;
  const isShift=e.shiftKey;
  const code=e.code;
  for(let i=0;i<menus.length;i++){
    let menu=menus[i].submenu;
    for(let k=menu.length-1;k>=0;k--){
      let cmd=menu[k].cmd;
      let exc=[];
      for(let j=0;j<cmd.length;j++){
        if(cmd[j]==='Alt'&&isAlt===true){
          exc[j]=true;
        }else if(exc&&cmd[j]==='Ctrl'&&isCtrl===true){
          exc[j]=true;
        }else if(exc&&cmd[j]==='Shift'&&isShift===true){
          exc[j]=true;
        }else if(`Key${cmd[j]}`===code){
          exc[j]=true;
        }else{
          exc[j]=false;
        }
      }
      let isExc=true;
      exc.forEach(e=>{
        if(e===false){
          isExc=false;
        }
      })
      if(exc.length>0&&isExc===true){
        emit('click',menu[k].key);
        return;
      }
    }
  }
}

onMounted(()=>{
  window.addEventListener('keydown',keydownMenu);
})

onBeforeUnmount(()=>{
  window.removeEventListener('keydown',keydownMenu);
})

</script>
<template>
  <div>
    <div class="menu-mesk" v-if="active!==null" @click="active=null"></div>
    <div class="menu-container" :key="locale">
      <ul>
        <li @click="active=item.key" v-for="item in menus" :key="$t('menu.file.name')">
          <span>{{ item.name }}</span>
          <ul class="sub-menu" :class="{'active':active===item.key}" >
            <li v-for="subitem in item.submenu" :key="item.key" @click.stop="onClickMenu(subitem.key)">
              <span>{{ subitem.name }}</span>
              <span class="cmd">{{ subitem.cmd.join('+') }}</span>
            </li>
          </ul>
        </li>
      </ul>
    </div>
  </div>
</template>
<style lang="scss">
.menu-mesk{
  position: absolute;
  width: 100vw;
  top: 30px;
  height: calc(100vh - 30px);
  z-index: 996;
  overflow: hidden;
}
.menu-container{
  $menuShadowColor:var(--hoverBackgroundColor);
  position: absolute;
  top: 0;
  left: 0;
  font-size: .9em;
  color: var(--textColor);
 &>ul{
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: row;
  li{
    cursor: pointer;
    padding: 0 .3em;
    height: 30px;
    line-height: 30px;
    &:hover{
      background-color: var(--hoverBackground);
    }
  }
  ul.sub-menu{
    list-style: none;
    padding: 0;
    margin: 0;
    position: absolute;
    display: none;
    width: 15em;
    z-index: 997;
    box-shadow: 0 0 .6em $menuShadowColor;
    background-color: var(--backgroundColor);
    border-radius: .3em;
    &.active{
      display: block;
    }
    li{
      padding: .2em .6em;
      display: flex;
      justify-content: space-between;
      cursor: pointer;
      span{
        overflow:hidden;
        text-overflow:ellipsis;
      }
      .cmd{
        font-size: .3em;
        color: rgb(133, 133, 133);
      }
    }
  }
  &::after{
    content: '';
    display: block;
    clear: both;
  }
 }
}
</style>