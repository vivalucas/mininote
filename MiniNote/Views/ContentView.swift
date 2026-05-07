import SwiftUI

struct ContentView: View {
    @Environment(AppState.self) private var state

    // Settings
    @State private var fontSize: Double = 14
    @State private var wordWrap: Bool = true
    @State private var showLineNumbers: Bool = false
    @State private var showSettings = false

    // Close alert
    @State private var pendingCloseDoc: Document? = nil
    @State private var showCloseAlert = false

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
                    state.saveSession()
                }
            )

            Divider()

            // Editor area
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
                    currentLine: lineNumber(at: state.cursorPosition, in: doc.content),
                    currentColumn: columnNumber(at: state.cursorPosition, in: doc.content)
                )
            }
        }
        .frame(minWidth: 600, minHeight: 400)
        .onAppear { state.restoreSessionOrStartFresh() }
        .onReceive(
            NotificationCenter.default.publisher(for: .appDidResignActive),
            perform: state.onLifecycleEvent
        )
        .onReceive(
            NotificationCenter.default.publisher(for: .appWillTerminate),
            perform: state.onLifecycleEvent
        )
        .sheet(isPresented: $showSettings) {
            SettingsView(
                fontSize: $fontSize,
                wordWrap: $wordWrap,
                showLineNumbers: $showLineNumbers,
                documents: $state.documents
            )
        }
        .alert("是否保存更改？", isPresented: $showCloseAlert, presenting: pendingCloseDoc) { doc in
            Button("保存") {
                state.saveDocument(doc)
                state.closeTab(doc)
            }
            Button("不保存") { state.closeTab(doc) }
            Button("取消", role: .cancel) { pendingCloseDoc = nil }
        } message: { doc in
            Text("「\(doc.fileName)」有未保存的更改。")
        }
    }

    // MARK: - Close flow

    private func requestClose(_ doc: Document) {
        if doc.isModified {
            pendingCloseDoc = doc
            showCloseAlert = true
        } else {
            state.closeTab(doc)
        }
    }

    // MARK: - Helpers

    private func lineNumber(at pos: Int, in text: String) -> Int {
        let end = text.utf16.index(
            text.utf16.startIndex,
            offsetBy: min(pos, text.utf16.count),
            limitedBy: text.utf16.endIndex
        ) ?? text.utf16.startIndex
        return text[..<end].components(separatedBy: .newlines).count
    }

    private func columnNumber(at pos: Int, in text: String) -> Int {
        let end = text.utf16.index(
            text.utf16.startIndex,
            offsetBy: min(pos, text.utf16.count),
            limitedBy: text.utf16.endIndex
        ) ?? text.utf16.startIndex
        if let lastNewline = text[..<end].lastIndex(of: "\n") {
            return text.distance(from: lastNewline, to: end) + 1
        }
        return pos + 1
    }
}
