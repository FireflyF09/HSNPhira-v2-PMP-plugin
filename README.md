# HSNPhira v2 Web API Plugin for Phira-mp+

HSNPhira v2 前端所需的 Web API 端点，以 WASM 插件形式运行在 Phira-mp+ 上。

## 功能

提供 HSNPhira v2 前端所需的 REST API 端点：

- **访客统计** — 访问过的用户数量
- **房间 API** — 房间列表、详情
- **游玩时间排行** — 在线时长排行
- **房间事件流** — SSE 实时房间事件

## 快速部署（下载发行版 .wasm）

无需本地编译，直接从 GitHub Releases 下载预构建的 WASM 插件文件：

```bash
# 1. 创建插件目录
mkdir -p /path/to/phira-mp-plus/plugins/hsnphira-v2

# 2. 下载最新 Release 的 .wasm
wget -O /path/to/phira-mp-plus/plugins/hsnphira-v2/plugin.wasm \
  https://github.com/FireflyF09/HSNPhira-v2-PMP-plugin/releases/latest/download/hsnphira_v2_pmp_plugin.wasm

# 3. 重启服务器
systemctl restart phira-mp-plus

# 4. 验证
plugin list                    # 检查 hsnphira-v2-pmp-plugin 已加载
curl http://localhost:12347/api/rooms/info
```

> 发行版 .wasm 文件来自 GitHub Releases → 选择最新版本 → Assets 下载。

## 从源码构建

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### 安装 WASM 目标

```bash
rustup target add wasm32-unknown-unknown
```

### 验证环境

```bash
rustc --version                    # 需 1.75+
rustup target list --installed     # 需包含 wasm32-unknown-unknown
cargo --version
```

## 构建

### 1. 克隆插件仓库

```bash
git clone https://github.com/FireflyF09/HSNPhira-v2-PMP-plugin
cd HSNPhira-v2-PMP-plugin
```

### 2. 下载 SDK 依赖

插件编译需要 `phira-plugin-sdk` 和 WIT 定义文件（来自 [Phira-mp-plus](https://github.com/HyperSynapseNetwork/Phira-mp-plus) 仓库）：

```bash
# 方式一：从 PMP Release 下载（推荐）
wget https://github.com/HyperSynapseNetwork/Phira-mp-plus/releases/latest/download/phira-plugin-sdk.tar.gz
tar xzf phira-plugin-sdk.tar.gz
# 解压后得到 phira-plugin-sdk/ 和 wit/

# 方式二：从主仓库直接克隆
git clone --depth 1 --filter=blob:none \
    https://github.com/HyperSynapseNetwork/Phira-mp-plus \
    /tmp/phira-mp-plus
ln -s /tmp/phira-mp-plus/phira-plugin-sdk ./phira-plugin-sdk
ln -s /tmp/phira-mp-plus/wit ./wit
```

### 3. 安装 wasm-tools

```bash
cargo install wasm-tools
```

### 4. 编译

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
wasm-tools component new \
  target/wasm32-unknown-unknown/release/hsnphira_v2_pmp_plugin.wasm \
  -o target/wasm32-unknown-unknown/release/hsnphira_v2_pmp_plugin.component.wasm
```

产物：`target/wasm32-unknown-unknown/release/hsnphira_v2_pmp_plugin.component.wasm`

### 5. 查看编译产物

```bash
ls -lh target/wasm32-unknown-unknown/release/hsnphira_v2_pmp_plugin.component.wasm
```

### 6. 清理构建

```bash
cargo clean
```

## 部署

### 1. 从 Release 下载（推荐）

```bash
wget -O /path/to/phira-mp-plus/plugins/hsnphira-v2/plugin.wasm \
  https://github.com/FireflyF09/HSNPhira-v2-PMP-plugin/releases/latest/download/hsnphira_v2_pmp_plugin.wasm
```

### 2. 或从本地构建复制

```bash
cp target/wasm32-unknown-unknown/release/hsnphira_v2_pmp_plugin.component.wasm \
   /path/to/phira-mp-plus/plugins/hsnphira-v2/plugin.wasm
```

### 3. 验证

```bash
plugin list                                   # 检查已加载
plugin info hsnphira-v2-pmp-plugin            # 查看详情
curl http://localhost:12347/api/rooms/info    # HTTP 验证
```

## API 端点

| 路由 | 方法 | 用途 |
|------|------|------|
| `/api/auth/visited/count` | GET | 访问过的用户数量 |
| `/api/rooms/info` | GET | 房间列表 |
| `/api/rooms/info/:name` | GET | 单房间详情 |
| `/newapi/rooms/listen` | SSE | 房间事件流 |
| `/rankapi/playtime_leaderboard` | GET | 游玩时间排行榜 |

## 常见问题

### WIT 文件找不到

```
failed to read path for WIT ../wit/phira-plugin.wit
```

解决：确认 `wit/` 目录存在于项目根目录：

```bash
ls wit/phira-plugin.wit
# 或从 PMP Release 下载 SDK tarball（内含 wit/）
```

### wit-bindgen 未安装

```toml
[dependencies]
wit-bindgen = "0.58"
```

### 编译缺少 C 编译器

```bash
# Ubuntu/Debian
apt install build-essential cmake
# macOS
xcode-select --install
```

### 编译慢

```bash
cargo install sccache
export RUSTC_WRAPPER=sccache
cargo build --target wasm32-unknown-unknown
```
