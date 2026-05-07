import SwiftUI

struct SettingsView: View {
    // General
    @AppStorage("fontSize") private var fontSize: Double = 14
    @AppStorage("wordWrap") private var wordWrap: Bool = true
    @AppStorage("showLineNumbers") private var showLineNumbers: Bool = false

    // Appearance
    @AppStorage("theme") private var theme: String = "system"

    // Behavior — per-type default render
    @AppStorage("renderMint") private var renderMint: Bool = false
    @AppStorage("renderTxt") private var renderTxt: Bool = false
    @AppStorage("renderMd") private var renderMd: Bool = false

    // Startup
    @AppStorage("startupBehavior") private var startupBehavior: String = "continue"

    // Update check
    @State private var updateMessage: String = ""
    @State private var isChecking = false

    private let updateService = UpdateService()

    var body: some View {
        TabView {
            generalTab
            appearanceTab
            behaviorTab
            updateTab
        }
        .frame(width: 420, height: 300)
    }

    // MARK: - General

    private var generalTab: some View {
        VStack(alignment: .leading, spacing: 16) {
            Picker("启动行为", selection: $startupBehavior) {
                Text("继续上一个会话").tag("continue")
                Text("启动新会话").tag("new")
            }
            .pickerStyle(.radioGroup)

            Toggle("自动换行", isOn: $wordWrap)
            Toggle("行号显示", isOn: $showLineNumbers)

            Spacer()
        }
        .padding()
        .tabItem { Label("通用", systemImage: "gearshape") }
    }

    // MARK: - Appearance

    private var appearanceTab: some View {
        VStack(alignment: .leading, spacing: 16) {
            Picker("主题", selection: $theme) {
                Text("浅色").tag("light")
                Text("深色").tag("dark")
                Text("跟随系统").tag("system")
            }
            .pickerStyle(.radioGroup)

            HStack {
                Text("字号")
                Slider(value: $fontSize, in: 10...30, step: 1)
                Text("\(Int(fontSize)) pt")
                    .frame(width: 40, alignment: .trailing)
            }

            HStack {
                Text("字体")
                Button("选择字体...") {
                    NSFontManager.shared.orderFrontFontPanel(nil)
                }
            }

            Spacer()
        }
        .padding()
        .tabItem { Label("外观", systemImage: "paintpalette") }
    }

    // MARK: - Behavior

    private var behaviorTab: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("默认渲染状态")
                .font(.headline)

            Toggle(".mint 默认渲染", isOn: $renderMint)
            Toggle(".txt 默认渲染", isOn: $renderTxt)
            Toggle(".md 默认渲染", isOn: $renderMd)

            Spacer()

            HStack {
                Spacer()
                Button("恢复默认设置") {
                    resetDefaults()
                }
            }
        }
        .padding()
        .tabItem { Label("行为", systemImage: "text.word.spacing") }
    }

    // MARK: - Update

    private var updateTab: some View {
        VStack(alignment: .leading, spacing: 16) {
            Button(action: checkForUpdate) {
                HStack {
                    if isChecking {
                        ProgressView()
                            .scaleEffect(0.8)
                    }
                    Text("检查更新")
                }
            }
            .disabled(isChecking)

            if !updateMessage.isEmpty {
                Text(updateMessage)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()
        }
        .padding()
        .tabItem { Label("更新", systemImage: "arrow.down.circle") }
    }

    // MARK: - Actions

    private func checkForUpdate() {
        isChecking = true
        updateMessage = ""
        Task {
            defer { isChecking = false }
            if let result = try? await updateService.checkForUpdate() {
                if result.hasUpdate, let release = result.release {
                    updateMessage = "有新版本 \(release.tagName)"
                    NSWorkspace.shared.open(release.htmlURL)
                } else {
                    updateMessage = "已是最新版本"
                }
            } else {
                updateMessage = "检查失败，请稍后重试"
            }
        }
    }

    private func resetDefaults() {
        fontSize = 14
        wordWrap = true
        showLineNumbers = false
        theme = "system"
        renderMint = false
        renderTxt = false
        renderMd = false
        startupBehavior = "continue"
        updateMessage = "已恢复默认设置"
    }
}
