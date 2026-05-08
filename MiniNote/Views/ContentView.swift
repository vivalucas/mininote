import SwiftUI

struct ContentView: View {
    @Environment(AppState.self) private var state

    // Settings (persisted via @AppStorage, shared with SettingsView)
    @AppStorage("fontSize") private var fontSize: Double = 16
    @AppStorage("wordWrap") private var wordWrap: Bool = true
    @AppStorage("showLineNumbers") private var showLineNumbers: Bool = false
    @AppStorage("theme") private var theme: String = "system"

    // Close alert
    @State private var closeTarget: Document? = nil
    @State private var showCloseAlert = false

    // First launch
    @AppStorage("hasLaunchedBefore") private var hasLaunchedBefore = false
    @State private var showFirstLaunch = false

    var body: some View {
        @Bindable var state = state

        VStack(spacing: 0) {
            TabBar(
                documents: $state.documents,
                activeTabId: $state.activeTabId,
                onNewTab: { state.newTab() },
                onCloseTab: { doc in requestClose(doc) },
                onSelectTab: { id in state.switchToTab(id) },
                onReorder: { from, to in
                    state.documents.move(fromOffsets: from, toOffset: to)
                    state.saveSessionSnapshot()
                }
            )

            Divider()

            if let doc = state.activeDocument {
                if doc.isRendering {
                    MarkdownView(content: doc.content, fontSize: fontSize)
                } else {
                    EditorTextView(
                        text: Binding(
                            get: { doc.content },
                            set: { newValue in
                                doc.content = newValue
                                doc.isModified = true
                                state.scheduleContentSave(for: doc)
                            }
                        ),
                        cursorPosition: Binding(
                            get: { doc.cursorPosition },
                            set: { pos in
                                doc.cursorPosition = pos
                                state.cursorPosition = pos
                            }
                        ),
                        fontSize: fontSize,
                        wordWrap: wordWrap,
                        showLineNumbers: showLineNumbers
                    )
                }
                Divider()
                StatusBar(
                    document: doc,
                    currentLine: line(at: state.cursorPosition, in: doc.content),
                    currentColumn: column(at: state.cursorPosition, in: doc.content)
                )
            }
        }
        .frame(minWidth: 600, minHeight: 400)
        .preferredColorScheme(colorScheme)
        .onAppear {
            state.restoreSessionOrStartFresh()
            if !hasLaunchedBefore {
                showFirstLaunch = true
            }
        }
        .onReceive(
            NotificationCenter.default.publisher(for: .appDidResignActive),
            perform: state.onLifecycleEvent
        )
        .onReceive(
            NotificationCenter.default.publisher(for: .appWillTerminate),
            perform: state.onLifecycleEvent
        )
        .onReceive(
            NotificationCenter.default.publisher(for: .closeCurrentTab),
            perform: { notification in
                if let doc = notification.object as? Document {
                    requestClose(doc)
                }
            }
        )
        .sheet(isPresented: $showFirstLaunch) {
            FirstLaunchView {
                showFirstLaunch = false
                hasLaunchedBefore = true
            }
        }
        .alert(String(localized: "alert.saveChanges"), isPresented: $showCloseAlert) {
            Button(String(localized: "alert.save")) {
                guard let doc = closeTarget else { return }
                state.saveDocument(doc)
                state.closeTab(doc)
                closeTarget = nil
            }
            Button(String(localized: "alert.dontSave")) {
                guard let doc = closeTarget else { return }
                state.closeTab(doc)
                closeTarget = nil
            }
            Button(String(localized: "alert.cancel"), role: .cancel) {
                closeTarget = nil
            }
        } message: {
            if let doc = closeTarget {
                Text(String(format: NSLocalizedString("alert.hasChanges", comment: ""), doc.displayName))
            }
        }
    }

    private var colorScheme: ColorScheme? {
        switch theme {
        case "light": return .light
        case "dark": return .dark
        default: return nil
        }
    }

    private func requestClose(_ doc: Document) {
        if doc.isModified {
            closeTarget = doc
            showCloseAlert = true
        } else {
            state.closeTab(doc)
        }
    }

    private func line(at pos: Int, in text: String) -> Int {
        let idx = text.utf16.index(
            text.utf16.startIndex,
            offsetBy: min(pos, text.utf16.count),
            limitedBy: text.utf16.endIndex
        ) ?? text.utf16.startIndex
        return text[..<idx].components(separatedBy: .newlines).count
    }

    private func column(at pos: Int, in text: String) -> Int {
        let clampedPos = min(pos, text.utf16.count)
        let idx = text.utf16.index(
            text.utf16.startIndex,
            offsetBy: clampedPos,
            limitedBy: text.utf16.endIndex
        ) ?? text.utf16.startIndex
        if let lastNewline = text[..<idx].lastIndex(of: "\n") {
            return text.distance(from: lastNewline, to: idx) + 1
        }
        return clampedPos + 1
    }
}
