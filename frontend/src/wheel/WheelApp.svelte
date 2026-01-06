<script lang="ts">
  import "./wheel.css";
  import { listItems, openItem } from "../shared/api";
  import { truncate } from "../shared/utils";
  import type { Item } from "../shared/types";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";

  let items: Item[] = [];
  let selected: Item | null = null;

  const appWindow = getCurrentWindow();

  async function closeWindow() {
    await appWindow.hide();
  }

  async function refresh() {
    items = await listItems(16, true);
    selected = items[0] ?? null;
  }

  async function openSelected(item: Item) {
    await openItem(item.id);
    await closeWindow();
  }

  function handleKey(event: KeyboardEvent) {
    if (event.key === "Escape") {
      closeWindow();
    }
  }

  onMount(() => {
    refresh();
  });
</script>

<svelte:window on:keydown={handleKey} />

<div class="wheel-overlay">
  <div class="wheel">
    {#each items as item, index}
      <button
        class="segment"
        style={`--i: ${index}; --total: ${items.length || 1};`}
        on:mouseenter={() => (selected = item)}
        on:click={() => openSelected(item)}
      >
        {#if item.thumb_path}
          <img src={item.thumb_path} alt={item.title ?? item.item_type} />
        {:else}
          <span class="icon">{item.item_type.toUpperCase()}</span>
        {/if}
      </button>
    {/each}
    <div class="hub">
      <h2>{selected?.title ?? selected?.item_type ?? "No items"}</h2>
      <p>{selected ? truncate(selected.payload) : ""}</p>
    </div>
  </div>
  <p class="hint">Click an item to open. ESC to close.</p>
</div>
