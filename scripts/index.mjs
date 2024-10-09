import { URL, fileURLToPath } from 'node:url'
import { join } from 'node:path'
import fs from 'fs-extra'

const getRootDir = () => {
    return fileURLToPath(new URL('..', import.meta.url))
}

const buildPlugins = async () => {
    const root = getRootDir()
    const pluginRoot = join(root, 'plugins')
    const destRoot = join(root, 'src-tauri', 'resources', 'plugins')
    await fs.emptyDir(destRoot)
    const pluginDirs = await fs.readdir(join(pluginRoot, 'workspaces'))

    for (const plugin of pluginDirs) {
        const src = join(pluginRoot, 'workspaces', plugin, 'dist')
        const dest = join(destRoot, plugin)
        await fs.copy(src, dest)
    }
}

const copyBinaries = async () => {
    const root = getRootDir()
    const src = join(root, 'binaries')
    const dest = join(root, 'src-tauri', 'binaries')
    if (!fs.exists(dest)) {
        await fs.mkdir(dest)
    }
    return await fs.copy(src, dest, {
        filter(src) {
            return !src.endsWith('.md')
        }
    })
}

const run = async () => {
    await buildPlugins()
    await copyBinaries()
}

run()
