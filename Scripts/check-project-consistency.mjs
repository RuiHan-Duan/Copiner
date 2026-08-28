import fs from 'node:fs'
import path from 'node:path'
import process from 'node:process'
import { fileURLToPath } from 'node:url'

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..')
const expectedPackageManager = 'pnpm@11.12.0'
const errors = []

const exists = (relativePath) => fs.existsSync(path.join(rootDir, relativePath))
const read = (relativePath) => fs.readFileSync(path.join(rootDir, relativePath), 'utf8')

if (!exists('Desktop/pnpm-lock.yaml')) errors.push('缺少 Desktop/pnpm-lock.yaml')
for (const forbidden of ['Desktop/package-lock.json', 'Desktop/yarn.lock']) {
  if (exists(forbidden)) errors.push(`禁止存在 ${forbidden}，Desktop 只使用 pnpm 锁文件`)
}

const packageJson = JSON.parse(read('Desktop/package.json'))
if (packageJson.packageManager !== expectedPackageManager) {
  errors.push(`packageManager 必须精确为 ${expectedPackageManager}`)
}
if (packageJson.engines?.node !== '>=24 <25' || packageJson.engines?.pnpm !== '11.12.0') {
  errors.push('Desktop engines 必须固定 Node >=24 <25 和 pnpm 11.12.0')
}

const pnpmSettings = read('Desktop/pnpm-workspace.yaml')
if (!/^allowBuilds:\s*\r?\n\s+esbuild: true\s*$/m.test(pnpmSettings)) {
  errors.push('pnpm allowBuilds 必须只允许 esbuild 执行安装脚本')
}
if (/^packages:/m.test(pnpmSettings)) errors.push('Desktop 是单 package，不应配置 workspace packages')

const requiredFiles = [
  'Desktop/AGENTS.md',
  'Desktop/README.md',
  'Desktop/Conversation/README.md',
  'Desktop/TaskFlow/README.md',
  'Desktop/Harness/README.md',
  'Crates/AGENTS.md',
  'Crates/CoreEngine/README.md',
  'Crates/DesktopHost/README.md',
  'WorkspaceService/AGENTS.md',
  'Hooks/session-start.schema.json',
  'Docs/04-AI辅助开发指南.md',
]
for (const requiredFile of requiredFiles) {
  if (!exists(requiredFile)) errors.push(`缺少工程框架文件：${requiredFile}`)
}

const makefile = read('Makefile')
if (!makefile.includes('pnpm install --frozen-lockfile')) errors.push('Makefile 未使用 frozen pnpm 安装')
if (!makefile.includes('cargo clippy --workspace --all-targets -- -D warnings')) {
  errors.push('Makefile 缺少 Rust workspace clippy 门禁')
}

const buildScript = read('build.ps1')
for (const expected of ['pnpm install --frozen-lockfile', "Require-Tool 'cargo'", 'cargo test --workspace']) {
  if (!buildScript.includes(expected)) errors.push(`build.ps1 缺少：${expected}`)
}

const workflow = read('.github/workflows/ci.yml')
for (const expected of [
  'node-version: 24.14.0',
  'corepack prepare pnpm@11.12.0 --activate',
  'pnpm install --frozen-lockfile',
  'cargo clippy --workspace --all-targets -- -D warnings',
  'pnpm run check:consistency',
]) {
  if (!workflow.includes(expected)) errors.push(`CI 缺少：${expected}`)
}

for (const documentation of ['README.md', 'Desktop/README.md', 'Docs/03-技术选型.md']) {
  if (!read(documentation).includes('pnpm')) errors.push(`${documentation} 未说明 pnpm`)
}

const coworkLineCount = read('COWORK.md').trim().split(/\r?\n/).length
if (coworkLineCount > 30) errors.push(`COWORK.md 当前 ${coworkLineCount} 行，超过渐进加载约定的 30 行`)

const hookExample = JSON.parse(read('Hooks/session-start.example.json'))
if (hookExample.sources.some((source) => source.enabled !== false)) {
  errors.push('SessionStart 示例中的来源必须全部默认禁用')
}

if (errors.length > 0) {
  console.error('Copiner 工程一致性检查失败：')
  for (const error of errors) console.error(`- ${error}`)
  process.exit(1)
}

console.log(`Copiner 工程一致性检查通过：${expectedPackageManager}、模块文档、构建、CI 与安全示例一致。`)
