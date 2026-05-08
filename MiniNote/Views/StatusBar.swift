import SwiftUI

struct StatusBar: View {
    let document: Document

    private var lineInfo: String {
        String(format: NSLocalizedString("status.position", comment: ""), currentLine, currentColumn)
    }

    private var charCount: String {
        String(format: NSLocalizedString("status.chars", comment: ""), document.content.count)
    }

    private var encodingLabel: String {
        switch document.encoding {
        case .utf8: "UTF-8"
        default: "UTF-8"
        }
    }

    private var lineEndingLabel: String {
        document.lineEnding.rawValue
    }

    private var renderLabel: String {
        String(localized: document.isRendering ? "status.rendered" : "status.plain")
    }

    let currentLine: Int
    let currentColumn: Int

    var body: some View {
        HStack(spacing: 16) {
            Text(lineInfo)
            Divider().frame(height: 12)
            Text(charCount)
            Divider().frame(height: 12)
            Text(encodingLabel)
            Divider().frame(height: 12)
            Text(lineEndingLabel)
            Divider().frame(height: 12)
            Text(renderLabel)
            Spacer()
        }
        .font(.system(size: 11))
        .foregroundColor(.secondary)
        .padding(.horizontal, 12)
        .padding(.vertical, 4)
        .background(Color(nsColor: .windowBackgroundColor))
    }
}
