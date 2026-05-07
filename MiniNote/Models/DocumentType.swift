import Foundation

enum DocumentType: String, CaseIterable, Codable {
    case mint
    case txt
    case md

    var displayName: String {
        switch self {
        case .mint: "MiniNote 文档"
        case .txt:  "纯文本文档"
        case .md:   "Markdown 文档"
        }
    }

    var fileExtension: String { rawValue }

    var defaultRenderSetting: Bool { false }
}
