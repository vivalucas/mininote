import Foundation

enum Language: String, CaseIterable, Identifiable {
    case chinese = "zh"
    case english = "en"
    case japanese = "ja"

    var id: String { rawValue }

    var displayName: String {
        switch self {
        case .chinese: "中文"
        case .english: "English"
        case .japanese: "日本語"
        }
    }

    static var systemDefault: Language {
        let code = Locale.current.language.languageCode?.identifier ?? "en"
        switch code {
        case "zh":
            return .chinese
        case "ja":
            return .japanese
        default:
            return .english
        }
    }
}

enum LocalizationService {
    static func text(_ key: String, language rawValue: String? = nil) -> String {
        let language = rawValue ?? UserDefaults.standard.string(forKey: "appLanguage")
        let resolved = language ?? Language.systemDefault.rawValue
        return table[key]?[resolved] ?? table[key]?[Language.english.rawValue] ?? key
    }

    static func formatted(_ key: String, _ arguments: CVarArg..., language: String? = nil) -> String {
        String(format: text(key, language: language), arguments: arguments)
    }

    private static let table: [String: [String: String]] = [
        "menu.new": [
            "zh": "新建",
            "en": "New",
            "ja": "新規"
        ],
        "menu.open": [
            "zh": "打开...",
            "en": "Open...",
            "ja": "開く..."
        ],
        "menu.save": [
            "zh": "保存",
            "en": "Save",
            "ja": "保存"
        ],
        "menu.saveAs": [
            "zh": "另存为...",
            "en": "Save As...",
            "ja": "名前を付けて保存..."
        ],
        "menu.closeTab": [
            "zh": "关闭标签页",
            "en": "Close Tab",
            "ja": "タブを閉じる"
        ],
        "menu.find": [
            "zh": "查找...",
            "en": "Find...",
            "ja": "検索..."
        ],
        "menu.findReplace": [
            "zh": "查找替换...",
            "en": "Find & Replace...",
            "ja": "検索と置換..."
        ],
        "menu.toggleRender": [
            "zh": "Markdown 渲染切换",
            "en": "Toggle Markdown Rendering",
            "ja": "Markdown レンダリング切替"
        ],
        "menu.zoomIn": [
            "zh": "放大",
            "en": "Zoom In",
            "ja": "拡大"
        ],
        "menu.zoomOut": [
            "zh": "缩小",
            "en": "Zoom Out",
            "ja": "縮小"
        ],
        "menu.zoomReset": [
            "zh": "实际大小",
            "en": "Actual Size",
            "ja": "実際のサイズ"
        ],
        "menu.help": [
            "zh": "帮助与更新",
            "en": "Help & Updates",
            "ja": "ヘルプと更新"
        ],
        "menu.checkUpdates": [
            "zh": "检查更新...",
            "en": "Check for Updates...",
            "ja": "アップデートを確認..."
        ],
        "menu.viewOnGitHub": [
            "zh": "在 GitHub 上查看",
            "en": "View on GitHub",
            "ja": "GitHub で開く"
        ],
        "menu.contact": [
            "zh": "联系方式",
            "en": "Contact",
            "ja": "お問い合わせ"
        ],

        "settings.general": [
            "zh": "通用",
            "en": "General",
            "ja": "一般"
        ],
        "settings.appearance": [
            "zh": "外观",
            "en": "Appearance",
            "ja": "外観"
        ],
        "settings.behavior": [
            "zh": "行为",
            "en": "Behavior",
            "ja": "動作"
        ],
        "settings.update": [
            "zh": "更新",
            "en": "Update",
            "ja": "アップデート"
        ],
        "settings.language": [
            "zh": "语言",
            "en": "Language",
            "ja": "言語"
        ],
        "settings.startup": [
            "zh": "启动行为",
            "en": "Startup Behavior",
            "ja": "起動動作"
        ],
        "settings.continueLast": [
            "zh": "继续上一个会话",
            "en": "Continue Last Session",
            "ja": "前回のセッションを続ける"
        ],
        "settings.newSession": [
            "zh": "启动新会话",
            "en": "Start New Session",
            "ja": "新しいセッションを開始"
        ],
        "settings.wordWrap": [
            "zh": "自动换行",
            "en": "Word Wrap",
            "ja": "自動折り返し"
        ],
        "settings.lineNumbers": [
            "zh": "行号显示",
            "en": "Line Numbers",
            "ja": "行番号表示"
        ],
        "settings.theme": [
            "zh": "主题",
            "en": "Theme",
            "ja": "テーマ"
        ],
        "settings.light": [
            "zh": "浅色",
            "en": "Light",
            "ja": "ライト"
        ],
        "settings.dark": [
            "zh": "深色",
            "en": "Dark",
            "ja": "ダーク"
        ],
        "settings.system": [
            "zh": "跟随系统",
            "en": "Follow System",
            "ja": "システムに従う"
        ],
        "settings.fontSize": [
            "zh": "字号",
            "en": "Font Size",
            "ja": "フォントサイズ"
        ],
        "settings.font": [
            "zh": "字体",
            "en": "Font",
            "ja": "フォント"
        ],
        "settings.chooseFont": [
            "zh": "选择字体...",
            "en": "Choose Font...",
            "ja": "フォントを選択..."
        ],
        "settings.defaultRender": [
            "zh": "默认渲染状态",
            "en": "Default Render State",
            "ja": "デフォルトレンダリング"
        ],
        "settings.renderMint": [
            "zh": ".mint 默认渲染",
            "en": "Render .mint by default",
            "ja": ".mint をデフォルトでレンダリング"
        ],
        "settings.renderTxt": [
            "zh": ".txt 默认渲染",
            "en": "Render .txt by default",
            "ja": ".txt をデフォルトでレンダリング"
        ],
        "settings.renderMd": [
            "zh": ".md 默认渲染",
            "en": "Render .md by default",
            "ja": ".md をデフォルトでレンダリング"
        ],
        "settings.resetDefaults": [
            "zh": "恢复默认设置",
            "en": "Reset to Defaults",
            "ja": "デフォルトに戻す"
        ],
        "settings.checkUpdate": [
            "zh": "检查更新",
            "en": "Check for Updates",
            "ja": "アップデートを確認"
        ],

        "update.hasUpdate": [
            "zh": "有新版本 %@",
            "en": "New version available: %@",
            "ja": "新しいバージョンがあります: %@"
        ],
        "update.availableTitle": [
            "zh": "发现新版本",
            "en": "New Version Available",
            "ja": "新しいバージョンがあります"
        ],
        "update.availableMessage": [
            "zh": "MiniNote %@ 可用。当前版本为 %@。",
            "en": "MiniNote %@ is available. You have %@.",
            "ja": "MiniNote %@ が利用可能です。現在のバージョンは %@ です。"
        ],
        "update.latest": [
            "zh": "已是最新版本",
            "en": "You are up to date",
            "ja": "最新バージョンです"
        ],
        "update.latestMessage": [
            "zh": "MiniNote %@ 是最新版本。",
            "en": "MiniNote %@ is the latest version.",
            "ja": "MiniNote %@ は最新バージョンです。"
        ],
        "update.failed": [
            "zh": "无法检查更新",
            "en": "Could not check for updates",
            "ja": "アップデートを確認できませんでした"
        ],
        "update.failedMessage": [
            "zh": "请检查网络连接后重试。",
            "en": "Please check your internet connection and try again.",
            "ja": "ネットワーク接続を確認して、もう一度お試しください。"
        ],
        "update.download": [
            "zh": "下载",
            "en": "Download",
            "ja": "ダウンロード"
        ],
        "update.later": [
            "zh": "稍后",
            "en": "Later",
            "ja": "後で"
        ],
        "settings.defaultsRestored": [
            "zh": "已恢复默认设置",
            "en": "Defaults restored",
            "ja": "デフォルトに戻しました"
        ],

        "alert.saveChanges": [
            "zh": "是否保存更改？",
            "en": "Save changes?",
            "ja": "変更を保存しますか？"
        ],
        "alert.save": [
            "zh": "保存",
            "en": "Save",
            "ja": "保存"
        ],
        "alert.dontSave": [
            "zh": "不保存",
            "en": "Don't Save",
            "ja": "保存しない"
        ],
        "alert.cancel": [
            "zh": "取消",
            "en": "Cancel",
            "ja": "キャンセル"
        ],
        "alert.hasChanges": [
            "zh": "「%@」有未保存的更改。",
            "en": "\"%@\" has unsaved changes.",
            "ja": "「%@」に未保存の変更があります。"
        ],

        "welcome.title": [
            "zh": "欢迎使用 MiniNote",
            "en": "Welcome to MiniNote",
            "ja": "MiniNote へようこそ"
        ],
        "welcome.subtitle": [
            "zh": "一个轻量级纯文本编辑器，对标 Windows 11 新版记事本。",
            "en": "A lightweight plain text editor, inspired by Windows 11 Notepad.",
            "ja": "Windows 11 のメモ帳にインスパイアされた軽量テキストエディタ。"
        ],
        "welcome.quickSetup": [
            "zh": "快速设置",
            "en": "Quick Setup",
            "ja": "クイック設定"
        ],
        "welcome.setDefault": [
            "zh": "将 .mint 文件默认用 MiniNote 打开",
            "en": "Set MiniNote as default for .mint files",
            "ja": ".mint ファイルのデフォルトを MiniNote に設定"
        ],
        "welcome.mdRender": [
            "zh": "打开 .md 文件时默认渲染 Markdown",
            "en": "Render Markdown by default for .md files",
            "ja": ".md ファイルをデフォルトで Markdown レンダリング"
        ],
        "welcome.settingsHint": [
            "zh": "随时可在偏好设置（Cmd+,）中修改这些选项。",
            "en": "You can change these anytime in Settings (Cmd+,).",
            "ja": "これらの設定は環境設定（Cmd+,）でいつでも変更できます。"
        ],
        "welcome.start": [
            "zh": "开始使用",
            "en": "Get Started",
            "ja": "はじめる"
        ],

        "status.position": [
            "zh": "行 %d, 列 %d",
            "en": "Ln %d, Col %d",
            "ja": "%d 行, %d 列"
        ],
        "status.chars": [
            "zh": "%d 字符",
            "en": "%d chars",
            "ja": "%d 文字"
        ],
        "status.plain": [
            "zh": "纯文本",
            "en": "Text",
            "ja": "テキスト"
        ],
        "status.rendered": [
            "zh": "渲染",
            "en": "Rendered",
            "ja": "レンダリング"
        ],

        "common.untitled": [
            "zh": "未标题",
            "en": "Untitled",
            "ja": "無題"
        ],
        "document.mint": [
            "zh": "MiniNote 文档",
            "en": "MiniNote Document",
            "ja": "MiniNote 文書"
        ],
        "document.txt": [
            "zh": "纯文本文档",
            "en": "Plain Text Document",
            "ja": "プレーンテキスト文書"
        ],
        "document.md": [
            "zh": "Markdown 文档",
            "en": "Markdown Document",
            "ja": "Markdown 文書"
        ],
        "file.error.cannotRead": [
            "zh": "无法读取文件",
            "en": "Cannot read file",
            "ja": "ファイルを読み込めません"
        ],
        "file.error.cannotWrite": [
            "zh": "无法写入文件",
            "en": "Cannot write file",
            "ja": "ファイルを書き込めません"
        ],
        "file.error.unsupportedType": [
            "zh": "不支持的文件格式",
            "en": "Unsupported file format",
            "ja": "サポートされていないファイル形式です"
        ],
        "contact.title": [
            "zh": "MiniNote 联系信息",
            "en": "MiniNote Contact Info",
            "ja": "MiniNote 連絡先"
        ],
        "contact.body": [
            "zh": "作者：Lucas\n\n功能建议与问题反馈：\nhttps://github.com/vivalucas/mininote/issues\n\n邮箱：lucas6.zju@vip.163.com",
            "en": "Author: Lucas\n\nBug reports & feature requests:\nhttps://github.com/vivalucas/mininote/issues\n\nEmail: lucas6.zju@vip.163.com",
            "ja": "作者：Lucas\n\nバグ報告・機能リクエスト：\nhttps://github.com/vivalucas/mininote/issues\n\nメール：lucas6.zju@vip.163.com"
        ]
    ]
}
