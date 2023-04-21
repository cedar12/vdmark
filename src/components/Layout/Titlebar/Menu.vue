<script setup lang="ts">
import {ref,defineEmits} from 'vue';
import {useI18n} from 'vue-i18n';

const emit=defineEmits(['click']);
const {t}=useI18n();

const menus=[
  {
    key:'file',
    name:t('menu.file'),
    submenu:[
      {
        key:'new',
        name:t('menu.new'),
        cmd:['ctrl','n'],
      },
      {
        key:'open',
        name:t('menu.open'),
        cmd:['ctrl','o'],
      },
      {
        key:'save',
        name:t('menu.save'),
        cmd:['ctrl','s'],
      },
      {
        key:'resave',
        name:t('menu.saveas'),
        cmd:['ctrl','alt','s'],
      },
      {
        key:'config',
        name:t('menu.config'),
        cmd:[],
      }
    ]
  },
  {
    key:'view',
    name:t('menu.view.name'),
    submenu:[
      {
        key:'viewEdit',
        name:t('menu.view.edit'),
        cmd:[],
      },
      {
        key:'viewSource',
        name:t('menu.view.source'),
        cmd:[],
      },
      {
        key:'viewMix',
        name:t('menu.view.mix'),
        cmd:[],
      }
    ]
  },
  {
    key:'help',
    name:t('menu.help'),
    submenu:[
      {
        key:'about',
        name:t('menu.about'),
        cmd:[],
      }
    ]
  }
]

const active=ref<string|null>(null);

const onClickMenu=(key:string)=>{
  active.value=null;
  emit('click',key);
}

</script>
<template>
  <div class="menu-mesk" v-if="active!==null" @click="active=null">

  </div>
  <div class="menu-container">
    <ul>
      <li @click="active=item.key" v-for="item in menus" :key="item.key">
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
</template>
<style lang="scss">
.menu-mesk{
  position: absolute;
  width: 100vw;
  height: calc(100vh - 30px);
  z-index: 996;
  overflow: hidden;
}
.menu-container{
  position: absolute;
  top: 0;
  left: 0;
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
    width: 12em;
    z-index: 997;
    box-shadow: 0 0 5px #ccc;
    background-color: #fff;
    &.active{
      display: block;
    }
    li{
      padding: .2em .6em;
      display: flex;
      justify-content: space-between;
      cursor: pointer;
      .cmd{
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