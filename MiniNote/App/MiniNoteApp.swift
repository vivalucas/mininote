import AppKit
import SwiftUI

@main
struct MiniNoteApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) private var appDelegate
    @State private var appState = AppState()
    @AppStorage("appLanguage") private var appLanguage: String = Language.systemDefault.rawValue
    private let updateService = UpdateService()

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environment(appState)
                .onAppear {
                    appDelegate.appState = appState
                }
        }
        .windowResizability(.contentSize)
        .defaultSize(width: 800, height: 600)
        .commands {
            // File menu
            CommandGroup(replacing: .newItem) {
                Button(LocalizationService.text("menu.new", language: appLanguage)) {
                    appState.newTab()
                }
                    .keyboardShortcut("n")
                Button(LocalizationService.text("menu.open", language: appLanguage)) {
                    appState.openFile()
                }
                    .keyboardShortcut("o")
            }

            CommandGroup(replacing: .saveItem) {
                Button(LocalizationService.text("menu.save", language: appLanguage)) {
                    if let doc = appState.activeDocument {
                        appState.saveDocument(doc)
                    }
                }
                .keyboardShortcut("s")

                Button(LocalizationService.text("menu.saveAs", language: appLanguage)) {
                    if let doc = appState.activeDocument {
                        appState.saveAsDocument(doc)
                    }
                }
                .keyboardShortcut("s", modifiers: [.command, .shift])
            }

            CommandGroup(before: .toolbar) {
                Button(LocalizationService.text("menu.toggleRender", language: appLanguage)) {
                    if let doc = appState.activeDocument {
                        appState.toggleRendering(for: doc)
                    }
                }
                .keyboardShortcut("r")
            }

            // Edit menu additions
            CommandGroup(after: .textEditing) {
                Divider()
                Button(LocalizationService.text("menu.find", language: appLanguage)) {
                    NotificationCenter.default.post(name: .showFindPanel, object: nil)
                }
                .keyboardShortcut("f")

                Button(LocalizationService.text("menu.findReplace", language: appLanguage)) {
                    NotificationCenter.default.post(name: .showFindReplacePanel, object: nil)
                }
                .keyboardShortcut("f", modifiers: [.command, .option])
            }

            // View menu — zoom
            CommandGroup(before: .textFormatting) {
                Button(LocalizationService.text("menu.zoomIn", language: appLanguage)) {
                    appState.zoomIn()
                }
                    .keyboardShortcut("=", modifiers: [.command])
                Button(LocalizationService.text("menu.zoomOut", language: appLanguage)) {
                    appState.zoomOut()
                }
                    .keyboardShortcut("-", modifiers: [.command])
                Button(LocalizationService.text("menu.zoomReset", language: appLanguage)) {
                    appState.resetZoom()
                }
                    .keyboardShortcut("0", modifiers: [.command])
            }

            // Window menu — close tab
            CommandGroup(after: .windowSize) {
                Button(LocalizationService.text("menu.closeTab", language: appLanguage)) {
                    if let doc = appState.activeDocument {
                        // Post to ContentView via a notification or direct action
                        NotificationCenter.default.post(
                            name: .closeCurrentTab,
                            object: doc
                        )
                    }
                }
                .keyboardShortcut("w")
            }

            CommandMenu(LocalizationService.text("menu.help", language: appLanguage)) {
                Button(LocalizationService.text("menu.checkUpdates", language: appLanguage)) {
                    checkForUpdates()
                }
                Button(LocalizationService.text("menu.viewOnGitHub", language: appLanguage)) {
                    NSWorkspace.shared.open(updateService.repositoryURL)
                }
                Divider()
                Button(LocalizationService.text("menu.contact", language: appLanguage)) {
                    showContact()
                }
            }
        }

        Settings {
            SettingsView()
        }
    }

    private func checkForUpdates() {
        Task {
            do {
                guard let result = try await updateService.checkForUpdate() else {
                    await MainActor.run {
                        showAlert(
                            title: L("update.failed"),
                            message: L("update.failedMessage")
                        )
                    }
                    return
                }

                await MainActor.run {
                    if result.hasUpdate, let release = result.release {
                        let latestVersion = updateService.normalizedVersion(release.tagName)
                        let currentVersion = updateService.currentAppVersion()
                        showUpdateAvailableAlert(
                            latestVersion: latestVersion,
                            currentVersion: currentVersion,
                            releaseURL: release.htmlURL
                        )
                    } else {
                        showAlert(
                            title: L("update.latest"),
                            message: LocalizationService.formatted(
                                "update.latestMessage",
                                updateService.currentAppVersion(),
                                language: appLanguage
                            )
                        )
                    }
                }
            } catch {
                await MainActor.run {
                    showAlert(
                        title: L("update.failed"),
                        message: L("update.failedMessage")
                    )
                }
            }
        }
    }

    private func showUpdateAvailableAlert(
        latestVersion: String,
        currentVersion: String,
        releaseURL: URL
    ) {
        let alert = NSAlert()
        alert.messageText = L("update.availableTitle")
        alert.informativeText = LocalizationService.formatted(
            "update.availableMessage",
            latestVersion,
            currentVersion,
            language: appLanguage
        )
        alert.addButton(withTitle: L("update.download"))
        alert.addButton(withTitle: L("update.later"))
        NSApp.activate(ignoringOtherApps: true)
        if alert.runModal() == .alertFirstButtonReturn {
            NSWorkspace.shared.open(releaseURL)
        }
    }

    private func showContact() {
        showAlert(title: L("contact.title"), message: L("contact.body"))
    }

    private func showAlert(title: String, message: String) {
        let alert = NSAlert()
        alert.messageText = title
        alert.informativeText = message
        alert.addButton(withTitle: "OK")
        NSApp.activate(ignoringOtherApps: true)
        _ = alert.runModal()
    }

    private func L(_ key: String) -> String {
        LocalizationService.text(key, language: appLanguage)
    }
}
