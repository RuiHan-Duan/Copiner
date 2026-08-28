<#
.SYNOPSIS
    Copiner Windows 构建入口，与根 Makefile 提供等价目标。

.EXAMPLE
    ./build.ps1 check
    ./build.ps1 dev

.NOTES
    需要 PowerShell 7、Node 24、pnpm 11.12.0 和 Rust stable。
    缺失工具时明确报错，不静默跳过验证。
#>
param(
    [Parameter(Position = 0)]
    [ValidateSet('help', 'install', 'build', 'test', 'lint', 'check', 'clean', 'dev', 'workspace-run')]
    [string]$Target = 'help'
)

$ErrorActionPreference = 'Stop'
$Root = $PSScriptRoot

function Require-Tool([string]$Name) {
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "缺少必需工具：$Name"
    }
}

function Invoke-In([string]$Directory, [scriptblock]$Block) {
    Push-Location (Join-Path $Root $Directory)
    try {
        & $Block
        if ($LASTEXITCODE -ne 0) { throw "命令失败，退出码 $LASTEXITCODE" }
    }
    finally { Pop-Location }
}

function Desktop-Install { Require-Tool 'pnpm'; Invoke-In 'Desktop' { pnpm install --frozen-lockfile } }
function Desktop-Build { Require-Tool 'pnpm'; Invoke-In 'Desktop' { pnpm run build } }
function Desktop-Test {
    Require-Tool 'pnpm'
    Invoke-In 'Desktop' { pnpm run typecheck; if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }; pnpm run check:consistency }
    Write-Host 'Desktop: 当前无交互单测，以 typecheck、build、lint 和一致性检查验收'
}
function Desktop-Lint { Require-Tool 'pnpm'; Invoke-In 'Desktop' { pnpm run lint } }
function Rust-Build { Require-Tool 'cargo'; Invoke-In '.' { cargo build --workspace } }
function Rust-Test { Require-Tool 'cargo'; Invoke-In '.' { cargo test --workspace } }
function Rust-Lint {
    Require-Tool 'cargo'
    Invoke-In '.' {
        cargo fmt --all -- --check
        if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
        cargo clippy --workspace --all-targets -- -D warnings
    }
}

switch ($Target) {
    'help' {
        Write-Host @'
Copiner 构建入口（PowerShell 7）

  ./build.ps1 install        安装 Desktop 冻结依赖
  ./build.ps1 build          构建 Desktop 与 Rust workspace
  ./build.ps1 test           执行 Desktop 骨架检查与 Rust 测试
  ./build.ps1 lint           执行前端与 Rust 静态检查
  ./build.ps1 check          build + test + lint
  ./build.ps1 clean          清理生成物
  ./build.ps1 dev            启动 Tauri 桌面调试
  ./build.ps1 workspace-run  运行未配置的 WorkspaceService
'@
    }
    'install' { Desktop-Install }
    'build' { Desktop-Build; Rust-Build }
    'test' { Desktop-Test; Rust-Test }
    'lint' { Desktop-Lint; Desktop-Test; Rust-Lint }
    'check' { Desktop-Build; Rust-Build; Desktop-Test; Rust-Test; Desktop-Lint; Rust-Lint }
    'clean' {
        Remove-Item -Recurse -Force (Join-Path $Root 'Desktop/dist') -ErrorAction SilentlyContinue
        Remove-Item -Recurse -Force (Join-Path $Root 'Desktop/node_modules/.vite') -ErrorAction SilentlyContinue
        Require-Tool 'cargo'
        Invoke-In '.' { cargo clean }
    }
    'dev' {
        $Tauri = Join-Path $Root 'Desktop/node_modules/.bin/tauri.cmd'
        if (-not (Test-Path $Tauri)) { throw '缺少 Tauri CLI，请先执行 ./build.ps1 install' }
        Invoke-In 'Crates/DesktopHost' { & $Tauri dev }
    }
    'workspace-run' {
        Require-Tool 'cargo'
        Invoke-In '.' { cargo run -p copiner-workspace-service }
    }
}
