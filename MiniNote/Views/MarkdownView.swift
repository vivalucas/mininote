import SwiftUI

struct MarkdownView: View {
    let content: String
    var fontSize: Double

    var body: some View {
        ScrollView {
            if let attributedString = try? AttributedString(
                markdown: content,
                options: AttributedString.MarkdownParsingOptions(
                    allowsExtendedAttributes: true,
                    interpretedSyntax: .inlineOnlyPreservingWhitespace
                )
            ) {
                Text(attributedString)
                    .font(.system(size: fontSize))
                    .textSelection(.enabled)
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding()
            } else {
                Text(content)
                    .font(.system(size: fontSize))
                    .textSelection(.enabled)
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding()
            }
        }
    }
}
