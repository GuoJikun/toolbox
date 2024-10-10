<script setup lang="ts">
import { ref } from 'vue'
import tinyColor from 'tinycolor2'
import { useThrottleFn } from '@vueuse/core'
import { ElMessage } from 'element-plus'

const colorSource = ref<string>("")

enum ColorFormat {
    hex = "hex",
    rgb = "rgb",
    hsl = "hsl",
    hsv = "hsv",
    error = "error"
}
interface ColorItem {
    type: ColorFormat;
    value: string;
}
const colorTarget = ref<Array<ColorItem>>([])

const handleChange = () => {
    const tmp = tinyColor(colorSource.value)
    const isValid = tmp.isValid()
    if(!isValid){
        colorTarget.value = [{ type: "error", value: "输入的颜色格式不正确"}] as Array<ColorItem>
    }
    const format = tmp.getFormat()
    let result = new Map();
    const hex = tmp.toHexString()
    const rgb = tmp.toRgbString()
    const hsl = tmp.toHslString()
    const hsv = tmp.toHsvString()
    result.set("hex", hex);
    result.set("rgb", rgb);
    result.set("hsl", hsl);
    result.set("hsv", hsv);
    colorTarget.value = []
    if(result.get(format)){
        result.delete(format)
    }
    for (const [key, val] of result) {
        console.log(key,val)
        colorTarget.value.push({
            type: key,
            value: val
        })
    }
}

const handleCopy = useThrottleFn((text:string) => {
    navigator.clipboard.writeText(text).then(() => {
        ElMessage.success("复制成功")
    }).catch(()=>{
        ElMessage.error("复制失败")
    })
},100)

</script>

<template>
<div class="color-conversion">
    <div class="color-conversion__header">
        <h2>输入值：</h2>
        <el-input v-model.trim="colorSource" size="large">
            <template #append>
                <el-button @click="handleChange">转换</el-button>
            </template>
        </el-input>
    </div>
    <div class="color-conversion__body">
        <div style="display: flex;align-items: flex-end;gap: 8px">
            <h2>输出值</h2>
            <el-text  type="info">点击鼠标右键复制颜色</el-text>
        </div>
        <div class="color-conversion__body-wrap">
            <div v-for="color in colorTarget" :key="color.type" class="color-conversion__body-item" @click.right.prevent="handleCopy(color.value)">
                <span>{{color.type.toUpperCase()}}：</span>
                <div>{{color.value}}</div>
            </div>
        </div>

    </div>
</div>
</template>

<style scoped lang="scss">
.color-conversion {
    width: 100%;
    &__header {
        margin-bottom: 24px;
    }
    &__body {
        width: 100%;
        &-wrap {
            display: flex;
            flex-direction: column;
            gap: 12px;
            margin-top: 12px;
        }
        &-item {
            display: flex;
            align-items: center;
            padding: 12px;
            border-radius: 8px;
            background-color: #fff;
            box-shadow: 0 0 8px #ccc;
            cursor: copy;
        }
    }
}
</style>