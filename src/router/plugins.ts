import layout from '@/views/plugins/layout.vue'
export const plugins = {
    name: 'plugins',
    path: '/plugins',
    component: layout,
    children: [
        {
            path: '/plugins/color-conversion',
            component: () => import('@/views/plugins/color-conversion.vue')
        }
    ]
}

export default plugins