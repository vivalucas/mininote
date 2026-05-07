import SwiftUI

struct StatusBar: View {
    let document: Document

    private var lineInfo: String {
        "行 \(currentLine), 列 \(currentColumn)"
    }

    private var charCount: String {
        "\(document.content.count) 字符"
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
        document.isRendering ? "渲染" : "纯文本"
    }

    // Approximation — NSTextView provides exact values via binding
    var currentLine: Int = 1
    var currentColumn: Int = 1

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
