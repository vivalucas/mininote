import SwiftUI
import Observation

@MainActor
@Observable
final class AppState {
    var documents: [Document] = []
    var activeTabId: UUID = UUID()
    var cursorPosition: Int = 0

    var activeDocument: Document? {
        documents.first(where: { $0.id == activeTabId })
    }

    private let fileService = FileService()
    private let sessionStore = SessionStore()

    // MARK: - Tab Management

    func newTab() {
        let doc = Document(isRendering: defaultRender(for: .mint))
        documents.append(doc)
        activeTabId = doc.id
        saveSessionSnapshot()
    }

    func closeTab(_ doc: Document) {
        if doc.isScratch {
            let store = sessionStore
            let docId = doc.id
            Task { await store.deleteScratchFile(id: docId) }
        }
        documents.removeAll { $0.id == doc.id }
        if activeTabId == doc.id {
            activeTabId = documents.first?.id ?? UUID()
        }
        if documents.isEmpty {
            newTab()
        }
        saveSessionSnapshot()
    }

    func switchToTab(_ id: UUID) {
        guard id != activeTabId else { return }
        if let current = activeDocument, current.isScratch {
            let store = sessionStore
            let docId = current.id
            let content = current.content
            Task {
                await store.flushImmediately(id: docId, content: content)
            }
        }
        activeTabId = id
        if let target = documents.first(where: { $0.id == id }) {
            cursorPosition = target.cursorPosition
        }
        saveSessionSnapshot()
    }

    // MARK: - Markdown Toggle

    func toggleRendering(for doc: Document) {
        doc.isRendering.toggle()
    }

    // MARK: - File Operations

    func openFile() {
        let panel = NSOpenPanel()
        panel.allowedContentTypes = [.plainText, .init(filenameExtension: "mint")!]
        panel.allowsMultipleSelection = false
        guard panel.runModal() == .OK, let url = panel.url else { return }
        if let existing = documents.first(where: { $0.fileURL == url }) {
            activeTabId = existing.id
            return
        }
        if let doc = try? fileService.open(url: url) {
            doc.isRendering = defaultRender(for: doc.type)
            documents.append(doc)
            activeTabId = doc.id
            saveSessionSnapshot()
        }
    }

    func saveDocument(_ doc: Document) {
        if doc.fileURL != nil {
            try? fileService.save(document: doc)
        } else {
            saveAsDocument(doc)
        }
    }

    func saveAsDocument(_ doc: Document) {
        let panel = NSSavePanel()
        panel.allowedContentTypes = [.plainText, .init(filenameExtension: "mint")!]
        panel.nameFieldStringValue = doc.fileName
        guard panel.runModal() == .OK, let url = panel.url else { return }
        try? fileService.saveAs(document: doc, to: url)
    }

    // MARK: - Zoom

    func zoomIn() {
        // Handled by NSTextView via font size changes.
        // Post notification that EditorTextView can observe.
        NotificationCenter.default.post(name: .editorZoomIn, object: nil)
    }

    func zoomOut() {
        NotificationCenter.default.post(name: .editorZoomOut, object: nil)
    }

    func resetZoom() {
        NotificationCenter.default.post(name: .editorZoomReset, object: nil)
    }

    // MARK: - Session Persistence

    func scheduleScratchSave(for doc: Document) {
        guard doc.isScratch else { return }
        let store = sessionStore
        let docId = doc.id
        let content = doc.content
        Task {
            await store.scheduleScratchWrite(id: docId, content: content)
        }
    }

    func flushCurrentScratch() {
        guard let doc = activeDocument, doc.isScratch else { return }
        let store = sessionStore
        let docId = doc.id
        let content = doc.content
        Task {
            await store.flushImmediately(id: docId, content: content)
        }
    }

    func flushAllScratch() {
        let scratchDocs = documents.filter { $0.isScratch }.map {
            (id: $0.id, content: $0.content)
        }
        let store = sessionStore
        Task {
            await store.flushAll(scratchDocs: scratchDocs)
        }
    }

    func saveSessionSnapshot() {
        let activeIndex = documents.firstIndex(where: { $0.id == activeTabId }) ?? 0
        let tabInfos = documents.map { doc in
            SessionTabInfo(
                id: doc.id,
                type: doc.type,
                fileURL: doc.fileURL,
                cursorPosition: doc.cursorPosition,
                isRendering: doc.isRendering
            )
        }
        let store = sessionStore
        Task { await store.saveSession(tabInfos: tabInfos, activeIndex: activeIndex) }
    }

    func restoreSessionOrStartFresh() {
        let behavior = UserDefaults.standard.string(forKey: "startupBehavior") ?? "continue"
        guard behavior == "continue" else {
            newTab()
            return
        }
        let store = sessionStore
        Task { [weak self] in
            if let (tabInfos, activeIndex) = await store.loadSession(),
               !tabInfos.isEmpty {
                var restored: [Document] = []
                for info in tabInfos {
                    let doc = Document(
                        id: info.id,
                        type: info.type,
                        fileURL: info.fileURL,
                        cursorPosition: info.cursorPosition,
                        isRendering: info.isRendering
                    )
                    if doc.isScratch {
                        doc.content = await store.readScratchContent(id: info.id) ?? ""
                    } else if let url = info.fileURL {
                        doc.content = (try? String(contentsOf: url, encoding: .utf8)) ?? ""
                    }
                    restored.append(doc)
                }
                let restoredTabs = restored
                await MainActor.run {
                    guard let self else { return }
                    self.documents = restoredTabs
                    let idx = min(activeIndex, restoredTabs.count - 1)
                    self.activeTabId = restoredTabs[idx].id
                    self.cursorPosition = restoredTabs[idx].cursorPosition
                }
            } else {
                await MainActor.run { self?.newTab() }
            }
        }
    }

    // MARK: - Lifecycle

    func onLifecycleEvent(_ notification: Notification) {
        switch notification.name {
        case .appDidResignActive:
            flushCurrentScratch()
            saveSessionSnapshot()
        case .appWillTerminate:
            flushAllScratch()
            saveSessionSnapshot()
        default:
            break
        }
    }

    // MARK: - Settings Helpers

    private func defaultRender(for type: DocumentType) -> Bool {
        let key: String = switch type {
        case .mint: "renderMint"
        case .txt:  "renderTxt"
        case .md:   "renderMd"
        }
        return UserDefaults.standard.bool(forKey: key)
    }
}
