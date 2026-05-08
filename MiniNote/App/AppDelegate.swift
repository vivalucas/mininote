import AppKit

@MainActor
final class AppDelegate: NSObject, NSApplicationDelegate {
    var appState: AppState?
    private var isTerminating = false

    func applicationDidResignActive(_ notification: Notification) {
        NotificationCenter.default.post(name: .appDidResignActive, object: nil)
    }

    func applicationShouldTerminate(_ sender: NSApplication) -> NSApplication.TerminateReply {
        guard !isTerminating else { return .terminateNow }
        isTerminating = true
        Task {
            if let appState {
                await appState.flushAndSaveBeforeTerminate()
            }
            sender.reply(toApplicationShouldTerminate: true)
        }
        return .terminateLater
    }

    func applicationDidFinishLaunching(_ notification: Notification) {
        NSWindow.allowsAutomaticWindowTabbing = false
    }

    func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
        if !flag {
            sender.windows.first?.makeKeyAndOrderFront(nil)
        }
        return true
    }
}

// MARK: - Notification Names

extension Notification.Name {
    static let appDidResignActive = Notification.Name("appDidResignActive")
    static let appWillTerminate = Notification.Name("appWillTerminate")
    static let editorZoomIn = Notification.Name("editorZoomIn")
    static let editorZoomOut = Notification.Name("editorZoomOut")
    static let editorZoomReset = Notification.Name("editorZoomReset")
    static let closeCurrentTab = Notification.Name("closeCurrentTab")
    static let showFindPanel = Notification.Name("showFindPanel")
    static let showFindReplacePanel = Notification.Name("showFindReplacePanel")
}
