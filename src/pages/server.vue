<script setup lang="ts" >
import { openserver } from '../functions/server';
import { onMounted } from 'vue';
onMounted(async ()=>{
    await openserver()
})
</script>

<template>
    <div style="width: 100%;height: 100%;background: white;">
        server
    </div>
</template>