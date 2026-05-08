import Foundation

struct SessionTabInfo: Codable, Sendable {
    let id: UUID
    let type: DocumentType
    let fileURL: URL?
    let cursorPosition: Int
    let isModified: Bool
    let isRendering: Bool

    init(
        id: UUID,
        type: DocumentType,
        fileURL: URL?,
        cursorPosition: Int,
        isModified: Bool = false,
        isRendering: Bool
    ) {
        self.id = id
        self.type = type
        self.fileURL = fileURL
        self.cursorPosition = cursorPosition
        self.isModified = isModified
        self.isRendering = isRendering
    }

    enum CodingKeys: String, CodingKey {
        case id
        case type
        case fileURL
        case cursorPosition
        case isModified
        case isRendering
    }

    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        id = try container.decode(UUID.self, forKey: .id)
        type = try container.decode(DocumentType.self, forKey: .type)
        fileURL = try container.decodeIfPresent(URL.self, forKey: .fileURL)
        cursorPosition = try container.decode(Int.self, forKey: .cursorPosition)
        isModified = try container.decodeIfPresent(Bool.self, forKey: .isModified) ?? false
        isRendering = try container.decode(Bool.self, forKey: .isRendering)
    }
}

struct SessionData: Codable, Sendable {
    var tabs: [SessionTabInfo]
    var activeTabIndex: Int
}

actor SessionStore {
    private let sessionsDir: URL
    private let sessionFile: URL
    private var writeTask: Task<Void, Never>?
    /// Tab content waiting to be persisted after debounce.
    private var pendingContent: [UUID: PendingContent] = [:]

    private struct PendingContent: Sendable {
        let content: String
        let isScratch: Bool
    }

    init() {
        let appSupport = FileManager.default.urls(
            for: .applicationSupportDirectory,
            in: .userDomainMask
        ).first!
        sessionsDir = appSupport
            .appendingPathComponent("MiniNote")
            .appendingPathComponent("sessions")
        sessionFile = sessionsDir.appendingPathComponent("session.json")
        try? FileManager.default.createDirectory(
            at: sessionsDir,
            withIntermediateDirectories: true
        )
    }

    // MARK: - Session metadata

    func saveSession(tabInfos: [SessionTabInfo], activeIndex: Int) {
        let data = SessionData(tabs: tabInfos, activeTabIndex: activeIndex)
        if let encoded = try? JSONEncoder().encode(data) {
            try? encoded.write(to: sessionFile, options: .atomic)
        }
    }

    func loadSession() -> (tabs: [SessionTabInfo], activeIndex: Int)? {
        guard let data = try? Data(contentsOf: sessionFile),
              let session = try? JSONDecoder().decode(SessionData.self, from: data)
        else { return nil }
        return (session.tabs, session.activeTabIndex)
    }

    // MARK: - Tab content auto-save (debounced + lifecycle)

    /// Called on every content change for a document.
    /// Stores content in memory and (re)schedules a 3-second debounced write.
    func scheduleContentWrite(id: UUID, content: String, isScratch: Bool) {
        pendingContent[id] = PendingContent(content: content, isScratch: isScratch)
        writeTask?.cancel()
        writeTask = Task {
            try? await Task.sleep(for: .seconds(3))
            guard !Task.isCancelled else { return }
            self.flushPendingContent()
        }
    }

    /// Immediate write for a single document (tab switch, app resign).
    func flushImmediately(id: UUID, content: String, isScratch: Bool) {
        writeTask?.cancel()
        writeTask = nil
        writeContent(id: id, content: content, isScratch: isScratch)
        pendingContent.removeValue(forKey: id)
    }

    /// Flush all open documents to disk-backed session storage.
    func flushAll(documents: [(id: UUID, content: String, isScratch: Bool)]) {
        writeTask?.cancel()
        writeTask = nil
        for (id, content, isScratch) in documents {
            writeContent(id: id, content: content, isScratch: isScratch)
        }
        pendingContent.removeAll()
    }

    private func flushPendingContent() {
        let pending = pendingContent
        pendingContent.removeAll()
        writeTask = nil
        for (id, value) in pending {
            writeContent(id: id, content: value.content, isScratch: value.isScratch)
        }
    }

    // MARK: - Individual file I/O

    func writeContent(id: UUID, content: String, isScratch: Bool) {
        let fileURL = contentFileURL(for: id, isScratch: isScratch)
        try? content.write(to: fileURL, atomically: true, encoding: .utf8)
    }

    func readContent(id: UUID, isScratch: Bool) -> String? {
        let fileURL = contentFileURL(for: id, isScratch: isScratch)
        return try? String(contentsOf: fileURL, encoding: .utf8)
    }

    func deleteContentFiles(id: UUID) {
        pendingContent.removeValue(forKey: id)
        try? FileManager.default.removeItem(at: scratchFileURL(for: id))
        try? FileManager.default.removeItem(at: draftFileURL(for: id))
    }

    func deleteDraftFile(id: UUID) {
        pendingContent.removeValue(forKey: id)
        try? FileManager.default.removeItem(at: draftFileURL(for: id))
    }

    private func contentFileURL(for id: UUID, isScratch: Bool) -> URL {
        isScratch ? scratchFileURL(for: id) : draftFileURL(for: id)
    }

    private func scratchFileURL(for id: UUID) -> URL {
        sessionsDir.appendingPathComponent("\(id.uuidString).mint")
    }

    private func draftFileURL(for id: UUID) -> URL {
        sessionsDir.appendingPathComponent("\(id.uuidString).draft")
    }
}
