import SwiftUI

struct ContentView: View {
    @Environment(AppState.self) private var state

    // Settings
    @State private var fontSize: Double = 14
    @State private var wordWrap: Bool = true
    @State private var showLineNumbers: Bool = false
    @State private var showSettings = false

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
                                state.scheduleScratchSave(for: doc)
                            }
                        ),
                        cursorPosition: Binding(
                            get: { state.cursorPosition },
                            set: { pos in
                                state.cursorPosition = pos
                                doc.cursorPosition = pos
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
        .sheet(isPresented: $showSettings) {
            SettingsView(
                fontSize: $fontSize,
                wordWrap: $wordWrap,
                showLineNumbers: $showLineNumbers
            )
        }
        .sheet(isPresented: $showFirstLaunch) {
            FirstLaunchView {
                showFirstLaunch = false
                hasLaunchedBefore = true
            }
        }
        .alert("是否保存更改？", isPresented: $showCloseAlert) {
            Button("保存") {
                guard let doc = closeTarget else { return }
                state.saveDocument(doc)
                state.closeTab(doc)
                closeTarget = nil
            }
            Button("不保存") {
                guard let doc = closeTarget else { return }
                state.closeTab(doc)
                closeTarget = nil
            }
            Button("取消", role: .cancel) {
                closeTarget = nil
            }
        } message: {
            if let doc = closeTarget {
                Text("「\(doc.fileName)」有未保存的更改。")
            }
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
        let idx = text.utf16.index(
            text.utf16.startIndex,
            offsetBy: min(pos, text.utf16.count),
            limitedBy: text.utf16.endIndex
        ) ?? text.utf16.startIndex
        if let lastNewline = text[..<idx].lastIndex(of: "\n") {
            return text.distance(from: lastNewline, to: idx) + 1
        }
        return pos + 1
    }
}
