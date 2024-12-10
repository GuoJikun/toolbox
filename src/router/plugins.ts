import layout from '@/views/plugins/layout.vue'
export const plugins = {
    name: 'plugins',
    path: '/plugins',
    component: layout,
    children: [
        {
            path: '/plugins/color-conversion',
            component: () => import('@/views/plugins/color-conversion.vue')
        },
        {
            path: '/plugins/local-server',
            component: () => import('@/views/plugins/local-server.vue')
        }
    ]
}

export default plugins
