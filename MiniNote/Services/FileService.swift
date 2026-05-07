import Foundation

final class FileService {
    enum FileError: LocalizedError {
        case cannotRead
        case cannotWrite
        case unsupportedType

        var errorDescription: String? {
            switch self {
            case .cannotRead:  "无法读取文件"
            case .cannotWrite: "无法写入文件"
            case .unsupportedType: "不支持的文件格式"
            }
        }
    }

    func open(url: URL) throws -> Document {
        let type = detectType(from: url)
        let content = try String(contentsOf: url, encoding: .utf8)
        let lineEnding = detectLineEnding(in: content)

        return Document(
            content: content,
            type: type,
            fileURL: url,
            cursorPosition: 0,
            isModified: false,
            isRendering: false
        )
    }

    func save(document: Document) throws {
        guard let url = document.fileURL else {
            throw FileError.cannotWrite
        }
        try document.content.write(to: url, atomically: true, encoding: .utf8)
        document.isModified = false
    }

    func saveAs(document: Document, to url: URL) throws {
        let newType = detectType(from: url)
        // Ensure line endings are normalized for the new file
        let content = document.content
        try content.write(to: url, atomically: true, encoding: .utf8)
        document.fileURL = url
        document.type = newType
        document.isModified = false
    }

    // MARK: - Helpers

    private func detectType(from url: URL) -> DocumentType {
        switch url.pathExtension.lowercased() {
        case "mint": .mint
        case "md", "markdown": .md
        default: .txt
        }
    }

    private func detectLineEnding(in content: String) -> LineEnding {
        if content.contains("\r\n") { .crlf } else { .lf }
    }

    func defaultFileName(type: DocumentType) -> String {
        "未标题.\(type.fileExtension)"
    }

    func isSupportedExtension(_ ext: String) -> Bool {
        ["mint", "txt", "md", "markdown"].contains(ext.lowercased())
    }
}
