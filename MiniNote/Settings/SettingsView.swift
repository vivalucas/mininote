import SwiftUI

struct SettingsView: View {
    @Environment(\.dismiss) private var dismiss

    // General
    @Binding var fontSize: Double
    @Binding var wordWrap: Bool
    @Binding var showLineNumbers: Bool

    // Appearance
    @AppStorage("theme") private var theme: String = "system"
    @AppStorage("fontName") private var fontName: String = "system"

    // Behavior — per-type default render
    @AppStorage("renderMint") private var renderMint: Bool = false
    @AppStorage("renderTxt") private var renderTxt: Bool = false
    @AppStorage("renderMd") private var renderMd: Bool = false

    // Startup
    @AppStorage("startupBehavior") private var startupBehavior: String = "continue"

    // Update check
    @State private var updateMessage: String = ""
    @State private var isChecking = false

    @Binding var documents: [Document]

    private let updateService = UpdateService()

    var body: some View {
        TabView {
            // General
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

            // Appearance
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
                Spacer()
            }
            .padding()
            .tabItem { Label("外观", systemImage: "paintpalette") }

            // Behavior
            VStack(alignment: .leading, spacing: 16) {
                Toggle(".mint 默认渲染", isOn: $renderMint)
                Toggle(".txt 默认渲染", isOn: $renderTxt)
                Toggle(".md 默认渲染", isOn: $renderMd)
                Spacer()
            }
            .padding()
            .tabItem { Label("行为", systemImage: "text.word.spacing") }

            // Update
            VStack(alignment: .leading, spacing: 16) {
                Button(action: checkForUpdate) {
                    if isChecking {
                        ProgressView()
                            .scaleEffect(0.8)
                    }
                    Text("检查更新")
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
        .frame(width: 420, height: 280)
    }

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
        fontName = "system"
        renderMint = false
        renderTxt = false
        renderMd = false
        startupBehavior = "continue"
    }
}
