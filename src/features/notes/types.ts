export interface NoteMetadata {
  id: string;
  title: string;
  fileName: string;
  category: string;
  sourcePath?: string;
  sourceModifiedTime?: number;
  createdAt: string;
  updatedAt: string;
  wordCount: number;
  preview: string;
  tileColor?: string;
  renderMarkdown?: boolean;
}

export interface Note extends Omit<NoteMetadata, "preview"> {
  content: string;
}

export interface SaveNoteRequest {
  title: string;
  content: string;
  category: string;
  sourcePath?: string;
  sourceModifiedTime?: number;
  tileColor?: string;
  renderMarkdown?: boolean;
}

export interface SyncSourceRequest {
  content: string;
  expectedModifiedTime?: number;
  force?: boolean;
}

export interface SourceFileChangedPayload {
  noteId: string;
  title: string;
}
