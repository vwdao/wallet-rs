import { cpSync, rmSync, mkdirSync, existsSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const outDir = join(root, 'out')
const dest = join(root, '..', 'static', 'admin')

if (!existsSync(outDir)) {
  console.error('Missing out/. Run next build first.')
  process.exit(1)
}

rmSync(dest, { recursive: true, force: true })
mkdirSync(dest, { recursive: true })
cpSync(outDir, dest, { recursive: true })
console.log(`Synced ${outDir} -> ${dest}`)
