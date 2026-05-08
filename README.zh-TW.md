# MiniNote

輕量級純文字編輯器，參考 Windows 11 記事本設計。

[中文](README.md) | [English](README.en.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [Español](README.es.md) | [Português](README.pt-BR.md) | [Italiano](README.it.md) | [Русский](README.ru.md)

---

## 為什麼做這個

我從 Windows 換到 Mac，很多東西都適應了，但一直想念 Windows 11 新版記事本。

不是因為它多強大，恰恰是因為它夠簡單。開啟就寫，關了不了不丟，貼上過來就是乾淨文字。沒有格式，沒有富文字，沒有「智慧」提示。就是一個安靜的寫字的地方。

macOS 上找了很久，要麼太重（Obsidian、Typora），要麼太簡陋（系統自帶的文字編輯），要麼要訂閱。沒有一個剛好夠用的。

所以自己做了一個。

## 功能

- **分頁持久化**
  - 隨手新增分頁即可寫，內容即時暫存
  - 關機、重新開機、斷電，內容全部恢復，什麼都不丟
  - 關閉整個視窗靜默保留所有分頁，不彈任何提示
  - 關閉單個分頁時如有未儲存變更，彈出儲存/不儲存/取消

- **雙層儲存邏輯**
  - Session 層即時記錄一切，重新開機不丟
  - 磁碟層只在使用者明確 Cmd+S 時才寫入原檔案
  - 兩層互不干擾

- **無格式貼上**
  - 預設貼上即純文字，無需額外操作
  - 從網頁、微信、PDF 複製進來的文字，貼上自動去格式
  - 可當「格式清理中轉站」：富文字貼上進來 → 再複製出去 → 乾淨文字

- **Markdown 可選渲染**
  - 預設純文字編輯，`Cmd+R` 切換渲染檢視
  - 渲染檢視下不可編輯，切回純文字繼續寫
  - .mint / .txt / .md 三種檔案類型在設定中有獨立的渲染開關

- **Finder 整合**
  - 任意資料夾右鍵 → 「新增 MiniNote 文件」，直接建立 .mint 檔案
  - 支援 macOS 原生 Quick Look，選中檔案按空白鍵即可預覽
  - .mint 檔案預設關聯 MiniNote 開啟

- **其他特性**
  - 底部狀態列顯示行號/列號、字元數、編碼、換行符類型、渲染模式
  - 支援三種檔案格式：.mint（預設）、.txt、.md，另存為即可互相轉換
  - 主題切換：淺色 / 深色 / 依循系統
  - 設定頁面內可檢查 GitHub 更新

## 界面

```
+-------------------------------------------+
| 選單列                                     |
+-------------------------------------------+
| [未命名]  [notes.mint]  [ideas.md]  [+]   |
+-------------------------------------------+
|                                           |
|              文件編輯區域                  |
|                                           |
+-------------------------------------------+
| 狀態列（行/列 | 字元數 | UTF-8 | LF | 文字）|
+-------------------------------------------+
```

## 快速鍵

| 功能 | 快速鍵 |
|------|--------|
| 新增 | `Cmd+N` |
| 開啟 | `Cmd+O` |
| 儲存 | `Cmd+S` |
| 另存為 | `Cmd+Shift+S` |
| 關閉分頁 | `Cmd+W` |
| 復原 / 重做 | `Cmd+Z` / `Cmd+Shift+Z` |
| 尋找 | `Cmd+F` |
| 尋找與取代 | `Cmd+Option+F` |
| Markdown 渲染切換 | `Cmd+R` |
| 偏好設定 | `Cmd+,` |

## 支援的格式

| 格式 | 說明 |
|------|------|
| .mint | MiniNote 專屬格式，預設新增。純文字 + 輕量狀態資訊（游標位置、渲染狀態） |
| .txt | 標準純文字，相容其他編輯器 |
| .md | Markdown 格式 |

三種格式本質都是純文字，互相轉換就是改個副檔名的事。

## 系統要求

- macOS 26（Tahoe）或更高版本
- Apple Silicon Mac（M 系列晶片）

## 安裝

**方式一：DMG 安裝包**

1. 進入本倉庫的 [Releases](../../releases) 頁面，下載最新的 `MiniNote-[版本號].dmg`
2. 開啟 DMG，將 MiniNote 拖入 Applications 資料夾
3. 首次啟動時，macOS 可能提示「應用程式已損壞」或「無法驗證開發者」——這是 Gatekeeper 對未付費簽名應用程式的正常攔截，應用程式本身完好。在終端機執行以下命令移除隔離標記：
   ```bash
   xattr -cr /Applications/MiniNote.app
   ```
   之後雙擊即可正常啟動；或右鍵點擊 → 開啟 → 彈窗中再點「開啟」。

**方式二：ZIP 壓縮包**

1. 進入本倉庫的 [Releases](../../releases) 頁面，下載最新的 `MiniNote-[版本號].zip` 並解壓縮
2. 將 `MiniNote.app` 移動至 Applications 資料夾
3. 在終端機執行：
   ```bash
   xattr -cr /Applications/MiniNote.app
   ```

**方式三：自行編譯（無需繞過簽名）**

1. 複製本倉庫
2. 用 Xcode 開啟 `MiniNote.xcodeproj`
3. 在 **Signing & Capabilities** 中選擇你自己的開發者帳號
4. `⌘R` 執行，Xcode 會自動簽名

## 使用

- **新增分頁**：`Cmd+N` 建立暫存文件，直接開始寫，無需命名或選擇路徑
- **開啟檔案**：`Cmd+O` 開啟磁碟上的 .mint / .txt / .md 檔案
- **儲存**：`Cmd+S` 儲存當前分頁到磁碟；暫存文件會彈出另存為對話框
- **另存為**：`Cmd+Shift+S` 可將當前文件另存為不同格式（.mint ↔ .txt ↔ .md）
- **切換渲染**：`Cmd+R` 在純文字編輯和 Markdown 渲染檢視之間切換
- **尋找取代**：`Cmd+F` 尋找，`Cmd+Option+F` 尋找並取代
- **分頁排序**：拖曳分頁即可調整順序
- **關閉分頁**：`Cmd+W`，有未儲存變更時會彈出儲存提示
- **檢查更新**：偏好設定（`Cmd+,`）內點內點擊「檢查更新」

## 常見問題

**暫存文件存在哪裡？**

`~/Library/Application Support/MiniNote/sessions/` 目錄下。每個暫存文件一個檔案，外加一個 `session.json` 記錄分頁順序和元資訊。

**.mint 和 .txt 有什麼區別？**

本質完全相同，都是純文字。區別僅在於 .mint 會額外儲存游標位置和渲染狀態等輕量資訊，.txt 是標準純文字不寫入額外資訊。兩者隨時可通過另存為互相轉換，內容不變。

**支援語法高亮嗎？**

不支援。MiniNote 是純文字編輯器，保持純粹。Markdown 渲染基於系統 AttributedString，提供基礎的標題、列表、粗體等渲染，不做程式碼語法高亮。

**和系統文字編輯 / CotEditor / BBEdit 有什麼區別？**

MiniNote 的核心理念是：分頁持久化（重新開機不丟）+ 雙層儲存邏輯（暫存 vs 磁碟分離）。系統文字編輯不支援分頁持久化；CotEditor 功能更豐富但無此機制；BBEdit 太重。MiniNote 只做好這一件事。

**支援雲端同步嗎？**

不支援，也不會支援。所有資料存在本地，完全離線。

## 開發

技術棧：Swift 6 + SwiftUI + NSTextView（TextKit），零第三方依賴。

```bash
git clone https://github.com/vivalucas/mininote.git
open MiniNote.xcodeproj
# ⌘B 构建
```

## 授權

MIT License
