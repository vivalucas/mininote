import AppKit
import SwiftUI

@main
struct MiniNoteApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) private var appDelegate
    @State private var appState = AppState()

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
                Button(String(localized: "menu.new")) {
                    appState.newTab()
                }
                    .keyboardShortcut("n")
                Button(String(localized: "menu.open")) {
                    appState.openFile()
                }
                    .keyboardShortcut("o")
            }

            CommandGroup(replacing: .saveItem) {
                Button(String(localized: "menu.save")) {
                    if let doc = appState.activeDocument {
                        appState.saveDocument(doc)
                    }
                }
                .keyboardShortcut("s")

                Button(String(localized: "menu.saveAs")) {
                    if let doc = appState.activeDocument {
                        appState.saveAsDocument(doc)
                    }
                }
                .keyboardShortcut("s", modifiers: [.command, .shift])
            }

            CommandGroup(before: .toolbar) {
                Button(String(localized: "menu.toggleRender")) {
                    if let doc = appState.activeDocument {
                        appState.toggleRendering(for: doc)
                    }
                }
                .keyboardShortcut("r")
            }

            // Edit menu additions
            CommandGroup(after: .textEditing) {
                Divider()
                Button(String(localized: "menu.find")) {
                    NotificationCenter.default.post(name: .showFindPanel, object: nil)
                }
                .keyboardShortcut("f")

                Button(String(localized: "menu.findReplace")) {
                    NotificationCenter.default.post(name: .showFindReplacePanel, object: nil)
                }
                .keyboardShortcut("f", modifiers: [.command, .option])
            }

            // View menu — zoom
            CommandGroup(before: .textFormatting) {
                Button(String(localized: "menu.zoomIn")) {
                    appState.zoomIn()
                }
                    .keyboardShortcut("=", modifiers: [.command])
                Button(String(localized: "menu.zoomOut")) {
                    appState.zoomOut()
                }
                    .keyboardShortcut("-", modifiers: [.command])
                Button(String(localized: "menu.zoomReset")) {
                    appState.resetZoom()
                }
                    .keyboardShortcut("0", modifiers: [.command])
            }

            // Window menu — close tab
            CommandGroup(after: .windowSize) {
                Button(String(localized: "menu.closeTab")) {
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
        }

        Settings {
            SettingsView()
        }
    }
}
