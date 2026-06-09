[简体中文](readme.md) | [繁體中文](README_zh-TW.md) | **English** | [日本語](README_ja.md) | [한국어](README_ko.md) | [Deutsch](README_de.md) | [Français](README_fr.md) | [Español](README_es.md) | [Português](README_pt-BR.md) | [Italiano](README_it.md) | [Русский](README_ru.md)

# MiniNote

MiniNote is a local-first desktop notes app. Open it, write immediately, keep everything on your own machine, and pin notes to the desktop when you need them. No account, no cloud sync, no heavy knowledge-base workflow.

## What It Is For

- Capture ideas, commands, todos, meeting fragments, or work/game reminders.
- Keep reusable text pinned on the desktop for quick reading and copying.
- Open a small note window without interrupting your current task.
- Organize local drafts with simple categories.
- Write lightweight Markdown and preview headings, lists, quotes, and code blocks.

## Main Features

- **Local note library**: notes, categories, and settings are stored locally; you do not need to choose a file path first.
- **Quick notes**: open a small note window from the tray or a global shortcut; it can appear near the mouse cursor.
- **Desktop tiles**: pin a note on screen with custom colors and optional Markdown rendering.
- **Markdown preview**: useful for everyday structured text, not intended to be a full Markdown IDE.
- **Import and export**: supports `.mint`, `.md`, `.markdown`, and `.txt`.
- **Source-file sync protection**: imported files can keep a source link; MiniNote checks for external changes before writing back.
- **Custom appearance**: theme, font size, background image, shortcuts, and close-to-tray behavior are configurable.

## Supported Formats

| Format              | Use                                          |
| ------------------- | -------------------------------------------- |
| `.mint`             | MiniNote's default document type; UTF-8 text |
| `.md` / `.markdown` | Markdown document                            |
| `.txt`              | Standard plain text                          |

All supported files can still be opened in a normal text editor. MiniNote does not write private metadata into the file body.

## Install and Update

Official builds are published on [GitHub Releases](https://github.com/vivalucas/mininote/releases). Starting with 1.0.0, official release assets are provided for Windows and macOS only. Linux support remains in the source tree, but no official Linux package is published for now.

### Windows

- Installer: `mininote-<version>-windows-x64-setup.exe`
- Portable build: `mininote-<version>-windows-x64.exe`

Use the installer for regular use. Use the portable build when you want to try MiniNote temporarily or run it from a fixed folder.

### macOS

Apple Silicon users should download `mininote-<version>-macos-arm64.dmg`, open it, and drag `MiniNote.app` into `Applications`.

The macOS build is currently not formally signed. If macOS says the app cannot be opened, is damaged, or comes from an unidentified developer, first make sure the file came from this project's Release page, then run:

```bash
xattr -cr /Applications/MiniNote.app
```

Open the app again afterward. To update, quit MiniNote, download the new DMG, and replace the old app in `Applications`.

### Linux

MiniNote 1.0.0 does not ship official Linux packages. If you need a Linux build, build it from source; Linux packaging configuration is still kept in the repository.

## Where Data Lives

MiniNote does not upload notes and does not provide cloud sync. Notes, settings, and index data are stored in the system application data directory under `MiniNote`. If `MININOTE_DATA_DIR` is set, MiniNote uses that directory instead.

## Build From Source

You need Node.js, Rust, and the system dependencies required by Tauri.

```bash
npm ci
npm run tauri build
```

Development mode:

```bash
npm run tauri dev
```

## Boundaries

- No accounts or cloud sync.
- No rich text editor.
- No full Markdown IDE.
- No complex knowledge base, backlinks, or collaboration system.

MiniNote is meant to stay light, fast, local, and convenient.

## License

MIT License
