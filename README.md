# HSNPhira v2 Web API Plugin for Phira-mp+

HSNPhira v2 前端所需的 Web API 端点，以 WASM 插件形式运行在 Phira-mp+ 上。

## 功能

提供 HSNPhira v2 前端所需的 REST API 端点：

- **房间 API** — 房间列表、历史、详情
- **谱面排行 API** — 谱面排行、热门排行
- **用户排行 API** — 用户排行、在线时长排行

## 环境准备

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

### 3. 编译

```bash
cargo build --target wasm32-unknown-unknown --release
```

产物：`target/wasm32-unknown-unknown/release/hsnphira_v2_pmp_plugin.wasm`

### 4. 查看编译产物

```bash
ls -lh target/wasm32-unknown-unknown/release/hsnphira_v2_pmp_plugin.wasm
file target/wasm32-unknown-unknown/release/hsnphira_v2_pmp_plugin.wasm
sha256sum target/wasm32-unknown-unknown/release/hsnphira_v2_pmp_plugin.wasm
```

### 5. 清理构建

```bash
cargo clean
# 或仅清理产物
rm -f target/wasm32-unknown-unknown/release/hsnphira_v2_pmp_plugin.wasm
```

## 使用 GitHub Actions（无需本地环境）

每次推送到 `main` 后自动构建：

1. 打开 https://github.com/FireflyF09/HSNPhira-v2-PMP-plugin/actions
2. 选择最新的 workflow run
3. 在 Artifacts 下载 `.wasm`

手动触发：Actions → Build WASM Plugin → Run workflow

## 部署

### 1. 创建插件目录

```bash
mkdir -p /path/to/phira-mp-plus/data/plugins/hsnphira-v2
```

### 2. 复制 WASM

```bash
# 本地构建
cp target/wasm32-unknown-unknown/release/hsnphira_v2_pmp_plugin.wasm \
   /path/to/phira-mp-plus/data/plugins/hsnphira-v2/plugin.wasm

# 或从 Actions 下载
cp ~/Downloads/hsnphira-v2-pmp-plugin.wasm \
   /path/to/phira-mp-plus/data/plugins/hsnphira-v2/plugin.wasm
```

### 3. 创建 manifest

`/path/to/phira-mp-plus/data/plugins/hsnphira-v2/plugin.json`：

```json
{
  "capabilities": ["state.read", "admin", "room.manage"]
}
```

### 4. 验证

```bash
# CLI 验证
plugin list             # 检查 hsnphira-v2-pmp-plugin 已加载
plugin info hsnphira-v2-pmp-plugin  # 查看详情

# HTTP 验证
curl http://localhost:12347/newapi/rooms/info
curl http://localhost:12347/rankapi/playtime_leaderboard
curl http://localhost:12347/topchart/hot_rank/all

# 日志
journalctl -u phira-mp-plus -f | grep plugin
```

## API 端点

| 路由 | 方法 | 用途 |
|------|------|------|
| `/newapi/rooms/info` | GET | 房间列表 |
| `/newapi/rooms/history?room_id=X` | GET | 房间游玩历史 |
| `/newapi/rooms/listen` | SSE/WS | 房间事件 |
| `/api/rooms/info/:name` | GET | 单房间详情 |
| `/chart/:id/rank` | GET | 谱面排行信息 |
| `/topchart/chart_rank/:chart_id` | GET | 谱面排名详情 |
| `/topchart/hot_rank/:timeRange` | GET | 热门谱面排行 |
| `/user_rank/:timeRange` | GET | 用户排行 |
| `/rankapi/playtime_leaderboard` | GET | 在线时长排行 |

## 常见问题

### WIT 文件找不到

```
failed to read path for WIT ../../wit/phira-plugin.wit
```

解决：确认 `wit` 软链接存在：

```bash
ls -la wit   # 应指向 Phira-mp-plus/wit
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
