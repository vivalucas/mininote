[简体中文](readme.md) | **繁體中文** | [English](README_en-US.md) | [日本語](README_ja.md) | [한국어](README_ko.md) | [Deutsch](README_de.md) | [Français](README_fr.md) | [Español](README_es.md) | [Português](README_pt-BR.md) | [Italiano](README_it.md) | [Русский](README_ru.md)

# MiniNote

[![GitHub Release](https://img.shields.io/github/v/release/vivalucas/mininote?style=flat-square)](https://github.com/vivalucas/mininote/releases) [![License](https://img.shields.io/github/license/vivalucas/mininote?style=flat-square)](LICENSE) [![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey?style=flat-square)]()

MiniNote 是一款本機優先的桌面便箋應用。打開就寫，內容保存在自己的電腦上，需要時把筆記釘在桌面，不需要帳號、雲端同步或複雜的知識庫流程。

## 技术栈

- **框架**: Tauri 2 (Rust)
- **前端**: React 19 + TypeScript
- **样式**: TailwindCSS
- **性能**: 核心编辑器采用非受控架构与防抖渲染，实现长文本全速打字零卡顿体验。

## 適合做什麼

- 臨時記錄想法、指令、待辦、會議片段，或工作/遊戲提示。
- 把常用文字固定在桌面上，方便隨時查看和複製。
- 用一個小便箋視窗快速記錄，不打斷目前工作。
- 用簡單分類管理本機草稿。
- 用 Markdown 撰寫標題、列表、引用、程式碼區塊，並快速預覽。

## 主要功能

- **本機筆記庫**：筆記、分類和設定保存在本機，預設不需要先選擇檔案路徑。
- **快捷便箋**：從系統匣或全域快速鍵打開小視窗，也可以設定為在滑鼠附近出現。
- **桌面磁貼**：把某條筆記固定在螢幕上，支援自訂顏色和 Markdown 渲染。
- **Markdown 預覽**：適合日常結構化文字，不追求完整 Markdown IDE。
- **檔案匯入匯出**：支援匯入單一檔案，或將資料夾匯入為分類；支援 `.mint`、`.md`、`.markdown`、`.txt`。
- **來源檔案同步保護**：從外部檔案匯入、或匯出到檔案後可記錄關聯檔案；寫回前會檢查外部修改，避免靜默覆蓋。
- **可調外觀**：主題、字號、背景圖片、快速鍵、關閉到系統匣等都可以在設定中調整。

## 支援的檔案格式

| 格式                | 用途                                       |
| ------------------- | ------------------------------------------ |
| `.mint`             | MiniNote 預設文件類型，本質是 UTF-8 純文字 |
| `.md` / `.markdown` | Markdown 文件                              |
| `.txt`              | 標準純文字檔                               |

這些檔案都可以用一般文字編輯器打開。MiniNote 不會把私有中繼資料寫進檔案正文。

## 安裝和更新

官方預編譯套件發布在 [GitHub Releases](https://github.com/vivalucas/mininote/releases)。1.0.0 起正式發布只提供 Windows 和 macOS 套件；Linux 相關程式碼保留，但暫不提供官方 Linux 發布套件。

### Windows

- 安裝版：`mininote-<version>-windows-x64-setup.exe`
- 可攜版：`mininote-<version>-windows-x64.exe`

安裝版適合長期使用；可攜版適合臨時試用，或放在固定資料夾直接執行。

### macOS

Apple Silicon 裝置下載 `mininote-<version>-macos-arm64.dmg`，打開後把 `MiniNote.app` 拖到 `Applications`。

目前 macOS 套件沒有正式簽名。如果系統提示應用無法打開、已損毀，或提示來自未驗證開發者，請先確認檔案來自本專案 Release，然後在終端機執行：

```bash
xattr -cr /Applications/MiniNote.app
```

再重新打開應用。更新時請先退出 MiniNote，下載新版 DMG 後覆蓋 `Applications` 中的舊版本。

### Linux

1.0.0 暫不發布 Linux 安裝套件。需要 Linux 版本時，可以從原始碼自行構建；Linux 打包設定仍保留在倉庫中。

## 資料存在哪裡

MiniNote 不上傳筆記，也不提供雲端同步。筆記、設定和索引資料預設保存在系統應用資料目錄下的 `MiniNote` 資料夾中。設定 `MININOTE_DATA_DIR` 環境變數後，MiniNote 會使用該位置作為資料目錄。

## 從原始碼構建

需要 Node.js、Rust 和 Tauri 所需的系統依賴。

```bash
npm ci
npm run tauri build
```

開發模式：

```bash
npm run tauri dev
```

## 邊界

- 不做帳號和雲端同步。
- 不做富文字編輯器。
- 不做完整 Markdown IDE。
- 不做複雜知識庫、雙向連結或協作系統。

MiniNote 的目標是輕、快、本機、順手。

## License

MIT License
