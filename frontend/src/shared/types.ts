export type ItemType = "text" | "url" | "file" | "image";

export interface Item {
  id: string;
  created_at: number;
  item_type: ItemType;
  title?: string | null;
  payload: string;
  thumb_path?: string | null;
  pinned: number;
}

export interface ClipboardSnapshot {
  kind: "empty" | "text" | "image";
  text?: string;
}
