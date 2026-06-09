import { invoke } from "@tauri-apps/api/core";

export function saveImage(noteId: string, data: number[], extension: string): Promise<string> {
  return invoke("images_save", { noteId, data, extension });
}

export function getImagesBaseDir(): Promise<string> {
  return invoke("images_get_base_dir");
}

export function cleanUnusedImages(noteId: string, content: string): Promise<string[]> {
  return invoke("images_clean_unused", { noteId, content });
}
