import SwiftUI

@main
struct MiniNoteApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) private var appDelegate
    @State private var appState = AppState()

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environment(appState)
        }
        .windowResizability(.contentSize)
        .defaultSize(width: 800, height: 600)
        .commands {
            // File menu
            CommandGroup(replacing: .newItem) {
                Button("新建") { appState.newTab() }
                    .keyboardShortcut("n")
                Button("打开...") { appState.openFile() }
                    .keyboardShortcut("o")
            }

            CommandGroup(replacing: .saveItem) {
                Button("保存") {
                    if let doc = appState.activeDocument {
                        appState.saveDocument(doc)
                    }
                }
                .keyboardShortcut("s")

                Button("另存为...") {
                    if let doc = appState.activeDocument {
                        appState.saveAsDocument(doc)
                    }
                }
                .keyboardShortcut("s", modifiers: [.command, .shift])
            }

            CommandGroup(before: .toolbar) {
                Button("Markdown 渲染切换") {
                    if let doc = appState.activeDocument {
                        appState.toggleRendering(for: doc)
                    }
                }
                .keyboardShortcut("r")
            }

            // Edit menu additions
            CommandGroup(after: .textEditing) {
                Divider()
                Button("查找...") {
                    NotificationCenter.default.post(name: .showFindPanel, object: nil)
                }
                .keyboardShortcut("f")

                Button("查找替换...") {
                    NotificationCenter.default.post(name: .showFindReplacePanel, object: nil)
                }
                .keyboardShortcut("f", modifiers: [.command, .option])
            }

            // View menu — zoom
            CommandGroup(before: .textFormatting) {
                Button("放大") { appState.zoomIn() }
                    .keyboardShortcut("=", modifiers: [.command])
                Button("缩小") { appState.zoomOut() }
                    .keyboardShortcut("-", modifiers: [.command])
                Button("实际大小") { appState.resetZoom() }
                    .keyboardShortcut("0", modifiers: [.command])
            }

            // Window menu — close tab
            CommandGroup(after: .windowSize) {
                Button("关闭标签页") {
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
