# MiniNote

A lightweight plain text editor for macOS, inspired by the Windows 11 Notepad experience.

[中文](README.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [Русский](README.ru.md)

---

## Why I Built This

I switched from Windows to Mac and got used to most things, but I kept missing the Windows 11 Notepad.

Not because it's powerful -- precisely because it's simple. Open it and write, close it without losing anything, paste and get clean text. No formatting, no rich text, no "smart" suggestions. Just a quiet place to write.

I searched for a long time on macOS. Either too heavy (Obsidian, Typora), too barebones (the built-in TextEdit), or subscription-based. Nothing that was just right.

So I built my own.

## Features

- **Tab Persistence**
  - Create a new tab and start writing, content auto-saves in real time
  - Shutdown, restart, power loss -- everything is restored, nothing is lost
  - Closing the window silently preserves all tabs, no prompts
  - Closing a single tab with unsaved changes shows Save / Don't Save / Cancel

- **Two-Layer Save Logic**
  - Session layer records everything in real time, survives restarts
  - Disk layer only writes to the file on explicit Cmd+S
  - The two layers operate independently

- **Paste as Plain Text**
  - Paste is plain text by default, no extra steps needed
  - Text copied from web pages, WeChat, PDFs is automatically stripped of formatting
  - Works as a "format cleaning station": paste rich text in, copy it out, get clean text

- **Optional Markdown Rendering**
  - Plain text editing by default, Cmd+R to toggle rendered view
  - Rendered view is read-only, switch back to plain text to continue editing
  - .mint / .txt / .md each have independent render toggles in Settings

- **Finder Integration**
  - Right-click any folder in Finder to create a new .mint file
  - Native macOS Quick Look support -- select a file and press Space to preview
  - .mint files open with MiniNote by default

- **Other**
  - Status bar shows line/column, character count, encoding, line ending, render mode
  - Three file formats: .mint (default), .txt, .md -- convert via Save As
  - Theme switching: Light / Dark / Follow System
  - Check for GitHub updates from Settings

## Layout

```
+-------------------------------------------+
| Menu Bar                                  |
+-------------------------------------------+
| [Untitled]  [notes.mint]  [ideas.md]  [+] |
+-------------------------------------------+
|                                           |
|              Editor Area                  |
|                                           |
+-------------------------------------------+
| Status Bar (Ln/Col | Chars | UTF-8 | LF | Text) |
+-------------------------------------------+
```

## Keyboard Shortcuts

| Function | Shortcut |
|----------|----------|
| New | `Cmd+N` |
| Open | `Cmd+O` |
| Save | `Cmd+S` |
| Save As | `Cmd+Shift+S` |
| Close Tab | `Cmd+W` |
| Undo / Redo | `Cmd+Z` / `Cmd+Shift+Z` |
| Find | `Cmd+F` |
| Find & Replace | `Cmd+Option+F` |
| Toggle Markdown | `Cmd+R` |
| Settings | `Cmd+,` |

## Supported Formats

| Format | Description |
|--------|-------------|
| .mint | MiniNote native format, default for new files. Plain text + lightweight state info (cursor position, render state) |
| .txt | Standard plain text, compatible with other editors |
| .md | Markdown format |

All three are plain text at their core. Converting between them is just changing the extension.

## Requirements

- macOS 26 (Tahoe) or later
- Apple Silicon Mac (M-series chips)

## Installation

**Option 1: DMG Installer**

1. Go to the [Releases](../../releases) page and download the latest `MiniNote-[version].dmg`
2. Open the DMG and drag MiniNote into your Applications folder
3. On first launch, macOS may say "app is damaged" or "cannot verify developer" -- this is normal Gatekeeper behavior for unsigned apps. Run this in Terminal to remove the quarantine flag:
   ```bash
   xattr -cr /Applications/MiniNote.app
   ```
   Then double-click to launch; or right-click, Open, then click Open in the dialog.

**Option 2: ZIP Archive**

1. Go to the [Releases](../../releases) page and download the latest `MiniNote-[version].zip`, then unzip
2. Move `MiniNote.app` to your Applications folder
3. Run in Terminal:
   ```bash
   xattr -cr /Applications/MiniNote.app
   ```

**Option 3: Build from Source (no signing workaround needed)**

1. Clone this repository
2. Open `MiniNote.xcodeproj` in Xcode
3. In **Signing & Capabilities**, select your own developer account
4. `Cmd+R` to run -- Xcode handles signing automatically

## Usage

- **New Tab**: `Cmd+N` creates a scratch document, start writing immediately
- **Open File**: `Cmd+O` opens .mint / .txt / .md files from disk
- **Save**: `Cmd+S` saves the current tab to disk; scratch documents trigger Save As
- **Save As**: `Cmd+Shift+S` saves as a different format (.mint / .txt / .md)
- **Toggle Rendering**: `Cmd+R` switches between plain text editing and Markdown rendered view
- **Find & Replace**: `Cmd+F` to find, `Cmd+Option+F` to find and replace
- **Reorder Tabs**: Drag tabs to rearrange
- **Close Tab**: `Cmd+W`, prompts to save if there are unsaved changes
- **Check Updates**: Settings (`Cmd+,`) has a "Check for Updates" button

## FAQ

**Where are scratch documents stored?**

In `~/Library/Application Support/MiniNote/sessions/`. Each scratch document is a separate file, plus a `session.json` that records tab order and metadata.

**What's the difference between .mint and .txt?**

They're identical at their core -- both plain text. The only difference is that .mint additionally saves cursor position and render state. You can convert between them freely via Save As without losing content.

**Does it support syntax highlighting?**

No. MiniNote is a plain text editor, keeping it pure. Markdown rendering uses the system's AttributedString for basic headings, lists, bold, etc. No code syntax highlighting.

**How is it different from TextEdit / CotEditor / BBEdit?**

MiniNote's core philosophy is: tab persistence (survives restart) + two-layer save logic (scratch vs. disk separation). TextEdit doesn't support tab persistence; CotEditor is more feature-rich but lacks this mechanism; BBEdit is too heavy. MiniNote does just this one thing well.

**Does it support cloud sync?**

No, and it never will. All data is stored locally, completely offline.

## Development

Tech stack: Swift 6 + SwiftUI + NSTextView (TextKit), zero third-party dependencies.

```bash
git clone https://github.com/vivalucas/mininote.git
open MiniNote.xcodeproj
# Cmd+B to build
```

## License

MIT License
