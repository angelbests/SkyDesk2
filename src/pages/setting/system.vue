<script setup lang="ts">
// import { refreshAll,StaticInfo,Process,MemoryInfo,Disk,Network,Cpu } from "tauri-plugin-system-info-api";
// import { onMounted, ref } from "vue";
// onMounted(async ()=>{
//     setInterval(async () => {
//         // await refreshAll()
//         await networksinfo()
//         // await cpuinfo()
//         // await memoryinfo()
//         // await diskinfo()
//         // await processinfo()

//     }, 1000);
//     // await pcinfo()
// })

// const cpu = ref<Cpu[]>()
// const network = ref<Network>()
// const disk = ref<Disk[]>()
// const memory = ref<MemoryInfo>()
// const process = ref<Process[]>()
// const sysinfo = ref<StaticInfo>()
// // 网络
// import { networks } from "tauri-plugin-system-info-api";
// const networksinfo = async function(){
//     let res = await networks()
//     res.filter(item=>{
//         console.log(item.interface_name,item.received,item.transmitted)
//     })
//     console.log(res)
// }

// import { cpuInfo,refreshCpu } from "tauri-plugin-system-info-api";
// const cpuinfo =async function(){
//     console.log(await cpuInfo())
//     console.log(await refreshCpu())
// }

// import { disks } from "tauri-plugin-system-info-api";
// const diskinfo =async function(){
//     console.log(await disks())
// }

// import { memoryInfo,refreshMemory } from "tauri-plugin-system-info-api";
// const memoryinfo =async function(){
//     console.log(await memoryInfo())
//     console.log(await refreshMemory())
// }

// import { processes,refreshProcesses } from "tauri-plugin-system-info-api";
// const processinfo =async function(){
//     console.log(await processes())
//     console.log(await refreshProcesses())
// }

// import { staticInfo,batteries } from "tauri-plugin-system-info-api";
// const pcinfo = async function(){
//     console.log(await staticInfo())
//     console.log(await batteries())
// }


</script>

<template>

</template>

<style>
.window {
    width: 100%;
    height: 100%;
}
</style>