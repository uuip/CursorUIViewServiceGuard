# CursorUIViewService Guard

一个用于监测并自动重启无响应的 CursorUIViewService 进程的守护程序。CursorUIViewService 无响应后会导致输入卡顿。

## 功能

- 监测 `CursorUIViewService` 进程的响应状态
- 检测到进程无响应时自动终止该进程
- 使用 macOS 系统 API 准确判断进程响应状态

## 构建

### Rust 版本
```bash
cargo build --release
```

## 运行

### Rust 版本
```bash
cargo run --release
```

## 系统要求

- macOS 系统
- 对于 Rust 版本：Rust 1.70+

## 工作原理

程序通过以下步骤工作：

1. 搜索名为 `CursorUIViewService` 的进程
2. 使用 macOS 系统 API `CGSEventIsAppUnresponsive` 检测进程响应状态
3. 如果检测到进程无响应，发送 `SIGKILL` 信号终止进程
4. 循环监测，确保 Cursor 编辑器始终保持响应状态

## 注意事项

- 此程序专门为 macOS 系统设计
- 需要相应的系统权限来监测和终止进程
- 建议在后台运行以持续监测