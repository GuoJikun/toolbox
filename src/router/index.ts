import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import HomeLayout from '../views/home/layout.vue'

import Home from '../views/home/index.vue'
import Search from '../views/search/index.vue'

const routes: RouteRecordRaw[] = [
    {
        path: '/',
        name: 'homeLayout',
        component: HomeLayout,
        redirect: '/home',
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
            {
                path: '/plugins',
                name: 'plugins',
                component: () => import('@/views/home/plugins/index.vue')
            }
        ]
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
