import { defineStore } from 'pinia'
import type { PluginConfig, InstalledPkg } from '@/utils/typescript'

interface State {
    plugins: PluginConfig[]
    installedPkg: InstalledPkg[]
}

export const useIndexStore = defineStore('index', {
    state: (): State => ({
        plugins: [],
        installedPkg: []
    }),

    actions: {
        addPlugin(val: PluginConfig) {
            this.plugins.push(val)
        },
        updatePlugins(val: PluginConfig[]) {
            this.plugins = val
        },
        updateInstalledPkg(val: InstalledPkg[]) {
            this.installedPkg = val
        }
    },
    persist: {
        key: 'vtools-index',
        paths: ['plugins', 'installedPkg']
    }
})
