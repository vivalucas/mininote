import SwiftUI
import AppKit

// MARK: - Plain text NSTextView (enforces paste-as-plain-text)

final class PlainTextView: NSTextView {
    override func paste(_ sender: Any?) {
        // Force plain text paste, stripping all formatting
        let pasteboard = NSPasteboard.general
        if let plainText = pasteboard.string(forType: .string) {
            // Delete any selected text first
            if let range = selectedRanges.first as? NSRange {
                insertText(plainText, replacementRange: range)
            }
        }
    }
}

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

        let textView = PlainTextView()
        textView.delegate = context.coordinator
        textView.allowsUndo = true
        textView.isRichText = false
        textView.font = NSFont.monospacedSystemFont(ofSize: fontSize, weight: .regular)
        textView.textColor = .textColor
        textView.backgroundColor = .textBackgroundColor
        textView.isVerticallyResizable = true
        textView.isHorizontallyResizable = !wordWrap
        textView.textContainer?.widthTracksTextView = wordWrap
        if wordWrap {
            textView.textContainer?.containerSize = CGSize(
                width: scrollView.contentSize.width,
                height: CGFloat.greatestFiniteMagnitude
            )
        } else {
            textView.textContainer?.containerSize = CGSize(
                width: CGFloat.greatestFiniteMagnitude,
                height: CGFloat.greatestFiniteMagnitude
            )
        }
        textView.string = text
        textView.isAutomaticSpellingCorrectionEnabled = false
        textView.isAutomaticQuoteSubstitutionEnabled = false
        textView.isAutomaticDashSubstitutionEnabled = false
        textView.isAutomaticTextReplacementEnabled = false

        scrollView.documentView = textView

        // Line number gutter
        let gutter = LineNumberGutter(font: NSFont.monospacedSystemFont(ofSize: fontSize, weight: .regular))
        gutter.clientView = textView
        scrollView.verticalRulerView = gutter
        scrollView.hasVerticalRuler = showLineNumbers
        scrollView.rulersVisible = showLineNumbers

        context.coordinator.textView = textView
        context.coordinator.parent = self
        context.coordinator.currentFontSize = fontSize
        context.coordinator.gutter = gutter

        // Observe scroll to update line numbers
        context.coordinator.clipViewObserver = NotificationCenter.default.addObserver(
            forName: NSView.boundsDidChangeNotification,
            object: scrollView.contentView,
            queue: .main
        ) { [weak gutter] _ in
            gutter?.needsDisplay = true
        }

        // Observe find/replace notifications
        context.coordinator.findPanelObserver = NotificationCenter.default.addObserver(
            forName: .showFindPanel,
            object: nil,
            queue: .main
        ) { [weak textView] _ in
            textView?.performFindPanelAction(nil)
        }

        context.coordinator.findReplacePanelObserver = NotificationCenter.default.addObserver(
            forName: .showFindReplacePanel,
            object: nil,
            queue: .main
        ) { [weak textView] _ in
            // Open find panel with "replace" mode
            // NSTextView's performFindPanelAction(nil) shows the panel;
            // calling it twice or via showFindPanel opens with replace if available.
            textView?.performFindPanelAction(nil)
        }

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
            context.coordinator.gutter?.font = newFont
        }

        textView.isHorizontallyResizable = !wordWrap
        textView.textContainer?.widthTracksTextView = wordWrap
        if wordWrap {
            textView.textContainer?.containerSize = CGSize(
                width: scrollView.contentSize.width,
                height: CGFloat.greatestFiniteMagnitude
            )
        } else {
            textView.textContainer?.containerSize = CGSize(
                width: CGFloat.greatestFiniteMagnitude,
                height: CGFloat.greatestFiniteMagnitude
            )
        }

        // Update line numbers visibility
        scrollView.hasVerticalRuler = showLineNumbers
        scrollView.rulersVisible = showLineNumbers
        if showLineNumbers {
            context.coordinator.gutter?.needsDisplay = true
        }

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
        if let obs = coordinator.clipViewObserver {
            NotificationCenter.default.removeObserver(obs)
        }
        if let obs = coordinator.findPanelObserver {
            NotificationCenter.default.removeObserver(obs)
        }
        if let obs = coordinator.findReplacePanelObserver {
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
        weak var gutter: LineNumberGutter?

        var zoomObserver: Any?
        var zoomOutObserver: Any?
        var zoomResetObserver: Any?
        var clipViewObserver: Any?
        var findPanelObserver: Any?
        var findReplacePanelObserver: Any?

        init(text: Binding<String>, cursorPosition: Binding<Int>) {
            _text = text
            _cursorPosition = cursorPosition
        }

        func textDidChange(_ notification: Notification) {
            guard let textView else { return }
            DispatchQueue.main.async {
                self.text = textView.string
                self.cursorPosition = textView.selectedRange().location
                self.gutter?.needsDisplay = true
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
            let font = NSFont.monospacedSystemFont(ofSize: currentFontSize, weight: .regular)
            textView?.font = font
            gutter?.font = font
            gutter?.needsDisplay = true
        }
    }
}

// MARK: - Line Number Gutter

final class LineNumberGutter: NSRulerView {
    var font: NSFont {
        didSet { needsDisplay = true }
    }

    init(font: NSFont) {
        self.font = font
        super.init(scrollView: nil, orientation: .verticalRuler)
        ruleThickness = 40
    }

    required init(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    override func draw(_ dirtyRect: NSRect) {
        guard let textView = clientView as? NSTextView,
              let layoutManager = textView.layoutManager,
              let textContainer = textView.textContainer
        else { return }

        NSColor.textBackgroundColor.set()
        dirtyRect.fill()

        let visibleRect = textView.visibleRect
        let nsString = textView.string as NSString
        let attrs: [NSAttributedString.Key: Any] = [
            .font: font,
            .foregroundColor: NSColor.secondaryLabelColor
        ]

        // Find the glyph range for the visible area
        let glyphRange = layoutManager.glyphRange(
            forBoundingRect: visibleRect,
            in: textContainer
        )

        // Walk through each line fragment in the visible range
        var glyphIndex = glyphRange.location
        while glyphIndex < NSMaxRange(glyphRange) {
            var effectiveRange = NSRange(location: 0, length: 0)
            let lineRect = layoutManager.lineFragmentRect(
                forGlyphAt: glyphIndex,
                effectiveRange: &effectiveRange,
                withoutAdditionalLayout: false
            )

            let charIndex = layoutManager.characterIndexForGlyph(at: glyphIndex)
            let y = NSMinY(lineRect) - NSMinY(visibleRect)

            // Count line number (1-based)
            let beforeText = nsString.substring(to: charIndex)
            let lineNum = beforeText.components(separatedBy: "\n").count
            let str = "\(lineNum)"
            let size = str.size(withAttributes: attrs)
            let x = ruleThickness - size.width - 6
            str.draw(at: NSPoint(x: x, y: y), withAttributes: attrs)

            // Advance to next line fragment
            glyphIndex = NSMaxRange(effectiveRange)
        }
    }
}
