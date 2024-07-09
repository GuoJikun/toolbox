import { createApp } from 'vue'
import App from './App.vue'
import Router from './router/index'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'

import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import zhCn from 'element-plus/es/locale/lang/zh-cn'

import '@/assets/css/global.scss'

import { registerShortcut, getWindow, initPluginDir } from '@/utils/index'

async function addShortcut() {
    const shortcut = 'Alt+Space'
    registerShortcut(shortcut, () => {
        console.log('Shortcut triggered')
        const label = getWindow('search')
        console.log(label)
        label?.show()
    })
}

addShortcut()
initPluginDir()

const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)

const app = createApp(App)
app.use(Router)
app.use(pinia)
app.use(ElementPlus, { locale: zhCn })
app.mount('#app')
