# MiniNote

MiniNote 是一个本地优先的桌面便签应用。打开就写，保存到本机，需要时把笔记钉在桌面上，不需要账号、云同步或复杂知识库。

**简体中文** | [繁體中文](README_zh-TW.md) | [English](README_en-US.md) | [日本語](README_ja.md) | [한국어](README_ko.md) | [Deutsch](README_de.md) | [Français](README_fr.md) | [Español](README_es.md) | [Português](README_pt-BR.md) | [Italiano](README_it.md) | [Русский](README_ru.md)

## 适合做什么

- 临时记想法、命令、待办、会议片段或游戏/工作提示。
- 把常用文字固定在桌面上，随时查看和复制。
- 用一个小便签窗口快速记录，不打断当前工作。
- 用简单分类管理本地草稿。
- 用 Markdown 写标题、列表、引用、代码块，再快速预览。

## 主要功能

- **本地笔记库**：笔记、分类和设置保存在本机，默认不需要先选择文件。
- **快捷便签**：从托盘或全局快捷键打开小窗口，可设置为在鼠标附近出现。
- **桌面磁贴**：把某条笔记固定在屏幕上，支持自定义颜色和 Markdown 渲染。
- **Markdown 预览**：适合日常结构化文本，不追求完整 Markdown IDE。
- **文件导入导出**：支持 `.mint`、`.md`、`.markdown`、`.txt`。
- **源文件同步保护**：从外部文件导入后可记录来源；写回源文件前会检查外部修改，避免静默覆盖。
- **可调外观**：主题、字号、背景图片、快捷键、关闭到托盘等都可以在设置中调整。

## 支持的文件格式

| 格式                | 用途                                       |
| ------------------- | ------------------------------------------ |
| `.mint`             | MiniNote 默认文档类型，本质是 UTF-8 纯文本 |
| `.md` / `.markdown` | Markdown 文档                              |
| `.txt`              | 标准纯文本文件                             |

这些文件都可以用普通文本编辑器打开。MiniNote 不会把私有元数据写进文件正文。

## 安装和更新

官方预编译包发布在 [GitHub Releases](https://github.com/vivalucas/mininote/releases)。1.0.0 起正式发布只提供 Windows 和 macOS 包；Linux 相关代码保留，但暂不提供官方 Linux 发布包。

### Windows

- 安装版：`mininote-<version>-windows-x64-setup.exe`
- 便携版：`mininote-<version>-windows-x64.exe`

安装版适合长期使用；便携版适合临时试用或放在固定目录直接运行。

### macOS

Apple Silicon 设备下载 `mininote-<version>-macos-arm64.dmg`，打开后把 `MiniNote.app` 拖到 `Applications`。

当前 macOS 包没有正式签名。如果系统提示应用无法打开、已损坏，或提示来自未验证开发者，请先确认文件来自本项目 Release，然后在终端执行：

```bash
xattr -cr /Applications/MiniNote.app
```

再重新打开应用。更新时退出 MiniNote，下载新版 DMG 后覆盖 `Applications` 中的旧版本。

### Linux

1.0.0 暂不发布 Linux 安装包。需要 Linux 版本时，可以从源码自行构建；Linux 打包配置仍保留在仓库中。

## 数据存在哪里

MiniNote 不上传笔记，不提供云同步。笔记、配置和索引默认保存在系统应用数据目录下的 `MiniNote` 文件夹中。设置 `MININOTE_DATA_DIR` 环境变量后，MiniNote 会使用该位置作为数据目录。

## 从源码构建

需要 Node.js、Rust 和 Tauri 所需的系统依赖。

```bash
npm ci
npm run tauri build
```

开发模式：

```bash
npm run tauri dev
```

## 边界

- 不做账号和云同步。
- 不做富文本编辑器。
- 不做完整 Markdown IDE。
- 不做复杂知识库、双链或协作系统。

MiniNote 的目标是轻、快、本地、顺手。

## License

MIT License
