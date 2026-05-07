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
    @AppStorage("appLanguage") private var appLanguage: String = Language.systemDefault.rawValue

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
            Picker(L("settings.language"), selection: $appLanguage) {
                ForEach(Language.allCases) { language in
                    Text(language.displayName).tag(language.rawValue)
                }
            }
            .pickerStyle(.menu)

            Picker(L("settings.startup"), selection: $startupBehavior) {
                Text(L("settings.continueLast")).tag("continue")
                Text(L("settings.newSession")).tag("new")
            }
            .pickerStyle(.radioGroup)

            Toggle(L("settings.wordWrap"), isOn: $wordWrap)
            Toggle(L("settings.lineNumbers"), isOn: $showLineNumbers)

            Spacer()
        }
        .padding()
        .tabItem { Label(L("settings.general"), systemImage: "gearshape") }
    }

    // MARK: - Appearance

    private var appearanceTab: some View {
        VStack(alignment: .leading, spacing: 16) {
            Picker(L("settings.theme"), selection: $theme) {
                Text(L("settings.light")).tag("light")
                Text(L("settings.dark")).tag("dark")
                Text(L("settings.system")).tag("system")
            }
            .pickerStyle(.radioGroup)

            HStack {
                Text(L("settings.fontSize"))
                Slider(value: $fontSize, in: 10...30, step: 1)
                Text("\(Int(fontSize)) pt")
                    .frame(width: 40, alignment: .trailing)
            }

            HStack {
                Text(L("settings.font"))
                Button(L("settings.chooseFont")) {
                    NSFontManager.shared.orderFrontFontPanel(nil)
                }
            }

            Spacer()
        }
        .padding()
        .tabItem { Label(L("settings.appearance"), systemImage: "paintpalette") }
    }

    // MARK: - Behavior

    private var behaviorTab: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text(L("settings.defaultRender"))
                .font(.headline)

            Toggle(L("settings.renderMint"), isOn: $renderMint)
            Toggle(L("settings.renderTxt"), isOn: $renderTxt)
            Toggle(L("settings.renderMd"), isOn: $renderMd)

            Spacer()

            HStack {
                Spacer()
                Button(L("settings.resetDefaults")) {
                    resetDefaults()
                }
            }
        }
        .padding()
        .tabItem { Label(L("settings.behavior"), systemImage: "text.word.spacing") }
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
                    Text(L("settings.checkUpdate"))
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
        .tabItem { Label(L("settings.update"), systemImage: "arrow.down.circle") }
    }

    // MARK: - Actions

    private func checkForUpdate() {
        isChecking = true
        updateMessage = ""
        Task {
            defer { isChecking = false }
            if let result = try? await updateService.checkForUpdate() {
                if result.hasUpdate, let release = result.release {
                    updateMessage = LocalizationService.formatted(
                        "update.hasUpdate",
                        release.tagName,
                        language: appLanguage
                    )
                    NSWorkspace.shared.open(release.htmlURL)
                } else {
                    updateMessage = L("update.latest")
                }
            } else {
                updateMessage = L("update.failed")
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
        updateMessage = L("settings.defaultsRestored")
    }

    private func L(_ key: String) -> String {
        LocalizationService.text(key, language: appLanguage)
    }
}
