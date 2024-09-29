import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import HomeLayout from '../views/home/layout.vue'

import Home from '../views/home/index.vue'
import Search from '../views/search/index.vue'

import plugins from "./plugins"

const routes: RouteRecordRaw[] = [
    {
        children: [
            {
                path: '/home',
                name: 'home',
                component: Home
            },
            {
                path: '/setting',
                name: 'setting',
                component: () => import('@/views/home/setting/index.vue')
            },
            plugins
        ],
        component: HomeLayout,
        name: 'homeLayout',
        path: '/',
        redirect: '/home'
    },
    {
        path: '/search',
        name: 'search',
        component: Search
    }
]

export default createRouter({
    history: createWebHistory(),
    routes
})
