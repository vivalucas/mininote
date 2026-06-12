[简体中文](readme.md) | [繁體中文](README_zh-TW.md) | [English](README_en-US.md) | [日本語](README_ja.md) | **한국어** | [Deutsch](README_de.md) | [Français](README_fr.md) | [Español](README_es.md) | [Português](README_pt-BR.md) | [Italiano](README_it.md) | [Русский](README_ru.md)

# MiniNote

[![GitHub Release](https://img.shields.io/github/v/release/vivalucas/mininote?style=flat-square)](https://github.com/vivalucas/mininote/releases) [![License](https://img.shields.io/github/license/vivalucas/mininote?style=flat-square)](LICENSE) [![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey?style=flat-square)]()

MiniNote는 로컬 우선 데스크톱 메모 앱입니다. 열면 바로 쓰고, 모든 내용을 내 컴퓨터에 저장하며, 필요할 때 메모를 데스크톱에 고정할 수 있습니다. 계정, 클라우드 동기화, 무거운 지식 관리 흐름은 필요하지 않습니다.

## Technology Stack

- **Framework**: Tauri 2 (Rust)
- **Frontend**: React 19 + TypeScript
- **Styling**: TailwindCSS
- **Performance**: Uncontrolled editor architecture with debounced rendering for zero-lag typing.

## 어떤 용도에 적합한가

- 아이디어, 명령어, 할 일, 회의 메모, 업무나 게임 중 참고할 내용을 빠르게 기록합니다.
- 자주 쓰는 텍스트를 데스크톱에 고정해 바로 읽고 복사합니다.
- 현재 작업을 크게 방해하지 않고 작은 메모 창에서 기록합니다.
- 로컬 초안을 간단한 카테고리로 정리합니다.
- Markdown으로 제목, 목록, 인용, 코드 블록을 작성하고 빠르게 미리 봅니다.

## 주요 기능

- **로컬 메모 라이브러리**: 메모, 카테고리, 설정은 로컬에 저장되며 먼저 파일 경로를 고를 필요가 없습니다.
- **빠른 메모**: 트레이 또는 전역 단축키로 작은 메모 창을 열 수 있으며, 마우스 근처에 표시되도록 설정할 수 있습니다.
- **데스크톱 타일**: 메모를 화면에 고정하고 색상과 Markdown 렌더링을 설정할 수 있습니다.
- **Markdown 미리보기**: 일상적인 구조화 텍스트에 적합하며, 완전한 Markdown IDE를 목표로 하지 않습니다.
- **가져오기와 내보내기**: 단일 파일을 가져오거나 폴더를 카테고리로 가져올 수 있으며 `.mint`, `.md`, `.markdown`, `.txt`를 지원합니다.
- **원본 파일 동기화 보호**: 외부 파일에서 가져온 메모와 내보낸 파일은 연결된 파일을 보관할 수 있습니다. 다시 쓰기 전에 외부 변경을 확인해 조용히 덮어쓰는 일을 피합니다.
- **외관 설정**: 테마, 글꼴 크기, 배경 이미지, 단축키, 트레이로 닫기 동작 등을 조정할 수 있습니다.

## 지원 형식

| 형식                | 용도                                       |
| ------------------- | ------------------------------------------ |
| `.mint`             | MiniNote 기본 문서 형식, UTF-8 일반 텍스트 |
| `.md` / `.markdown` | Markdown 문서                              |
| `.txt`              | 표준 일반 텍스트 파일                      |

지원되는 파일은 모두 일반 텍스트 편집기에서도 열 수 있습니다. MiniNote는 파일 본문에 비공개 메타데이터를 쓰지 않습니다.

## 설치와 업데이트

공식 빌드는 [GitHub Releases](https://github.com/vivalucas/mininote/releases)에 게시됩니다. 1.0.0부터 공식 릴리스 자산은 Windows와 macOS만 제공합니다. Linux 관련 코드는 소스 트리에 남아 있지만, 현재 공식 Linux 패키지는 게시하지 않습니다.

### Windows

- 설치 프로그램: `mininote-<version>-windows-x64-setup.exe`
- 포터블 빌드: `mininote-<version>-windows-x64.exe`

일상적으로 사용하려면 설치 프로그램을 사용하세요. 임시로 사용하거나 고정 폴더에서 바로 실행하려면 포터블 빌드가 적합합니다.

### macOS

Apple Silicon 사용자는 `mininote-<version>-macos-arm64.dmg`를 다운로드하고 연 뒤 `MiniNote.app`을 `Applications`로 드래그하세요.

현재 macOS 빌드는 공식 서명되지 않았습니다. macOS에서 앱을 열 수 없거나, 손상되었거나, 확인되지 않은 개발자 앱이라고 표시하면 먼저 파일이 이 프로젝트의 Release 페이지에서 받은 것인지 확인한 다음 터미널에서 실행하세요.

```bash
xattr -cr /Applications/MiniNote.app
```

그런 다음 앱을 다시 여세요. 업데이트할 때는 MiniNote를 종료하고 새 DMG를 다운로드한 뒤 `Applications`의 기존 앱을 교체합니다.

### Linux

MiniNote 1.0.0은 공식 Linux 패키지를 제공하지 않습니다. Linux 빌드가 필요하면 소스에서 빌드하세요. Linux 패키징 설정은 저장소에 유지됩니다.

## 데이터 저장 위치

MiniNote는 메모를 업로드하지 않으며 클라우드 동기화도 제공하지 않습니다. 메모, 설정, 인덱스 데이터는 기본적으로 시스템 애플리케이션 데이터 디렉터리의 `MiniNote` 폴더에 저장됩니다. `MININOTE_DATA_DIR`을 설정하면 MiniNote는 해당 디렉터리를 데이터 디렉터리로 사용합니다.

## 소스에서 빌드

Node.js, Rust, Tauri에 필요한 시스템 의존성이 필요합니다.

```bash
npm ci
npm run tauri build
```

개발 모드:

```bash
npm run tauri dev
```

## 범위

- 계정이나 클라우드 동기화는 없습니다.
- 리치 텍스트 편집기가 아닙니다.
- 완전한 Markdown IDE가 아닙니다.
- 복잡한 지식베이스, 백링크, 협업 시스템이 아닙니다.

MiniNote의 목표는 가볍고, 빠르고, 로컬이며, 손쉽게 쓰는 것입니다.

## 라이선스

MIT License
