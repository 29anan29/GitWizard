# GitWizard

**Git 的自动化流程控制台** —— 意图驱动（Intent-Driven）的桌面 Git 工具。跟随步骤向导即可完成版本控制任务，无需记忆命令行，也**无需本机安装 Git**（内置静态链接的 libgit2 引擎）。

> 联系 / Maintainer: anwang13@outlook.com

## 文档

- `PRD.md` 产品需求
- `DESIGN.md` 设计系统（Cursor 风格暖色主题）
- `ROADMAP.md` 路线图与里程碑

## 技术栈

Tauri v2 (Rust) + SvelteKit + TypeScript，git2-rs 静态链接（无需系统安装 Git）。

## 开发

```bash
pnpm install
pnpm tauri dev
```

## 构建

```bash
pnpm tauri build
```

## 发布

推送 tag 触发 GitHub Actions 多平台打包（deb / rpm / AppImage / msi / exe / dmg / pkg，覆盖 x64 与 ARM）：

```bash
git tag v0.1.0 && git push origin v0.1.0
```
