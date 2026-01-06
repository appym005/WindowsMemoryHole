import { invoke } from "@tauri-apps/api/core";
import type { ClipboardSnapshot, Item } from "./types";

export async function addItem(item_type: string, payload: string, title?: string) {
  return invoke<{ id: string }>("add_item", { item_type, payload, title });
}

export async function listItems(limit = 16, include_pinned = true) {
  return invoke<Item[]>("list_items", { limit, include_pinned });
}

export async function openItem(id: string) {
  return invoke<{ ok: true }>("open_item", { id });
}

export async function deleteItem(id: string) {
  return invoke<{ ok: true }>("delete_item", { id });
}

export async function clipboardSnapshot() {
  return invoke<ClipboardSnapshot>("clipboard_snapshot");
}

export async function saveClipboardImage() {
  return invoke<{ path: string; thumb_path?: string }>("save_clipboard_image");
}
