# HSNPhira v2 数据迁移教程

本文档参考 [HSNPhira/backend-extension](https://github.com/HyperSynapseNetwork/HSNPhira/tree/backend-extension) 和 [HSNPhira/backend-remake](https://github.com/HyperSynapseNetwork/HSNPhira/tree/backend-remake) 分支，说明如何将 HSNPhira 后端数据迁移到 Phira-mp+。

## 迁移来源

| 分支 | 语言 | 数据存储 | 说明 |
|------|------|---------|------|
| `backend-extension` | Python (Flask) | SQLite (`phira_stats.db`) | 排行/谱面/用户统计等扩展 API |
| `backend-remake` | Python | SQLite/PostgreSQL | 完整后端（含 phira-mp 兼容层） |

---

## 1. 用户排行数据迁移

### 来源: `backend-extension/rank.py`

原后端从 SQLite 读取 `user_playtime` 表生成游玩时间排行。

**迁移步骤:**

1. 导出用户游玩时间数据：
   ```bash
   sqlite3 phira_stats.db "SELECT user_id, SUM(play_duration) as total_playtime FROM user_playtime GROUP BY user_id ORDER BY total_playtime DESC;" > playtime_export.csv
   ```

2. 导入到 Phira-mp+：
   ```bash
   psql -d phira_mp -c "
   CREATE TABLE IF NOT EXISTS user_playtime (
       user_id INTEGER PRIMARY KEY,
       total_playtime BIGINT DEFAULT 0,
       last_updated TIMESTAMP DEFAULT NOW()
   );"
   
   cat playtime_export.csv | while IFS='|' read uid playtime; do
       psql -d phira_mp -c "INSERT INTO user_playtime VALUES ($uid, $playtime, NOW()) ON CONFLICT (user_id) DO UPDATE SET total_playtime = $playtime, last_updated = NOW();"
   done
   ```

---

## 2. 用户信息数据迁移

### 来源: `backend-extension/user.py`

原后端使用 SQLite 存储用户 Phira ID 映射和房间活动记录。

**迁移步骤:**

```bash
sqlite3 phira_stats.db "SELECT * FROM users;" 2>/dev/null | \
while IFS='|' read uid name phira_id; do
    psql -d phira_mp -c "
    INSERT INTO mp_users (user_id, username, phira_id) 
    VALUES ($uid, '$name', $phira_id)
    ON CONFLICT (user_id) DO NOTHING;"
done
```

---

## 3. 房间历史数据迁移

### 来源: `backend-remake` 的 phira-mp 模块

`backend-remake` 分支包含完整的 phira-mp 服务端，其房间数据存储在 SQLite/PostgreSQL 中。

**迁移步骤:**

1. 从旧 phira-mp 数据库导出房间记录：
   ```bash
   sqlite3 phira_mp.db "SELECT * FROM round_results;" > round_results.csv
   ```

2. 导入到 Phira-mp+ RoundStore：
   ```bash
   # Phira-mp+ 的 round_store 数据存储在 data/rounds/ 目录
   # 将 CSV 转换为 Phira-mp+ 可识别的格式
   python3 << 'EOF'
   import json, csv
   with open('round_results.csv') as f:
       reader = csv.reader(f)
       for row in reader:
           record = {
               "chart_id": int(row[0]),
               "player_id": int(row[1]),
               "score": int(row[2]),
               "accuracy": float(row[3]),
               "perfect": int(row[4]),
               "good": int(row[5]),
               "bad": int(row[6]),
               "miss": int(row[7]),
               "max_combo": int(row[8])
           }
           print(json.dumps(record))
   EOF
   ```

---

## 4. 从 `backend-remake` 迁移完整服务

`backend-remake` 分支包含以下组件，每个都有对应的 Phira-mp+ 实现：

| backend-remake 组件 | Phira-mp+ 替代 |
|---------------------|---------------|
| `phira/` (Phira API 代理) | `phira_client.rs` + `PhiraRetryClient` |
| `phira-mp/` (游戏服务端) | `phira-mp-plus-server` |
| `phira-web-monitor/` (Web 监控) | `/api/events` SSE + `/api/ws` WebSocket |
| `phira-mp-logprocessor/` (日志处理) | 内置 telemetry/auto-cleanup |

---

## 参考

- [HSNPhira backend-extension 分支](https://github.com/HyperSynapseNetwork/HSNPhira/tree/backend-extension)
- [HSNPhira backend-remake 分支](https://github.com/HyperSynapseNetwork/HSNPhira/tree/backend-remake)
- [Phira-mp+ WIT 插件 SDK](../../phira-plugin-sdk/)
