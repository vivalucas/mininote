[简体中文](readme.md) | [繁體中文](README_zh-TW.md) | [English](README_en-US.md) | **日本語** | [한국어](README_ko.md) | [Deutsch](README_de.md) | [Français](README_fr.md) | [Español](README_es.md) | [Português](README_pt-BR.md) | [Italiano](README_it.md) | [Русский](README_ru.md)

# MiniNote

[![GitHub Release](https://img.shields.io/github/v/release/vivalucas/mininote?style=flat-square)](https://github.com/vivalucas/mininote/releases) [![License](https://img.shields.io/github/license/vivalucas/mininote?style=flat-square)](LICENSE) [![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey?style=flat-square)]()

MiniNote はローカルファーストのデスクトップメモアプリです。開いてすぐ書き、内容は自分のマシンに保存し、必要なときはノートをデスクトップに固定できます。アカウント、クラウド同期、重いナレッジベース運用は不要です。

## Technology Stack

- **Framework**: Tauri 2 (Rust)
- **Frontend**: React 19 + TypeScript
- **Styling**: TailwindCSS
- **Performance**: Uncontrolled editor architecture with debounced rendering for zero-lag typing.

## 何に向いているか

- アイデア、コマンド、TODO、会議メモ、仕事やゲーム中のリマインダーをすばやく残す。
- よく使う文章をデスクトップに固定し、すぐ読んだりコピーしたりする。
- 作業を大きく中断せず、小さなメモウィンドウで記録する。
- ローカルの下書きをシンプルなカテゴリで整理する。
- Markdown で見出し、リスト、引用、コードブロックを書き、すばやくプレビューする。

## 主な機能

- **ローカルノートライブラリ**：ノート、カテゴリ、設定はローカルに保存され、最初にファイルパスを選ぶ必要はありません。
- **クイックメモ**：トレイまたはグローバルショートカットから小さなメモウィンドウを開けます。マウス付近に表示する設定もできます。
- **デスクトップタイル**：ノートを画面上に固定し、色のカスタマイズや Markdown レンダリングを利用できます。
- **Markdown プレビュー**：日常的な構造化テキスト向けです。完全な Markdown IDE を目指したものではありません。
- **インポートとエクスポート**：単一ファイルの取り込みや、フォルダをカテゴリとして取り込めます。`.mint`、`.md`、`.markdown`、`.txt` に対応します。
- **元ファイル同期の保護**：外部ファイルから取り込んだノートやエクスポートしたファイルは関連ファイルを記録できます。書き戻す前に外部変更を確認し、気づかない上書きを避けます。
- **外観の調整**：テーマ、フォントサイズ、背景画像、ショートカット、トレイに閉じる動作などを設定できます。

## 対応フォーマット

| フォーマット        | 用途                                                    |
| ------------------- | ------------------------------------------------------- |
| `.mint`             | MiniNote の既定ドキュメント形式。UTF-8 プレーンテキスト |
| `.md` / `.markdown` | Markdown ドキュメント                                   |
| `.txt`              | 標準のプレーンテキストファイル                          |

これらのファイルは通常のテキストエディタでも開けます。MiniNote はファイル本文に独自の非公開メタデータを書き込みません。

## インストールと更新

公式ビルドは [GitHub Releases](https://github.com/vivalucas/mininote/releases) で公開されます。1.0.0 以降、公式リリース資産は Windows と macOS のみです。Linux 関連のコードはソースツリーに残していますが、現時点では公式 Linux パッケージは公開しません。

### Windows

- インストーラー：`mininote-<version>-windows-x64-setup.exe`
- ポータブル版：`mininote-<version>-windows-x64.exe`

通常利用にはインストーラーを使ってください。試用や固定フォルダから直接実行したい場合はポータブル版が向いています。

### macOS

Apple Silicon ユーザーは `mininote-<version>-macos-arm64.dmg` をダウンロードし、開いたあと `MiniNote.app` を `Applications` にドラッグしてください。

現在の macOS ビルドは正式に署名されていません。macOS がアプリを開けない、破損している、または未確認の開発元だと表示する場合は、まずファイルが本プロジェクトの Release から入手したものか確認し、ターミナルで次を実行してください。

```bash
xattr -cr /Applications/MiniNote.app
```

その後、アプリをもう一度開いてください。更新するときは MiniNote を終了し、新しい DMG をダウンロードして `Applications` 内の古いアプリを置き換えます。

### Linux

MiniNote 1.0.0 では公式 Linux パッケージを配布しません。Linux 版が必要な場合はソースからビルドしてください。Linux 向けのパッケージ設定はリポジトリに残っています。

## データの保存場所

MiniNote はノートをアップロードせず、クラウド同期も提供しません。ノート、設定、インデックスデータは既定でシステムのアプリデータディレクトリ内の `MiniNote` フォルダに保存されます。`MININOTE_DATA_DIR` を設定すると、そのディレクトリをデータ保存場所として使います。

## ソースからビルド

Node.js、Rust、Tauri に必要なシステム依存関係が必要です。

```bash
npm ci
npm run tauri build
```

開発モード：

```bash
npm run tauri dev
```

## 範囲

- アカウントやクラウド同期はありません。
- リッチテキストエディタではありません。
- 完全な Markdown IDE ではありません。
- 複雑なナレッジベース、バックリンク、共同編集システムではありません。

MiniNote は軽く、速く、ローカルで、手早く使えることを目指しています。

## ライセンス

MIT License
