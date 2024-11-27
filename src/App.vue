<script setup lang="ts">
document.addEventListener("contextmenu",(e)=>{
  e.preventDefault()
})
document.onkeydown = function(e){
  console.log(e)
  if(e.key == "F7"){
    return false
  }
  // 屏蔽刷新
  if(e.key == "F5"){
    return false
  }
  // 屏蔽刷新
  if(e.ctrlKey && e.key == "r"){
    return false
  }
  // 禁止打印
  if(e.ctrlKey && e.key == "p"){
    return false
  }
  // 禁止查找
  if(e.key == "F3"){
    return false
  }
  // 禁止强制刷新
  if(e.ctrlKey && e.shiftKey && e.key == "R"){
    
    return false
  }
  // 查看源代码
  if(e.ctrlKey && e.key == "u"){
    return false
  }
  // 查找网页
  if(e.ctrlKey && e.key == "f"){
    return false
  }
  // 查找下一个
  if(e.ctrlKey && e.key == "g"){
    return false
  }
  // 查找上一个
  if(e.ctrlKey && e.shiftKey && e.key == "G"){
    return false
  }
  // 下载
  if(e.ctrlKey && e.key == "j"){
    return false
  }
}
</script>

<template>
  <router-view></router-view>
</template>

<style>
html,body {
  overflow: hidden !important;
  width: 100vw;
  height: 100vh;
  background: transparent;
  padding: 0;
  margin: 0;
  border: none !important;
}

::-webkit-scrollbar {
    width: 2px;
}

::-webkit-scrollbar-thumb {
  background: rgba(123,123,123,0.5);
  border-radius: 5px;
}
::-webkit-scrollbar-button{
  display: none;
}

::-webkit-scrollbar-track {
  display: none;
}

::-webkit-scrollbar-corner {
  display: none;
}
</style>
