import { createApp } from 'vue'
import App from './App.vue'
import Router from './router/index'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'

import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import zhCn from 'element-plus/es/locale/lang/zh-cn'

import '@/assets/css/global.scss'
import '@icon-park/vue-next/styles/index.css'

import { registerShortcut, getWindow } from '@/utils/index'

async function addShortcut() {
    const shortcut = 'Alt+Space'
    registerShortcut(shortcut, async () => {
        console.log('Shortcut triggered')
        const label = await getWindow('search')
        console.log('label', label)
        if (label) {
            if (await label.isVisible()) {
                label.show()
            } else {
                label.setFocus()
            }
        }
        label?.show()
    })
}

addShortcut()

const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)

const app = createApp(App)
app.use(Router)
app.use(pinia)
app.use(ElementPlus, { locale: zhCn })
app.mount('#app')
