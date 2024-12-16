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
        },
        {
            name: 'pkgManager',
            path: '/plugins/pkg-manager',
            component: () => import('@/views/plugins/pkg-manager.vue')
        },
        {
            path: '/plugins/convert-image-to-webp',
            component: () => import('@/views/plugins/convert-image-to-webp.vue')
        }
    ]
}

export default plugins
