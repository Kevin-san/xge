# 示例实现指南

## 概述

本文档提供 Sprint 15 中所有示例工程的详细实现指南，帮助开发者理解如何使用 `engine-network`、`engine-hotfix` 和 `engine-plugin` crate。

---

## 1. network_chat - TCP 聊天室

### 功能说明
TCP 文本聊天室，支持多客户端连接、昵称、房间和广播消息。

### 启动方式

**服务端：**
```bash
cargo run --example network_chat -- --server 0.0.0.0:8080
```

**客户端：**
```bash
cargo run --example network_chat -- --client 127.0.0.1:8080 --name Alice
```

### 实现要点

#### 服务端结构

```rust
struct ChatServer {
    clients: HashMap<ClientId, ClientInfo>,
    rooms: HashMap<RoomId, Room>,
}

struct ClientInfo {
    channel: TcpChannel,
    nickname: String,
    current_room: Option<RoomId>,
}
```

#### 客户端结构

```rust
struct ChatClient {
    channel: TcpChannel,
    nickname: String,
    ui: ChatUI,
}
```

#### 消息协议

| 消息类型 | 格式 | 说明 |
| :--- | :--- | :--- |
| `JoinRoom` | `{ room_id, password? }` | 加入房间 |
| `CreateRoom` | `{ name, max_players, password? }` | 创建房间 |
| `ListRooms` | `{}` | 获取房间列表 |
| `SendMessage` | `{ room_id, content }` | 发送消息 |
| `ChatMessage` | `{ from, content, timestamp }` | 广播消息 |
| `UserJoined` | `{ client_id, nickname }` | 用户加入 |
| `UserLeft` | `{ client_id }` | 用户离开 |

---

## 2. network_echo - UDP Echo 服务

### 功能说明
UDP 回显服务器，支持可靠/不可靠模式切换。

### 启动方式

**服务端：**
```bash
cargo run --example network_echo -- --server 0.0.0.0:8081
```

**客户端：**
```bash
cargo run --example network_echo -- --client 127.0.0.1:8081 --reliable
```

### 实现要点

```rust
// 服务端循环
loop {
    if let Some((addr, data)) = udp.recv_from().await? {
        // 回显数据
        udp.send_to(addr, &data, reliability).await?;
    }
}
```

---

## 3. network_rpc - RPC 调用示例

### 功能说明
展示 `#[rpc]` 宏的客户端调用与服务端响应。

### 启动方式

**服务端：**
```bash
cargo run --example network_rpc -- --server 0.0.0.0:8082
```

**客户端：**
```bash
cargo run --example network_rpc -- --client 127.0.0.1:8082
```

### 实现要点

#### 定义 RPC 服务

```rust
#[rpc(server)]
impl GameService {
    #[rpc_method]
    async fn get_player_info(&self, player_id: u64) -> Result<PlayerInfo> {
        // 查询玩家信息
    }
    
    #[rpc_method(one_way)]
    async fn log_event(&self, event: GameEvent) {
        // 记录事件（无需返回）
    }
    
    #[rpc_method(timeout = "10s")]
    async fn heavy_computation(&self, input: ComplexInput) -> Result<ComplexOutput> {
        // 耗时操作
    }
}
```

#### 客户端调用

```rust
let client = RpcClient::new(channel);
let player_info = client.get_player_info(player_id).await?;
```

---

## 4. network_replication - Entity 位置同步

### 功能说明
2D 方块在多客户端间位置同步，演示预测和插值。

### 启动方式

**服务端：**
```bash
cargo run --example network_replication -- --server 0.0.0.0:8083
```

**客户端：**
```bash
cargo run --example network_replication -- --client 127.0.0.1:8083
```

### 实现要点

#### 同步组件

```rust
#[derive(Component, NetworkMessage)]
struct PlayerPosition {
    #[net_message(Interpolate)]
    pos: Vec2,
    
    #[net_message(Predict)]
    vel: Vec2,
    
    #[net_message(OnlyOwner)]
    input: InputState,
}
```

#### 客户端预测

```rust
// 记录输入和预测状态
prediction_buffer.push(tick, input, current_state);

// 应用预测
let predicted_state = predict_next_state(current_state, input);

// 收到权威状态时校正
if let Some(authoritative) = receive_snapshot().await {
    prediction_buffer.replay_from(authoritative.tick, authoritative.state);
}
```

#### 客户端插值

```rust
// 添加状态到缓冲区
interpolation_buffer.push(tick, state);

// 采样插值结果
let interpolated = interpolation_buffer.sample(current_time);
```

---

## 5. network_lobby - 大厅与房间匹配

### 功能说明
大厅 + 房间创建/加入 + 匹配算法展示。

### 启动方式

```bash
cargo run --example network_lobby
```

### 实现要点

#### 大厅系统

```rust
let mut lobby = Lobby::new(100);

// 创建房间
let room_id = lobby.create_room(creator_id, RoomConfig {
    name: "My Room".into(),
    max_players: 4,
    password: None,
    is_private: false,
    game_mode: "deathmatch".into(),
});

// 加入房间
lobby.join_room(client_id, room_id)?;

// 列出房间
let rooms = lobby.list_rooms();
```

#### 匹配系统

```rust
let mut matchmaker = Matchmaker::new(&lobby);

// 加入匹配队列
let queue_id = matchmaker.enqueue(client_id, PlayerProfile {
    skill: 1500, // ELO 分数
    ping: 30,    // 延迟 ms
    preferences: vec!["deathmatch".into()],
});

// 执行匹配
matchmaker.tick();
```

---

## 6. network_replay - 对战回放

### 功能说明
记录对战并支持回放、跳转、倍速播放。

### 启动方式

**记录：**
```bash
cargo run --example network_replay -- record output.replay
```

**回放：**
```bash
cargo run --example network_replay -- play output.replay --speed 2.0
```

### 实现要点

#### 记录流程

```rust
let mut recorder = ReplayRecorder::new("match.replay")?;

// 每帧记录
loop {
    let msg = network.recv().await?;
    recorder.record(current_tick, &msg);
    
    if game_ended {
        recorder.flush().await?;
        recorder.close().await?;
        break;
    }
}
```

#### 回放流程

```rust
let mut player = ReplayPlayer::new("match.replay")?;

// 设置播放速度
player.speed(2.0);

// 跳转到指定帧
player.seek_to(1000).await?;

// 逐帧播放
while let Some(frame) = player.next_frame().await? {
    apply_frame_to_world(frame);
}
```

---

## 7. hotfix_patch - 差分补丁工具

### 功能说明
CLI 工具，支持生成和应用差分补丁。

### 启动方式

**生成 patch：**
```bash
cargo run --example hotfix_patch -- diff old.bin new.bin out.patch
```

**应用 patch：**
```bash
cargo run --example hotfix_patch -- patch old.bin out.patch new.bin
```

### 实现要点

```rust
// 生成差分
DiffEngine::bsdiff("old.bin", "new.bin", "out.patch")?;

// 应用差分
DiffEngine::bspatch("old.bin", "out.patch", "new.bin")?;

// 创建带签名的 patch bundle
let mut bundle = PatchBundle::new("1.0.0", "1.0.1");
bundle.add_file("assets/texture.png", diff_data);
bundle.sign_ed25519(&private_key)?;

// 验证并应用
if bundle.verify_ed25519(&public_key, &signature) {
    bundle.apply("base_dir")?;
}
```

---

## 8. hotfix_script - 脚本热重载

### 功能说明
JS/Py/Lua 脚本热重载，修改源码后自动生效。

### 启动方式

```bash
cargo run --example hotfix_script --lang js --watch scripts/
```

### 实现要点

```rust
// 创建脚本运行时
let mut runtime = ScriptRuntime::new(ScriptLang::Js);

// 加载脚本
let handle = runtime.load("scripts/game.js")?;

// 监听文件变化自动重载
let mut watcher = ScriptFileWatcher::new(runtime.clone(), "scripts/");

// 调用脚本函数
let result = runtime.call(handle, "update", &[delta_time])?;
```

---

## 9. hotfix_asset - 资源热重载

### 功能说明
运行时修改纹理，观察画面立即刷新。

### 启动方式

```bash
cargo run --example hotfix_asset --watch assets/
```

### 实现要点

```rust
// 注册资源
let texture_handle = AssetHotreload::register_texture("assets/texture.png", handle);

// 轮询变化
loop {
    asset_hotreload.tick();
    
    // 渲染
    render_with_texture(texture_handle);
}

// 监听回调
asset_hotreload.on_change("assets/texture.png", |path| {
    // 资源变化时的处理
    log::info!("Texture changed: {}", path);
});
```

---

## 10. hotfix_grey - 灰度发布

### 功能说明
构造 user profile 并展示灰度匹配逻辑。

### 启动方式

```bash
cargo run --example hotfix_grey
```

### 实现要点

```rust
// 配置灰度规则
let mut grey = GreyRelease::new();
grey.by_channel(vec!["beta".into()]);
grey.by_version("^1.0.0".parse()?);
grey.by_ratio(0.3); // 30% 用户

// 检查用户是否在灰度范围内
let user = UserProfile {
    channel: "beta".into(),
    os_version: "10.15".into(),
    device_model: "iPhone12".into(),
    region: "CN".into(),
    user_id_hash: 123456,
};

if grey.match_user(&user) {
    // 应用新版本
    apply_update();
}
```

---

## 11. plugin_hello - 最小插件示例

### 功能说明
加载一个 Rust dylib 插件并 tick。

### 启动方式

```bash
cargo run --example plugin_hello --plugin ./plugins/hello.wasm
```

### 实现要点

#### 插件实现

```rust
#[derive(Default)]
struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn name(&self) -> &str { "hello" }
    fn version(&self) -> Version { Version::parse("1.0.0").unwrap() }
    
    fn on_load(&mut self, world: &mut World, registry: &mut PluginRegistry) {
        println!("HelloPlugin loaded!");
    }
    
    fn on_tick(&mut self, world: &mut World, dt: f32) {
        println!("Tick: {}ms", dt * 1000.0);
    }
    
    fn on_unload(&mut self, world: &mut World) {
        println!("HelloPlugin unloaded!");
    }
}

plugin_export!(HelloPlugin);
```

#### 加载插件

```rust
let mut lifecycle = PluginLifecycle::new(world, registry);
let handle = lifecycle.load("plugins/hello").await?;
```

---

## 12. plugin_ui_widget - 自定义 UI Widget

### 功能说明
插件注册一个自定义 UI Widget（如心形按钮）。

### 启动方式

```bash
cargo run --example plugin_ui_widget
```

### 实现要点

```rust
// 在插件中注册 Widget
fn on_load(&mut self, world: &mut World, registry: &mut PluginRegistry) {
    registry.register_ui_widget(HeartButtonWidget);
}

// 自定义 Widget 实现
struct HeartButtonWidget;

impl UiWidget for HeartButtonWidget {
    fn draw(&mut self, ui: &mut Ui) {
        if ui.button("❤️ Like").clicked() {
            // 处理点击
        }
    }
}
```

---

## 13. plugin_render_pass - 自定义渲染 Pass

### 功能说明
插件注册一个后处理渲染 Pass（如 bloom）。

### 启动方式

```bash
cargo run --example plugin_render_pass
```

### 实现要点

```rust
// 在插件中注册渲染 Pass
fn on_load(&mut self, world: &mut World, registry: &mut PluginRegistry) {
    registry.register_render_pass(BloomPass::new());
}

// Bloom 后处理 Pass
struct BloomPass {
    shader: Shader,
    framebuffer: Framebuffer,
}

impl RenderPass for BloomPass {
    fn render(&mut self, renderer: &mut Renderer) {
        // 应用 bloom 效果
        renderer.apply_post_process(&self.shader, &self.framebuffer);
    }
}
```

---

## 14. plugin_ffi - C ABI FFI 插件

### 功能说明
从 C 侧调用 engine 的 FFI 插件示例。

### 启动方式

```bash
cargo run --example plugin_ffi
```

### 实现要点

#### C 接口定义

```c
// plugin.h
typedef void* PluginHandle;

PluginHandle plugin_init();
void plugin_update(PluginHandle handle, float dt);
void plugin_shutdown(PluginHandle handle);
```

#### Rust 实现

```rust
#[no_mangle]
pub extern "C" fn plugin_init() -> *mut c_void {
    let plugin = Box::new(MyPlugin::new());
    Box::into_raw(plugin) as *mut c_void
}

#[no_mangle]
pub extern "C" fn plugin_update(handle: *mut c_void, dt: f32) {
    let plugin = unsafe { &mut *(handle as *mut MyPlugin) };
    plugin.update(dt);
}

#[no_mangle]
pub extern "C" fn plugin_shutdown(handle: *mut c_void) {
    unsafe { Box::from_raw(handle as *mut MyPlugin) };
}
```

---

## 附录：manifest.toml 示例

```toml
name = "my_plugin"
version = "1.0.0"
description = "A sample plugin"
authors = ["Developer <dev@example.com>"]
kind = "RustDylib"
entry_point = "libmy_plugin.so"

[dependencies]
engine-core = "^0.15.0"

[permissions]
file_read = ["data/*"]
file_write = ["data/save/*"]
net_connect = [("api.example.com", "80-443")]
memory_limit = "100MB"
cpu_limit = "10"  # seconds per minute
```