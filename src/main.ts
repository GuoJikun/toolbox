import { createApp } from "vue";
import App from "./App.vue";
import Router from "./router/index";

import "@/assets/css/global.scss";

import { registerShortcut, getWindow } from "@/utils/index";

async function addShortcut() {
    const shortcut = 'Alt+Space'
    registerShortcut(shortcut, ()=>{
        console.log('Shortcut triggered')
        const label = getWindow('search')
        console.log(label)
        label?.show()
    
    })
}

addShortcut()

createApp(App).use(Router).mount("#app");
