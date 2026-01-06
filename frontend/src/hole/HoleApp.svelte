<script lang="ts">
  import "./hole.css";
  import { addItem, clipboardSnapshot, saveClipboardImage } from "../shared/api";
  import { looksLikeUrl } from "../shared/utils";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  let value = "";
  let status = "";

  const appWindow = getCurrentWindow();

  async function closeWindow() {
    await appWindow.hide();
  }

  async function save() {
    const trimmed = value.trim();
    if (!trimmed) {
      status = "Nothing to save.";
      return;
    }
    const itemType = looksLikeUrl(trimmed) ? "url" : "text";
    await addItem(itemType, trimmed);
    value = "";
    status = "Saved.";
    await closeWindow();
  }

  async function grabClipboard() {
    const snapshot = await clipboardSnapshot();
    if (snapshot.kind === "text" && snapshot.text) {
      value = snapshot.text;
      status = "Clipboard text loaded.";
      return;
    }
    if (snapshot.kind === "image") {
      const { path } = await saveClipboardImage();
      await addItem("image", path, "Clipboard Image");
      status = "Clipboard image saved.";
      await closeWindow();
      return;
    }
    status = "Clipboard empty.";
  }

  function handleKey(event: KeyboardEvent) {
    if (event.key === "Escape") {
      closeWindow();
    }
  }

  async function handleDrop(event: DragEvent) {
    event.preventDefault();
    const files = Array.from(event.dataTransfer?.files ?? []);
    if (!files.length) {
      return;
    }
    for (const file of files) {
      const name = file.name || "Dropped File";
      const path = (file as File & { path?: string }).path ?? "";
      if (!path) {
        continue;
      }
      const isImage = file.type.startsWith("image/");
      await addItem(isImage ? "image" : "file", path, name);
    }
    status = `Saved ${files.length} item(s).`;
    await closeWindow();
  }

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
  }
</script>

<svelte:window on:keydown={handleKey} on:drop={handleDrop} on:dragover={handleDragOver} />

<div class="hole-shell">
  <div class="swirl" aria-hidden="true"></div>
  <h1>BlackHole</h1>
  <textarea bind:value placeholder="Drop text, URL, or file…"></textarea>
  <div class="actions">
    <button class="primary" on:click={save}>Save</button>
    <button on:click={grabClipboard}>Grab Clipboard</button>
    <button on:click={closeWindow}>Close</button>
  </div>
  <p class="status">{status}</p>
</div>
