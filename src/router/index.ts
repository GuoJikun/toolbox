

import { createRouter, createWebHashHistory, type RouteRecordRaw } from "vue-router";
import HomeLayout from "../views/home/layout.vue";

import Home from "../views/home/index.vue";
import Search from "../views/search/index.vue";

const routes: RouteRecordRaw[] = [
    {
        path: "/",
        name: "root",
        components: HomeLayout,
        redirect: "/home",
        children: [
            {
                path: "/home",
                name: "home",
                components: Home
            }
        ],
    },
    {
        path: "/search",
        name: "search",
        components: Search
        
    }
]

export default createRouter({
    history: createWebHashHistory(),
    routes
})