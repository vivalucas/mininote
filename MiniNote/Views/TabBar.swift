import SwiftUI

struct TabBar: View {
    @Binding var documents: [Document]
    @Binding var activeTabId: UUID
    var onNewTab: () -> Void
    var onCloseTab: (Document) -> Void
    var onSelectTab: (UUID) -> Void
    var appLanguage: String
    var onReorder: (IndexSet, Int) -> Void

    @State private var draggedTab: Document? = nil

    var body: some View {
        ScrollView(.horizontal, showsIndicators: false) {
            HStack(spacing: 0) {
                ForEach(documents) { doc in
                    TabButton(
                        document: doc,
                        isActive: doc.id == activeTabId,
                        onSelect: { onSelectTab(doc.id) },
                        onClose: { onCloseTab(doc) },
                        appLanguage: appLanguage
                    )
                    .onDrag {
                        draggedTab = doc
                        return NSItemProvider(object: doc.id.uuidString as NSString)
                    }
                    .onDrop(
                        of: [.text],
                        delegate: TabDropDelegate(
                            item: doc,
                            documents: documents,
                            draggedTab: $draggedTab,
                            onReorder: onReorder
                        )
                    )
                }

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

// MARK: - Drop Delegate

private struct TabDropDelegate: DropDelegate {
    let item: Document
    let documents: [Document]
    @Binding var draggedTab: Document?
    let onReorder: (IndexSet, Int) -> Void

    func performDrop(info: DropInfo) -> Bool {
        draggedTab = nil
        return true
    }

    func dropEntered(info: DropInfo) {
        guard let dragged = draggedTab,
              dragged.id != item.id,
              let fromIndex = documents.firstIndex(where: { $0.id == dragged.id }),
              let toIndex = documents.firstIndex(where: { $0.id == item.id })
        else { return }

        if fromIndex != toIndex {
            onReorder(IndexSet(integer: fromIndex), toIndex > fromIndex ? toIndex + 1 : toIndex)
        }
    }

    func dropUpdated(info: DropInfo) -> DropProposal? {
        DropProposal(operation: .move)
    }
}

// MARK: - Tab Button

private struct TabButton: View {
    let document: Document
    let isActive: Bool
    let onSelect: () -> Void
    let onClose: () -> Void
    let appLanguage: String

    @State private var isHovering = false

    var body: some View {
        Button(action: onSelect) {
            HStack(spacing: 6) {
                Image(systemName: document.isScratch ? "doc" : "doc.fill")
                    .font(.system(size: 11))

                Text(document.displayName(language: appLanguage))
                    .font(.system(size: 12))
                    .lineLimit(1)

                if document.isModified {
                    Circle()
                        .fill(Color.gray)
                        .frame(width: 6, height: 6)
                }

                Button(action: onClose) {
                    Image(systemName: "xmark")
                        .font(.system(size: 9, weight: .bold))
                        .frame(width: 16, height: 16)
                        .contentShape(Rectangle())
                }
                .buttonStyle(.plain)
                .opacity(isHovering || isActive ? 1 : 0)
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
