# HSNPhira v2 Web API Plugin for Phira-mp+

HSNPhira v2 前端所需的 Web API 端点，以 WASM 插件形式运行在 Phira-mp+ 上。

## 功能

提供 HSNPhira v2 前端所需的 REST API 端点：

- **房间 API** — 房间列表、历史、详情
- **谱面排行 API** — 谱面排行、热门排行
- **用户排行 API** — 用户排行、在线时长排行

## 依赖

- Phira-mp+ (支持 WIT/component-model 插件加载)
- Rust `wasm32-unknown-unknown` 目标：`rustup target add wasm32-unknown-unknown`

## 构建

```bash
cargo build --target wasm32-unknown-unknown --release
```

## 部署

```bash
cp target/wasm32-unknown-unknown/release/hsnphira_v2_pmp_plugin.wasm \
   /path/to/phira-mp-plus/data/plugins/hsnphira-v2/plugin.wasm
```

创建 `data/plugins/hsnphira-v2/plugin.json`:

```json
{
  "capabilities": ["state.read", "admin", "room.manage"]
}
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

## 架构

插件启动时通过 `api-call("http.register_route", ...)` 向 Phira-mp+ 服务器注册 HTTP 路由。每个请求由服务端的 `server_state_query` 处理。

参见 [docs/hsnphira-v2-migration.md](docs/hsnphira-v2-migration.md)。
