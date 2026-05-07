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
            CommandGroup(replacing: .newItem) {
                Button("新建") {
                    appState.newTab()
                }
                .keyboardShortcut("n")
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

            CommandGroup(replacing: .textFormatting) {
                Button("查找") {
                    // Handled by NSTextView natively via Cmd+F
                }
                .keyboardShortcut("f")

                Button("查找替换") {
                    // Trigger find-and-replace panel
                    NSApp.sendAction(
                        #selector(NSTextView.performFindPanelAction(_:)),
                        to: nil, from: nil
                    )
                }
                .keyboardShortcut("f", modifiers: [.command, .option])
            }
        }

        Settings {
            Text("偏好设置请在主窗口菜单栏 → 偏好设置中打开。")
                .padding()
        }
    }
}
