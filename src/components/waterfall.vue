<script setup lang="ts">
import { onMounted, ref, toRefs, watch } from 'vue';
const list = ref<any[]>([]) // 每列的数组
const props = defineProps<{
    col:number, // 几列
    gap:number, // 间距
    images:{ // 图片合集
        width:number // 源图片宽
        height:number, // 原图片高
        src:string, // 源图片
        thumbs:string, // 缩略图
        ratio:number, // 纵横比
    }[], 
}>()
const { images } = toRefs(props)
let height = ref<number[]>([]) // 每列高度的存储 用于计算最低列
let length = 0; // 上次数组的长度
const emit = defineEmits<{
    scrollload:[], // 滚动触底
}>()
const timer = ref(true)
onMounted(()=>{
    // 触底
    document.getElementById('waterfall')?.addEventListener("scroll",(e:any)=>{
        let height = e.target.scrollHeight - e.target.clientHeight
        if(e.target.scrollTop>(height-100)){
            if(timer.value){
                timer.value = false
                setTimeout(() => {
                    emit('scrollload')
                    timer.value = true
                }, 300);
            }
        }
    })

    // 初始化高度 和  列数组
    for(let i = 0;i<props.col;i++){
        height.value.push(0)
        list.value.push([])
    }
    waterfallset([...images.value])
})

// 瀑布流计算
const waterfallset = (arr:{
    width:number
    height:number,
    src:string ,
    thumbs:string,
    ratio:number,
}[])=>{
    for(let i = 0;i<arr.length;i++){
        let minvalue = Math.min(...height.value)    
        let index = height.value.indexOf(minvalue)
        let dom = document.getElementById(`c${index}`)
        if(dom){
            let rate = arr[i].width/dom.clientWidth
            let h = Math.trunc(arr[i].height/rate);
            let w = dom.clientWidth
            list.value[index].push({
                height:h,
                src:arr[i].src,
                thumbs:arr[i].thumbs,
                width:w
            })
            height.value[index] = height.value[index] + h
        }
    }
    length = length + arr.length
}
// 监听图片合集变化
watch(images.value,()=>{
    console.log(images.value)
    if(images.value.length == 0){
        for(let i = 0;i<props.col;i++){
            height.value[i] = 0
            list.value[i] = []
            length = 0
        }
    }
    let arr:any[] = []
    if(length > 0){
        arr = images.value.slice(length)
    }else{
        arr = [...images.value]
    }
    waterfallset(arr)
})

</script>

<template>
    <div class="container" id="waterfall">
        <div class="col" v-for="(_n,i) in props.col" :id="'c'+i" :style="{width: `calc( 100% / ${col})`,marginRight:`${gap}px`,marginLeft:i==0?`${gap}px`:`0px`}">
            <div v-for="(item) in list[i]" :src="item.thumbs" loading="lazy" :style="{width:'100%',marginBottom:gap+'px'}">
                <slot :image="item" style="width: 100%;"></slot>
            </div>
        </div>
    </div>
</template>

<style scoped>
.container {
    width: 100%;
    height: 100%;
    position: relative;
    display: flex;
    flex-direction: row;
    justify-content: space-evenly;
    overflow: hidden;
    overflow-y: scroll;
}
.col {
    margin-top: 10px;
    display: flex;
    flex-direction: column;
}

</style>