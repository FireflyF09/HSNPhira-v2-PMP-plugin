# HSNPhira v2 前端 Web API 迁移文档

## 概述

本 WASM 插件替代了 Phira-mp+ 内置的 Web API 路由，提供与 HSNPhira v2 前端兼容的接口。
它以 WIT/component-model 组件形式由 Phira-mp+ 在启动时加载。

## 路由变更

### 已移除（内置路由，移至插件）

| 旧路由 | 新路由（插件） | 方法 | 用途 |
|--------|--------------|------|------|
| `/api/rooms` | `/newapi/rooms/info` | GET | 房间列表 |
| `/api/rooms/<name>` | `/newapi/rooms/list` | GET | 单个房间详情 |
| — | `/newapi/rooms/history?room_id=X` | GET | 房间历史 |
| — | `/newapi/rooms/listen` | SSE/WS | 房间事件 |
| `/api/runtime` | — | — | 已移除（诊断） |
| `/api/simulation` | — | — | 已移除（模拟器） |
| `/api/simulation/world` | — | — | 已移除（模拟器） |
| `/api/benchmark/reports` | — | — | 已移除（性能测试） |
| `/api/benchmark/reports/history` | — | — | 已移除（性能测试） |
| `/api/players/all` | — | — | 已移除（玩家） |
| `/api/user_name/<id>` | — | — | 已移除（用户） |

### 保留（web-monitor 兼容）

| 路由 | 方法 | 用途 |
|------|------|------|
| `/api/events` | SSE | Runtime v2 事件流 |
| `/api/ws` | WebSocket | 实时事件流 |

### 新增（HSNPhira v2）

| 路由 | 方法 | 响应类型 | 用途 |
|------|------|---------|------|
| `/newapi/rooms/info` | GET | `Room[]` | 房间列表含在线人数 |
| `/newapi/rooms/history?room_id=X` | GET | `GameHistory[]` | 房间游玩历史 |
| `/api/rooms/info/:name` | GET | Room 详情 | 单个房间信息 |
| `/chart/:id/rank` | GET | ChartRank | 谱面排行 |
| `/topchart/chart_rank/:chart_id` | GET | ChartRankDetail | 谱面排名详情 |
| `/topchart/hot_rank/:timeRange` | GET | ChartRank[] | 热门谱面排行 |
| `/user_rank/:timeRange` | GET | UserRank[] | 用户排行 |
| `/rankapi/playtime_leaderboard` | GET | 排行榜 | 在线时长排行 |

## 响应类型

### Room（匹配 HSNPhira v2 前端）

```typescript
interface Room {
  id: string
  name: string
  owner: string
  owner_id: number
  player_count: number
  max_players: number
  status: string
  is_cycling: boolean
  chart_id?: number
  chart_name?: string
  players: Player[]
}
```

### GameHistory

```typescript
interface GameHistory {
  chart_id: number
  chart_name: string
  play_time: string
  players: PlayerScore[]
}

interface PlayerScore {
  user_id: number
  username: string
  phira_id: number
  score: number
  accuracy: number
  perfect: number
  good: number
  bad: number
  miss: number
  max_combo: number
}
```

### ChartRank

```typescript
interface ChartRank {
  rank: number
  chart_id: number
  chart_name: string
  play_count: number
  increase: number
}
```

### UserRank

```typescript
interface UserRank {
  rank: number
  user_id: number
  username: string
  phira_id: number
  play_time: number
}
```

## 构建与部署

### 前置条件
- Rust 工具链，添加 `wasm32-unknown-unknown` 目标：`rustup target add wasm32-unknown-unknown`

### 构建
```bash
cd .hsnphira-pmp-plugin
cargo build --target wasm32-unknown-unknown --release
```

### 部署
将编译后的 `.wasm` 文件复制到 Phira-mp+ 的插件目录：
```bash
cp target/wasm32-unknown-unknown/release/hsnphira_v2_pmp_plugin.wasm \
   ../data/plugins/hsnphira-v2/plugin.wasm
```

创建 manifest 文件：
```bash
cat > ../data/plugins/hsnphira-v2/plugin.json << 'EOF'
{
  "capabilities": ["state.read", "admin", "room.manage"]
}
EOF
```

### 验证
1. 启动 Phira-mp+ 服务器
2. 在 CLI 中检查插件已加载：`plugin list`
3. 测试端点：`curl http://localhost:12347/newapi/rooms/info`

## 架构

```
HSNPhira v2 前端 ←→ Phira-mp+ HTTP 服务器 (:12347)
                        │
              ┌─────────┴──────────┐
              │     动态路由        │
              └─────────┬──────────┘
                        │
           ┌────────────┼────────────┐
           │            │            │
     /api/events   /newapi/*     /chart/*
     /api/ws        (插件)       (插件)
      (内置)
```

## 参考

- HSNPhira 后端拓展分支：https://github.com/HyperSynapseNetwork/HSNPhira
- Phira-mp+ WIT 插件 SDK：`phira-plugin-sdk/`
