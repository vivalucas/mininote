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
        textView.textContainer?.containerWidth = wordWrap
            ? scrollView.contentSize.width
            : CGFloat.greatestFiniteMagnitude
        textView.string = text
        textView.isAutomaticSpellingCorrectionEnabled = false
        textView.isAutomaticGrammarCheckingEnabled = false
        textView.isAutomaticQuoteSubstitutionEnabled = false
        textView.isAutomaticDashSubstitutionEnabled = false
        textView.isAutomaticTextReplacementEnabled = false

        scrollView.documentView = textView

        context.coordinator.textView = textView
        context.coordinator.parent = self
        context.coordinator.currentFontSize = fontSize

        // Observe zoom notifications
        context.coordinator.zoomObserver = NotificationCenter.default.addObserver(
            forName: .editorZoomIn,
            object: nil,
            queue: .main
        ) { _ in context.coordinator.zoomIn() }

        context.coordinator.zoomOutObserver = NotificationCenter.default.addObserver(
            forName: .editorZoomOut,
            object: nil,
            queue: .main
        ) { _ in context.coordinator.zoomOut() }

        context.coordinator.zoomResetObserver = NotificationCenter.default.addObserver(
            forName: .editorZoomReset,
            object: nil,
            queue: .main
        ) { _ in context.coordinator.zoomReset() }

        return scrollView
    }

    func updateNSView(_ scrollView: NSScrollView, context: Context) {
        guard let textView = scrollView.documentView as? NSTextView else { return }

        if textView.string != text {
            textView.string = text
        }

        let newFont = NSFont.monospacedSystemFont(ofSize: fontSize, weight: .regular)
        if textView.font != newFont {
            textView.font = newFont
        }

        textView.isHorizontallyResizable = !wordWrap
        textView.textContainer?.widthTracksTextView = wordWrap
        textView.textContainer?.containerWidth = wordWrap
            ? scrollView.contentSize.width
            : CGFloat.greatestFiniteMagnitude

        context.coordinator.parent = self
        context.coordinator.currentFontSize = fontSize

        // Restore cursor position
        if textView.selectedRange().location != cursorPosition,
           cursorPosition <= textView.string.utf16.count {
            textView.setSelectedRange(NSRange(location: cursorPosition, length: 0))
        }
    }

    func dismantleNSView(_ nsView: NSScrollView, coordinator: Coordinator) {
        if let obs = coordinator.zoomObserver {
            NotificationCenter.default.removeObserver(obs)
        }
        if let obs = coordinator.zoomOutObserver {
            NotificationCenter.default.removeObserver(obs)
        }
        if let obs = coordinator.zoomResetObserver {
            NotificationCenter.default.removeObserver(obs)
        }
    }

    // MARK: - Coordinator

    final class Coordinator: NSObject, NSTextViewDelegate {
        @Binding var text: String
        @Binding var cursorPosition: Int
        weak var textView: NSTextView?
        var parent: EditorTextView?
        var currentFontSize: Double = 14

        var zoomObserver: Any?
        var zoomOutObserver: Any?
        var zoomResetObserver: Any?

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

        // MARK: - Zoom

        func zoomIn() {
            currentFontSize = min(currentFontSize + 2, 72)
            applyFontSize()
        }

        func zoomOut() {
            currentFontSize = max(currentFontSize - 2, 8)
            applyFontSize()
        }

        func zoomReset() {
            currentFontSize = parent?.fontSize ?? 14
            applyFontSize()
        }

        private func applyFontSize() {
            textView?.font = NSFont.monospacedSystemFont(
                ofSize: currentFontSize,
                weight: .regular
            )
        }
    }
}
