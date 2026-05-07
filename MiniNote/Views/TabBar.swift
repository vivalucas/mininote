import SwiftUI

struct TabBar: View {
    @Binding var documents: [Document]
    @Binding var activeTabId: UUID
    var onNewTab: () -> Void
    var onCloseTab: (Document) -> Void
    var onSelectTab: (UUID) -> Void
    var onReorder: (IndexSet, Int) -> Void

    var body: some View {
        ScrollView(.horizontal, showsIndicators: false) {
            HStack(spacing: 0) {
                ForEach(documents) { doc in
                    TabButton(
                        document: doc,
                        isActive: doc.id == activeTabId,
                        onSelect: { onSelectTab(doc.id) },
                        onClose: { onCloseTab(doc) }
                    )
                }
                // "+" button
                Button(action: onNewTab) {
                    Image(systemName: "plus")
                        .font(.system(size: 12, weight: .medium))
                        .frame(width: 28, height: 28)
                        .contentShape(Rectangle())
                }
                .buttonStyle(.plain)
                .padding(.horizontal, 8)
            }
            .padding(.vertical, 4)
            .padding(.horizontal, 4)
        }
        .background(Color(nsColor: .windowBackgroundColor))
    }
}

// MARK: - Tab Button

private struct TabButton: View {
    let document: Document
    let isActive: Bool
    let onSelect: () -> Void
    let onClose: () -> Void

    @State private var isHovering = false

    var body: some View {
        Button(action: onSelect) {
            HStack(spacing: 6) {
                // File icon
                Image(systemName: document.isScratch ? "doc" : "doc.fill")
                    .font(.system(size: 11))

                // File name
                Text(document.fileName)
                    .font(.system(size: 12))
                    .lineLimit(1)

                // Unsaved indicator
                if document.isModified {
                    Circle()
                        .fill(Color.gray)
                        .frame(width: 6, height: 6)
                }

                // Close button
                Button(action: onClose) {
                    Image(systemName: "xmark")
                        .font(.system(size: 9, weight: .bold))
                        .frame(width: 16, height: 16)
                        .contentShape(Rectangle())
                }
                .buttonStyle(.plain)
                .opacity(isHovering ? 1 : 0)
            }
            .padding(.horizontal, 12)
            .padding(.vertical, 6)
            .background(
                RoundedRectangle(cornerRadius: 6)
                    .fill(isActive
                        ? Color(nsColor: .selectedContentBackgroundColor).opacity(0.2)
                        : Color.clear)
            )
            .contentShape(Rectangle())
        }
        .buttonStyle(.plain)
        .onHover { hovering in
            isHovering = hovering
        }
    }
}
