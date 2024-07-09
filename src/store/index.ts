import { defineStore } from 'pinia'
import type { Plugin } from '@/utils/typescript'

interface State {
    plugins: Plugin[]
}

export const useIndexStore = defineStore('index', {
    state: (): State => ({
        plugins: []
    }),

    getters: {
        addPlugin: (state: State, val: Plugin) => state.plugins.push(val),
        updatePlugins: (state: State, val: Plugin[]) => (state.plugins = val)
    },
    actions: {},
    persist: {
        key: 'vtools-index',
        paths: ['plugins']
    }
})
