<script setup lang="ts">
document.addEventListener("contextmenu",(e)=>{
  // e.preventDefault()
})
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
