import { fetch } from "@tauri-apps/plugin-http"
// 9cda7ed49a914d5eb6987706d642da65
import { weatherStore } from "../stores/window"
export const getpoi =async function(city:string):Promise<any>{
    const w = weatherStore()
    if(w.apikey && city) {
        let respone = await fetch(`https://geoapi.qweather.com/v2/city/lookup?location=${city}&key=${w.apikey}&gzip=n`,{
            method:"GET",
        })
        if(respone.ok){
            let json =await respone.json()
            console.log(json)
            return json.location
        }
    }
    return []
}
// 9cda7ed49a914d5eb6987706d642da65
export const getWeather =async function(location:string):Promise<any>{
    const w = weatherStore()
    let header = new Headers()
    header.append("content-encoding","gzip")
    let respone = await fetch(`https://devapi.qweather.com/v7/weather/now?location=${location}&key=${w.apikey}`,{
        method:"GET",
        headers:header
    })
    console.log(respone) 
    try {
        if(respone.ok){
            // 解压gzip数据
            // for (const pair of respone.headers.entries()) {
            //     console.log(`${pair[0]}: ${pair[1]}`);
            // }
            let json = await respone.json()
            console.log(json) 
            // 解压Gzip
            // let blob = await respone.blob()
            // let ds = new DecompressionStream("gzip")
            // const de = blob.stream().pipeThrough(ds);
            // let res = await new Response(de).json()
            return json
        }
    } catch (error) {
        console.log(error)
    }
}