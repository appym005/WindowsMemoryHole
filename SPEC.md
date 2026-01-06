# SPEC.md — BlackHole Wheel (Windows MVP)

## 0) Goal (MVP)
Build a lightweight Windows desktop app (no Unity) that lets the user:

1) Press a **global hotkey** to open a small **“Hole” overlay** at the cursor to ingest:
- text / notes
- URLs
- files (drag-drop)
- images (drag-drop or clipboard image)

2) Press a **global hotkey** to open a fullscreen **“Weapon Wheel” overlay** that shows recently ingested items and lets the user open them.

**No Chrome integration in MVP.** (Only leave a clean hook for V2.)

---

## 1) Constraints
- Windows only
- Tauri v2 + Rust backend
- Vite + Svelte frontend
- Local persistence via SQLite in `%APPDATA%`
- Smooth UI animations are desired, but correctness + reliability beats perfect visuals in MVP

---

## 2) Repo Structure (required)
```
blackhole-wheel/
  SPEC.md
  frontend/
    index.html
    package.json
    vite.config.ts
    src/
      main.ts
      shared/
        api.ts
        types.ts
        utils.ts
      hole/
        HoleApp.svelte
        hole.ts
        hole.css
      wheel/
        WheelApp.svelte
        wheel.ts
        wheel.css
      assets/
        icons/
  src-tauri/
    Cargo.toml
    tauri.conf.json
    src/
      main.rs
      hotkeys.rs
      windows.rs
      db.rs
      ingest.rs
      open.rs
      models.rs
      paths.rs
      errors.rs
      tests/
        db_tests.rs
```

---

## 3) Runtime Files (AppData)
Store everything under:
`%APPDATA%/BlackHoleWheel/`

```
BlackHoleWheel/
  db.sqlite
  thumbs/
  images/
  config.json   (optional)
```

---

## 4) Data Model

### 4.1 ItemType
Allowed types:
- `text`
- `url`
- `file`
- `image`

### 4.2 SQLite schema (exact)
Run at startup if missing:

```sql
CREATE TABLE IF NOT EXISTS items (
  id TEXT PRIMARY KEY,
  created_at INTEGER NOT NULL,
  item_type TEXT NOT NULL,
  title TEXT,
  payload TEXT NOT NULL,
  thumb_path TEXT,
  pinned INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_items_created_at ON items(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_items_pinned ON items(pinned DESC);
```

Notes:
- `id`: UUID string
- `created_at`: unix epoch seconds
- `payload`:
  - text: raw text
  - url: full URL
  - file: absolute file path
  - image: absolute path to saved image in `images/`
- `thumb_path`: absolute path to a generated thumbnail in `thumbs/` (optional; recommended for images)

---

## 5) Windows / Overlay Behavior

### 5.1 Required windows
Two overlay windows; a “main” window may exist but should remain hidden/minimized if Tauri requires it.

#### Hole window
- name: `hole`
- size: ~420x220
- frameless
- transparent background
- always-on-top
- appears near mouse cursor (offset so it doesn’t cover cursor)
- closes on:
  - ESC
  - successful save
  - (optional) loss of focus, only if it doesn’t harm usability

#### Wheel window
- name: `wheel`
- fullscreen on primary monitor
- frameless
- always-on-top
- dim overlay background (fake blur ok)
- closes on:
  - ESC
  - (optional) click outside wheel

### 5.2 Never show both overlays simultaneously
At any time, at most one of `hole` or `wheel` can be visible.

---

## 6) Global Hotkeys (required)
Register system-wide hotkeys:

- `Ctrl+Shift+Space` → toggle Hole overlay
- `Ctrl+Shift+E` → toggle Wheel overlay

### 6.1 Toggle rules (exact)
- If Hole is open and Hole hotkey pressed again: **close Hole**
- If Wheel is open and Wheel hotkey pressed again: **close Wheel**
- If Hole open and Wheel hotkey pressed: **close Hole then open Wheel**
- If Wheel open and Hole hotkey pressed: **close Wheel then open Hole**
- Do not allow double-trigger races (debounce or state lock)

---

## 7) Tauri Commands (exact signatures)
Expose these commands to frontend.

### 7.1 add_item
Input:
- `item_type: string` (`text|url|file|image`)
- `payload: string`
- `title?: string`

Output:
- `{ id: string }`

### 7.2 list_items
Input:
- `limit: number` (default 16)
- `include_pinned: boolean` (default true)

Output:
- `Item[]` sorted:
  - pinned desc
  - created_at desc

### 7.3 open_item
Input:
- `id: string`

Output:
- `{ ok: true }`

### 7.4 delete_item (optional but recommended)
Input:
- `id: string`

Output:
- `{ ok: true }`

### 7.5 clipboard_snapshot (required)
Output:
- `{ kind: "empty" | "text" | "image", text?: string }`

If image exists, backend must support:

### 7.6 save_clipboard_image (required)
Output:
- `{ path: string, thumb_path?: string }`

---

## 8) Backend Modules Responsibilities

### paths.rs
- Resolve base app dir (`%APPDATA%/BlackHoleWheel`)
- Ensure `thumbs/` and `images/` exist
- Resolve DB path

### db.rs
- Init DB + migrations
- CRUD:
  - insert item
  - list recent items (pinned + recent)
  - get item by id
  - delete item (also delete internal thumb/image files if stored internally)

### ingest.rs
- URL detection (simple regex, conservative)
- Validate file paths exist (for `file` type)
- Save clipboard image to `images/`
- Thumbnail generation (recommended for images):
  - resize max 256x256
  - save PNG or WebP into `thumbs/`

### open.rs
Open based on type:
- url → default browser (ShellExecute)
- file/image → default app (ShellExecute)
- text → copy to clipboard (and return ok)

### windows.rs
- Create/configure windows
- Show/hide windows
- Position hole near cursor
- Fullscreen wheel on primary monitor

### hotkeys.rs
- Register global hotkeys
- Implement toggle rules and debouncing/state lock

---

## 9) Frontend UI Specs

### 9.1 Hole UI (HoleApp.svelte)
Visual:
- dark translucent background
- simple animated “swirl” (Canvas particles OR CSS animation; MVP doesn’t need perfect black-hole shader)

Controls:
- multiline text input
- buttons:
  - Save (auto-detect URL)
  - Grab Clipboard
  - Close

Drag-drop:
- dropping one or more files:
  - create `file` items immediately
  - title = filename
- if dropped file is an image:
  - create `image` item (payload = path)
  - (optional) request backend to thumb it later; MVP can skip thumbs for dropped images if needed

Auto-detect rule:
- if input matches URL regex → save as `url`
- else save as `text`

### 9.2 Wheel UI (WheelApp.svelte)
On open:
- call `list_items(limit=16, include_pinned=true)`

Display:
- fullscreen overlay
- dim background (fake blur ok)
- radial menu around center hub
- each segment shows:
  - thumbnail if available
  - else type icon

Interaction:
- hover highlights selection and shows center preview (title/type)
- click → `open_item(id)` then close wheel
- ESC closes wheel

---

## 10) Tests (required)
Backend unit tests:
- DB init creates tables
- insert/list ordering works
- delete removes item

Hotkeys/window behavior can be manually tested for MVP.

---

## 11) Done Criteria (MVP)
MVP is complete when:
- Hotkeys work globally and overlays toggle correctly
- Hole accepts: typed text, pasted URL, dropped file(s), clipboard text, clipboard image
- Items persist to SQLite and are visible in wheel
- Clicking wheel items opens correctly (URL/file/image) and text copies to clipboard
- ESC closes overlays reliably
