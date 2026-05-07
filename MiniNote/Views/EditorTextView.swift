import SwiftUI
import AppKit

struct EditorTextView: NSViewRepresentable {
    @Binding var text: String
    @Binding var cursorPosition: Int
    var fontSize: Double
    var wordWrap: Bool
    var showLineNumbers: Bool

    func makeCoordinator() -> Coordinator {
        Coordinator(text: $text, cursorPosition: $cursorPosition)
    }

    func makeNSView(context: Context) -> NSScrollView {
        let scrollView = NSScrollView()
        scrollView.hasVerticalScroller = true
        scrollView.hasHorizontalScroller = false
        scrollView.autohidesScrollers = true
        scrollView.borderType = .noBorder

        let textView = NSTextView()
        textView.delegate = context.coordinator
        textView.allowsUndo = true
        textView.isRichText = false
        textView.font = NSFont.monospacedSystemFont(ofSize: fontSize, weight: .regular)
        textView.textColor = .textColor
        textView.backgroundColor = .textBackgroundColor
        textView.isVerticallyResizable = true
        textView.isHorizontallyResizable = !wordWrap
        textView.textContainer?.widthTracksTextView = wordWrap
        textView.textContainer?.containerWidth = wordWrap ? scrollView.contentSize.width : CGFloat.greatestFiniteMagnitude
        textView.string = text
        textView.isAutomaticSpellingCorrectionEnabled = false
        textView.isAutomaticGrammarCheckingEnabled = false
        textView.isAutomaticQuoteSubstitutionEnabled = false
        textView.isAutomaticDashSubstitutionEnabled = false
        textView.isAutomaticTextReplacementEnabled = false

        scrollView.documentView = textView

        context.coordinator.textView = textView
        context.coordinator.parent = self

        return scrollView
    }

    func updateNSView(_ scrollView: NSScrollView, context: Context) {
        guard let textView = scrollView.documentView as? NSTextView else { return }

        if textView.string != text {
            textView.string = text
        }
        textView.font = NSFont.monospacedSystemFont(ofSize: fontSize, weight: .regular)
        textView.isHorizontallyResizable = !wordWrap
        textView.textContainer?.widthTracksTextView = wordWrap
        textView.textContainer?.containerWidth = wordWrap
            ? scrollView.contentSize.width
            : CGFloat.greatestFiniteMagnitude

        context.coordinator.parent = self

        // Restore cursor position if needed
        if textView.selectedRange().location != cursorPosition,
           cursorPosition <= textView.string.utf16.count {
            textView.setSelectedRange(NSRange(location: cursorPosition, length: 0))
        }
    }

    // MARK: - Coordinator

    final class Coordinator: NSObject, NSTextViewDelegate {
        @Binding var text: String
        @Binding var cursorPosition: Int
        weak var textView: NSTextView?
        var parent: EditorTextView?

        init(text: Binding<String>, cursorPosition: Binding<Int>) {
            _text = text
            _cursorPosition = cursorPosition
        }

        func textDidChange(_ notification: Notification) {
            guard let textView else { return }
            DispatchQueue.main.async {
                self.text = textView.string
                self.cursorPosition = textView.selectedRange().location
            }
        }

        func textViewDidChangeSelection(_ notification: Notification) {
            guard let textView else { return }
            DispatchQueue.main.async {
                self.cursorPosition = textView.selectedRange().location
            }
        }

        // Override paste to force plain text only
        func textView(_ textView: NSTextView, shouldChangeTextIn affectedCharRange: NSRange, replacementString: String?) -> String? {
            // NSTextView paste goes through here — the replacementString
            // from paste is already plain text because isRichText = false.
            // We also handle programmatic insertions here.
            return replacementString
        }
    }
}
