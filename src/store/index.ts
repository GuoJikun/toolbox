import { defineStore } from 'pinia'
import type { PluginConfig } from '@/utils/typescript'

interface State {
    plugins: PluginConfig[]
}

export const useIndexStore = defineStore('index', {
    state: (): State => ({
        plugins: []
    }),

    actions: {
        addPlugin(val: PluginConfig) {
            this.plugins.push(val)
        },
        updatePlugins(val: PluginConfig[]) {
            this.plugins = val
        }
    },
    persist: {
        key: 'vtools-index',
        paths: ['plugins']
    }
})
