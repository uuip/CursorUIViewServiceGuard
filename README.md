# CursorUIViewService Guard

一个用于监测并自动重启无响应的 CursorUIViewService 进程的守护程序。CursorUIViewService 无响应后会导致输入卡顿。

## 功能

- 监测 `CursorUIViewService` 进程的响应状态
- 检测到进程无响应时自动终止该进程
- 使用 macOS 系统 API 准确判断进程响应状态

## 构建

```bash
cargo build --release
```

## 注意事项

- 此程序专门为 macOS 系统设计
- 需要相应的系统权限来监测和终止进程
- 建议在后台运行以持续监测