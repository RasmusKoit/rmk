# RMK Postcard-RPC 迁移计划

## 项目概述

本文档详细规划了将RMK键盘固件从Vial协议完全迁移到postcard-rpc协议的实现计划。迁移后将提供更现代化、类型安全、高性能的键盘配置解决方案。

本计划基于现有的RMK项目结构，充分利用已有的crate：
- `rmk` - 主固件crate，将在其中添加postcard-rpc服务
- `rmk-config` - 配置解析crate，可扩展支持postcard-rpc配置
- `rmk-macro` - 宏定义crate，可添加postcard-rpc相关宏
- `rmk-protocol` - 新创建的协议定义crate

## 功能范围分析

### 当前RMK支持的功能

#### 1. 基础系统功能
- 协议版本查询 (`GetProtocolVersion`)
- 固件版本/运行时间查询 (`GetKeyboardValue`)
- 存储重置 (`EepromReset`)
- 引导加载器跳转 (`BootloaderJump`)
- 键盘矩阵状态查询 (`SwitchMatrixState`) - 调试用

#### 2. 键位映射功能
- 单键位获取/设置 (`DynamicKeymapGetKeyCode`, `DynamicKeymapSetKeyCode`)
- 批量键位操作 (`DynamicKeymapGetBuffer`, `DynamicKeymapSetBuffer`)
- 层数查询 (`DynamicKeymapGetLayerCount`)
- 键位映射重置 (`DynamicKeymapReset`)

#### 3. 宏管理功能
- 宏数量查询 (`DynamicKeymapMacroGetCount`)
- 宏缓冲区大小查询 (`DynamicKeymapMacroGetBufferSize`)
- 宏数据获取/设置 (`DynamicKeymapMacroGetBuffer`, `DynamicKeymapMacroSetBuffer`)
- 宏重置 (`DynamicKeymapMacroReset`)

#### 4. 高级功能
- Tap Dance配置 (`DynamicVialTapDanceGet`, `DynamicVialTapDanceSet`)
- Combo配置 (`DynamicVialComboGet`, `DynamicVialComboSet`)
- 旋转编码器配置 (`GetEncoder`, `SetEncoder`)
- Vial锁定机制 (`GetUnlockStatus`, `UnlockStart`, `UnlockPoll`, `Lock`)

#### 5. 键盘定义和自定义设置
- 键盘ID和定义获取 (`GetKeyboardId`, `GetKeyboardDef`)
- 布局选项配置 (`LayoutOptions`)
- QMK设置查询 (`QmkSettingsQuery`)

## 项目架构设计

### 基于现有crate的架构

#### 1. rmk-protocol crate (新建)
协议定义专用crate，定义所有的endpoint和数据类型。

```
rmk-protocol/
├── Cargo.toml
└── src/
    ├── lib.rs                   # 协议导出
    ├── endpoints/               # 所有endpoint定义
    │   ├── mod.rs
    │   ├── system.rs           # 系统信息相关
    │   ├── keymap.rs           # 键位映射
    │   ├── macro_mgmt.rs       # 宏管理
    │   ├── advanced.rs         # 高级功能(TapDance,Combo等)
    │   └── settings.rs         # 自定义设置
    ├── types/                  # 数据类型定义
    │   ├── mod.rs
    │   ├── keycode.rs          # 复用rmk::keycode定义
    │   ├── matrix.rs           # 复用rmk::matrix定义
    │   ├── combo.rs            # 复用rmk::combo定义
    │   ├── tap_dance.rs        # 复用rmk::tap_dance定义
    │   └── system.rs
    └── errors.rs               # 错误定义
```

#### 2. rmk crate 扩展
在现有的rmk crate中添加postcard-rpc支持：

```
rmk/src/
├── postcard_rpc/               # 新增模块
│   ├── mod.rs
│   ├── service.rs             # PostcardRpcService实现
│   ├── transport/             # 传输层实现
│   │   ├── mod.rs
│   │   ├── usb_serial.rs     # USB串口传输
│   │   └── ble_serial.rs     # BLE串口传输(可选)
│   └── handlers/              # 请求处理器
│       ├── mod.rs
│       ├── system.rs          # 系统信息处理
│       ├── keymap.rs          # 键位映射处理
│       ├── macro_mgmt.rs      # 宏管理处理
│       └── advanced.rs        # 高级功能处理
└── via/                        # 现有via模块(后续可删除)
```

#### 3. rmk-config crate 扩展
添加postcard-rpc相关配置：

```toml
# keyboard.toml新增配置项
[postcard_rpc]
enabled = true
transport = "usb_serial"  # 或 "ble_serial"
buffer_size = 1024
```

#### 4. 客户端工具 (独立项目)
作为独立项目开发，不在RMK主仓库中：

```
rmk-client/                     # PC端客户端库
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── transport/
    └── keyboard_client.rs

rmk-cli/                        # CLI工具
├── Cargo.toml
└── src/
    └── main.rs
```

## 分阶段实现计划

### 阶段1: 基础架构搭建 (2周)

#### 目标
建立postcard-rpc基础框架，包括协议定义和在rmk crate中的集成。

#### 任务列表

##### 1.1 创建rmk-protocol crate (3天)
- [ ] 设置rmk-protocol基础结构
- [ ] 定义基础数据类型，复用rmk中已有类型
- [ ] 实现错误处理体系
- [ ] 创建endpoint宏定义基础设施

**交付物:**
- `rmk-protocol/src/lib.rs` - crate导出
- `rmk-protocol/src/types/mod.rs` - 类型导出
- `rmk-protocol/src/errors.rs` - 错误定义
- 基础的endpoint定义

##### 1.2 在rmk crate中实现传输层 (4天)
- [ ] 创建 `rmk/src/postcard_rpc/mod.rs` 模块
- [ ] 实现USB串口传输 (`usb_serial.rs`)
- [ ] 实现COBS编码支持
- [ ] 添加连接管理和重连机制

**交付物:**
- `rmk/src/postcard_rpc/transport/usb_serial.rs`
- `rmk/src/postcard_rpc/transport/mod.rs`
- 传输层trait定义和实现

##### 1.3 集成到现有RMK架构 (3天)
- [ ] 在rmk中添加postcard-rpc feature flag
- [ ] 修改Cargo.toml添加依赖
- [ ] 创建PostcardRpcService结构
- [ ] 集成到keyboard.rs主循环

**交付物:**
```rust
// rmk/Cargo.toml
[features]
postcard_rpc = ["dep:postcard-rpc", "dep:rmk-protocol"]

// rmk/src/postcard_rpc/service.rs
pub struct PostcardRpcService<'a> {
    keymap: &'a RefCell<KeyMap>,
    transport: UsbSerialTransport,
}
```

### 阶段2: 核心键位功能 (2周)

#### 目标
实现最核心的键位映射功能，在rmk crate中添加postcard-rpc处理器。

#### 任务列表

##### 2.1 在rmk-protocol中定义键位操作 (3天)
- [ ] 定义键位操作相关类型
- [ ] 定义键位相关endpoint
- [ ] 创建与rmk::action的转换函数

**交付物:**
```rust
// rmk-protocol/src/endpoints/keymap.rs
endpoint!(GetKeycode, "keymap/get", GetKeycodeRequest, GetKeycodeResponse);
endpoint!(SetKeycode, "keymap/set", SetKeycodeRequest, EmptyResponse);

// rmk-protocol/src/types/keycode.rs
// 复用rmk中的KeyAction等类型
use rmk::action::KeyAction;
pub type KeycodeType = u16;
```

##### 2.2 在rmk中实现键位处理器 (4天)
- [ ] 创建 `rmk/src/postcard_rpc/handlers/keymap.rs`
- [ ] 实现单键位获取/设置处理器
- [ ] 实现批量操作处理器
- [ ] 集成到PostcardRpcService

**交付物:**
```rust
// rmk/src/postcard_rpc/handlers/keymap.rs
pub async fn handle_get_keycode(
    req: GetKeycodeRequest, 
    keymap: &RefCell<KeyMap>
) -> Result<GetKeycodeResponse, RmkError>

pub async fn handle_set_keycode(
    req: SetKeycodeRequest,
    keymap: &RefCell<KeyMap>
) -> Result<EmptyResponse, RmkError>
```

##### 2.3 存储集成 (3天)
- [ ] 集成现有的FlashChannel
- [ ] 实现键位更改的持久化
- [ ] 保持与现有存储系统的兼容性

**交付物:**
- 与现有`channel::FLASH_CHANNEL`的集成
- 键位更改自动保存功能

### 阶段3: 系统功能实现 (1.5周)

#### 目标
实现系统信息查询和控制功能，复用现有rmk中的系统功能。

#### 任务列表

##### 3.1 系统信息endpoint定义 (2天)
- [ ] 在rmk-protocol中定义系统相关endpoint
- [ ] 复用现有ViaKeyboardInfo等类型
- [ ] 添加新的系统信息类型

**交付物:**
```rust
// rmk-protocol/src/endpoints/system.rs
endpoint!(GetSystemInfo, "system/info", InfoRequest, SystemInfoResponse);
endpoint!(GetProtocolVersion, "system/version", EmptyRequest, VersionResponse);
endpoint!(JumpBootloader, "system/bootloader", EmptyRequest, EmptyResponse);
```

##### 3.2 在rmk中实现系统处理器 (3天)
- [ ] 创建 `rmk/src/postcard_rpc/handlers/system.rs`
- [ ] 复用现有的boot::jump_to_bootloader()
- [ ] 实现系统信息查询(运行时间、固件版本等)
- [ ] 集成现有的存储重置功能

**交付物:**
- 系统信息处理器实现
- 与现有系统功能的集成

##### 3.3 调试功能 (2天)
- [ ] 实现矩阵状态查询(复用matrix_tester feature)
- [ ] 添加调试日志接口
- [ ] 保持与现有调试功能的兼容性

**交付物:**
```rust
#[cfg(feature = "matrix_tester")]
endpoint!(GetMatrixState, "debug/matrix", EmptyRequest, MatrixStateResponse);
```

### 阶段4: 高级功能实现 (2.5周)

#### 目标
实现Tap Dance、Combo、编码器等高级功能，复用rmk中现有实现。

#### 任务列表

##### 4.1 Tap Dance管理 (4天)
- [ ] 在rmk-protocol中定义TapDance endpoint
- [ ] 复用rmk::tap_dance中的数据结构
- [ ] 实现TapDance配置的获取/设置处理器
- [ ] 集成存储功能

**交付物:**
```rust
// rmk-protocol/src/endpoints/advanced.rs
endpoint!(GetTapDance, "tapdance/get", TapDanceRequest, TapDanceResponse);
endpoint!(SetTapDance, "tapdance/set", TapDanceConfig, EmptyResponse);

// 复用 rmk::tap_dance::TapDance
```

##### 4.2 Combo管理 (4天)
- [ ] 在rmk-protocol中定义Combo endpoint
- [ ] 复用rmk::combo中的数据结构
- [ ] 实现Combo配置的获取/设置处理器
- [ ] 保持现有的Combo排序逻辑

**交付物:**
```rust
endpoint!(GetCombo, "combo/get", ComboRequest, ComboResponse);
endpoint!(SetCombo, "combo/set", ComboConfig, EmptyResponse);

// 复用 rmk::combo::Combo
```

##### 4.3 编码器配置 (3天)
- [ ] 定义编码器endpoint
- [ ] 复用现有编码器配置结构
- [ ] 实现多层编码器配置支持
- [ ] 集成到现有的编码器系统

**交付物:**
- 编码器配置endpoint和处理器
- 与现有编码器功能的集成

##### 4.4 安全机制 (4天)
- [ ] 实现新的锁定机制(可选保留vial_lock feature)
- [ ] 添加操作权限验证
- [ ] 实现解锁流程
- [ ] 保持与现有vial_lock的兼容性

**交付物:**
```rust
#[cfg(feature = "vial_lock")]
endpoint!(GetLockStatus, "security/status", EmptyRequest, LockStatusResponse);
```

### 阶段5: 宏和存储功能 (1.5周)

#### 目标
实现宏定义管理和配置存储功能，复用现有的宏系统。

#### 任务列表

##### 5.1 宏管理系统 (4天)
- [ ] 定义宏相关endpoint
- [ ] 复用现有的keyboard_macros结构
- [ ] 实现宏获取/设置功能
- [ ] 保持与现有宏系统的兼容性

**交付物:**
```rust
// rmk-protocol/src/endpoints/macro_mgmt.rs
endpoint!(GetMacro, "macro/get", MacroRequest, MacroResponse);
endpoint!(SetMacro, "macro/set", MacroDefinition, EmptyResponse);

// 复用 rmk::keyboard_macros中的结构
```

##### 5.2 配置管理 (3天)
- [ ] 实现配置备份endpoint
- [ ] 实现配置恢复endpoint
- [ ] 添加配置版本管理
- [ ] 与rmk-config集成

**交付物:**
```rust
endpoint!(BackupConfig, "storage/backup", EmptyRequest, ConfigBackupResponse);
endpoint!(RestoreConfig, "storage/restore", ConfigRestoreRequest, EmptyResponse);
```

### 阶段6: 集成和优化 (1.5周)

#### 目标
完成系统集成，优化性能，完善功能。

#### 任务列表

##### 6.1 Dispatch系统实现 (3天)
- [ ] 在rmk中实现postcard-rpc dispatch
- [ ] 注册所有endpoint处理器
- [ ] 实现错误处理和恢复机制
- [ ] 集成到主循环

**交付物:**
```rust
// rmk/src/postcard_rpc/service.rs
impl PostcardRpcService {
    pub fn new(keymap: &RefCell<KeyMap>) -> Self;
    pub async fn run(&mut self);
}
```

##### 6.2 性能优化 (2天)
- [ ] 优化传输层性能
- [ ] 实现批量操作优化
- [ ] 添加缓存机制
- [ ] 性能测试和调优

##### 6.3 功能完善 (2天)
- [ ] 添加配置迁移工具
- [ ] 完善错误处理
- [ ] 添加日志和调试支持
- [ ] 文档编写

### 阶段7: 客户端工具开发 (1周)

#### 目标
开发基础的PC端工具，作为独立项目。

#### 任务列表

##### 7.1 客户端库 (3天)
- [ ] 创建rmk-client项目
- [ ] 实现基础客户端功能
- [ ] 添加所有endpoint的客户端方法
- [ ] 创建使用示例

##### 7.2 CLI工具 (4天)
- [ ] 创建rmk-cli项目
- [ ] 实现基础CLI命令
- [ ] 添加交互式模式
- [ ] 编写使用文档

## 实现细节和技术规范

### 与现有RMK系统的集成

#### Feature Flag配置
```toml
# rmk/Cargo.toml
[features]
postcard_rpc = ["dep:postcard-rpc", "dep:rmk-protocol", "dep:embedded-io-async"]
# 可选：保留vial兼容性一段时间
vial = ["dep:usbd-hid"]  # 现有via功能
```

#### 主循环集成
```rust
// rmk/src/keyboard.rs
#[cfg(feature = "postcard_rpc")]
let postcard_service = PostcardRpcService::new(&keymap);

// 在主循环中
#[cfg(feature = "postcard_rpc")]
postcard_service.run().await;
```

### 传输层规范

#### USB串口传输
- 使用embassy-usb的CDC-ACM类
- 波特率: 不适用(USB虚拟串口)
- 缓冲区大小: 1024字节
- 使用COBS编码确保帧完整性

#### 数据复用
- 直接使用rmk中的KeyAction、KeyCode等类型
- 避免重复定义，减少内存占用
- 通过rmk-protocol crate re-export需要的类型

### 错误处理规范

```rust
// rmk-protocol/src/errors.rs
#[derive(Serialize, Deserialize, Schema, Debug)]
pub enum RmkError {
    // 复用现有错误类型
    InvalidKeyPosition { layer: u8, row: u8, col: u8 },
    StorageError,
    
    // 新增postcard-rpc特定错误
    ProtocolError,
    TransportError,
}
```

### 内存优化

- 使用heapless::Vec代替std::Vec
- 复用现有的固定大小数组
- 避免动态分配
- 共享缓冲区设计

### 兼容性策略

#### 过渡期方案
1. 同时支持vial和postcard-rpc (通过feature flag)
2. 逐步迁移用户到新协议
3. 最终删除via模块

#### 配置兼容
- 支持从vial.json导入配置
- 提供配置转换工具
- 保持键位码兼容性

## 测试策略

### 单元测试
- 所有endpoint处理器100%覆盖
- 数据序列化/反序列化测试
- 错误处理路径测试

### 集成测试
- 端到端协议测试
- 多并发请求测试
- 错误恢复测试

### 兼容性测试
- 多种硬件平台验证
- 不同固件版本兼容性
- 客户端跨平台测试

## 部署和发布计划

### 阶段性发布
1. **Alpha版本** (阶段1-2完成): 核心键位功能
2. **Beta版本** (阶段1-4完成): 完整功能集
3. **RC版本** (阶段1-5完成): 生产就绪
4. **正式版本** (全部完成): 完整工具链

### 迁移支持
- 提供Vial配置导入工具
- 编写详细迁移文档
- 提供技术支持渠道

## 项目总结

### 关键变更

1. **基于现有crate结构**
   - 主要功能实现在rmk crate中的新模块
   - rmk-protocol作为独立的协议定义crate
   - 复用现有的数据结构和功能实现

2. **渐进式迁移**
   - 通过feature flag控制新旧协议
   - 最大程度复用现有代码
   - 减少破坏性变更

3. **优化内存和性能**
   - 避免重复定义类型
   - 使用no_std友好的数据结构
   - 针对嵌入式环境优化

### 预期收益

- **开发效率提升**: 类型安全的协议定义
- **维护性改善**: 清晰的模块划分
- **性能提升**: 更高效的序列化
- **扩展性增强**: 易于添加新功能

### 时间规划

- 总时长: 10-12周
- 阶段1-3: 基础功能实现 (5.5周)
- 阶段4-5: 高级功能 (4周)
- 阶段6-7: 集成和工具 (2.5周)

---

**文档版本**: 2.0  
**创建日期**: 2025-01-12  
**最后更新**: 2025-01-12  
**状态**: 基于现有RMK架构的修订版