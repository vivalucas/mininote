import SwiftUI

struct SettingsView: View {
    // General
    @AppStorage("fontSize") private var fontSize: Double = 16
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

    @State private var updateMessage: String = ""
    @State private var isChecking = false

    private let updateService = UpdateService()

    var body: some View {
        TabView {
            generalTab
            appearanceTab
            behaviorTab
            aboutTab
        }
        .frame(width: 480, height: 320)
    }

    // MARK: - General

    private var generalTab: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text(String(localized: "settings.languageHint"))
                .font(.caption)
                .foregroundColor(.secondary)

            Picker(String(localized: "settings.startup"), selection: $startupBehavior) {
                Text(String(localized: "settings.continueLast")).tag("continue")
                Text(String(localized: "settings.newSession")).tag("new")
            }
            .pickerStyle(.radioGroup)

            Toggle(String(localized: "settings.wordWrap"), isOn: $wordWrap)
            Toggle(String(localized: "settings.lineNumbers"), isOn: $showLineNumbers)

            Spacer()
        }
        .padding()
        .tabItem { Label(String(localized: "settings.general"), systemImage: "gearshape") }
    }

    // MARK: - Appearance

    private var appearanceTab: some View {
        VStack(alignment: .leading, spacing: 16) {
            Picker(String(localized: "settings.theme"), selection: $theme) {
                Text(String(localized: "settings.light")).tag("light")
                Text(String(localized: "settings.dark")).tag("dark")
                Text(String(localized: "settings.system")).tag("system")
            }
            .pickerStyle(.radioGroup)

            HStack(alignment: .firstTextBaseline) {
                Text(String(localized: "settings.fontSize"))
                Slider(value: $fontSize, in: 10...30, step: 1)
                Text("\(Int(fontSize)) pt")
                    .frame(width: 40, alignment: .trailing)
            }

            HStack(alignment: .firstTextBaseline) {
                Text(String(localized: "settings.font"))
                Text("System")
                    .foregroundColor(.secondary)
                Text("SF Pro / PingFang / Hiragino")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()
        }
        .padding()
        .tabItem { Label(String(localized: "settings.appearance"), systemImage: "paintpalette") }
    }

    // MARK: - Behavior

    private var behaviorTab: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text(String(localized: "settings.defaultRender"))
                .font(.headline)

            Toggle(String(localized: "settings.renderMint"), isOn: $renderMint)
            Toggle(String(localized: "settings.renderTxt"), isOn: $renderTxt)
            Toggle(String(localized: "settings.renderMd"), isOn: $renderMd)

            Spacer()

            HStack {
                Spacer()
                Button(String(localized: "settings.resetDefaults")) {
                    resetDefaults()
                }
            }
        }
        .padding()
        .tabItem { Label(String(localized: "settings.behavior"), systemImage: "text.word.spacing") }
    }

    // MARK: - About

    private var aboutTab: some View {
        VStack(alignment: .leading, spacing: 14) {
            HStack(spacing: 12) {
                Image(nsImage: NSApp.applicationIconImage)
                    .resizable()
                    .frame(width: 48, height: 48)
                    .cornerRadius(10)
                VStack(alignment: .leading, spacing: 3) {
                    Text("MiniNote")
                        .font(.headline)
                    Text(String(format: NSLocalizedString("settings.version", comment: ""), updateService.currentAppVersion()))
                        .foregroundColor(.secondary)
                }
            }

            Divider()

            HStack(spacing: 8) {
                Button(action: checkForUpdate) {
                    if isChecking {
                        ProgressView()
                            .scaleEffect(0.75)
                    } else {
                        Text(String(localized: "settings.checkUpdate"))
                    }
                }
                .disabled(isChecking)

                Button(String(localized: "menu.viewOnGitHub")) {
                    NSWorkspace.shared.open(updateService.repositoryURL)
                }

                Button(String(localized: "menu.contact")) {
                    NSWorkspace.shared.open(updateService.issuesURL)
                }
            }

            if !updateMessage.isEmpty {
                Text(updateMessage)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }

            Spacer()
        }
        .padding()
        .tabItem { Label(String(localized: "settings.about"), systemImage: "info.circle") }
    }

    // MARK: - Actions

    private func resetDefaults() {
        fontSize = 16
        wordWrap = true
        showLineNumbers = false
        theme = "system"
        renderMint = false
        renderTxt = false
        renderMd = false
        startupBehavior = "continue"
    }

    private func checkForUpdate() {
        isChecking = true
        updateMessage = ""
        Task {
            defer { isChecking = false }
            if let result = try? await updateService.checkForUpdate() {
                if result.hasUpdate, let release = result.release {
                    updateMessage = String(format: NSLocalizedString("update.hasUpdate", comment: ""), release.tagName)
                    NSWorkspace.shared.open(release.htmlURL)
                } else {
                    updateMessage = String(localized: "update.latest")
                }
            } else {
                updateMessage = String(localized: "update.failed")
            }
        }
    }
}
