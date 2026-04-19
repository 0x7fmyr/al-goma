# al-goma — Albin's Grocery Machine

A terminal-based meal planner and grocery list generator. Tell it what dishes you know how to cook, and it'll randomly pick a weekly menu and build a shopping list for you, sorted by category so you can actually navigate a grocery store like a human.

Built in Rust with [ratatui](https://github.com/ratatui/ratatui).

![demo](https://github.com/user-attachments/assets/f9eb4657-c074-4850-a060-b8bf82fd869e)

---

## What it does

- Keeps a "Dishtabase" of your dishes and their ingredients
- Randomly generates a menu from your dishes (you pick how many)
- Builds a shopping list from the menu
- Categorizes ingredients so your list is sorted by grocery store section
- Exports the shopping list to a `.txt` file with optional numbering and category grouping
- Supports English and Swedish (set in `.config/settings.toml`)

---

## Building and running

You'll need Rust installed. If you don't have it: [rustup.rs](https://rustup.rs)

```bash
git clone https://github.com/0x7fmyr/al-goma
cd al-goma
cargo run --release
```

Requires a terminal that's at least 75×20.

---

## Controls

### Main menu
| Key | Action |
|-----|--------|
| `↑ / ↓` | Navigate |
| `Enter` | Select |
| `q` | Quit |

### New list / input fields
| Key | Action |
|-----|--------|
| Type | Enter text |
| `Enter` | Confirm |
| `Backspace` | Delete character |
| `Del` | Remove last ingredient |
| `Ctrl+S` | Save dish to Dishtabase |
| `Esc` | Cancel |

### Generated list
| Key | Action |
|-----|--------|
| `Enter` | Accept list → go to shopping list |
| `Del` | Re-roll a single dish |
| `Esc` | Cancel |

### Dishtabase — dish list
| Key | Action |
|-----|--------|
| `↑ / ↓` | Select dish |
| `Enter` | Open dish |
| `Del` | Delete dish (with confirmation) |
| `Esc` | Back |

### Dishtabase — editing a dish
| Key | Action |
|-----|--------|
| `↑ / ↓` | Select ingredient |
| `Enter` | Edit selected ingredient |
| `Ctrl+N` | Rename dish |
| `Ctrl+A` | Add ingredient |
| `Ctrl+K` | Change ingredient category |
| `Del` | Remove selected ingredient |
| `Esc` | Back |

### Shopping list
| Key | Action |
|-----|--------|
| `↑ / ↓` | Navigate |
| `Del` | Remove item |
| `Ctrl+A` | Add item |
| `Ctrl+P` | Export options |
| `Esc` | Back |

### Export options
| Key | Action |
|-----|--------|
| `↑ / ↓` | Select option |
| `Enter` | Toggle option (numbers / categories) |
| `p` | Write `.txt` file |
| `Esc` | Cancel |

---

## Language

The UI language is set in `.config/settings.toml`:

```toml
language = "Eng"   # or "Swe"
```

The file is created automatically on first run (defaults to English).

---

## Data

All data is stored in `.config/` inside the project directory:

- `.config/dishes.toml` — your Dishtabase
- `.config/sh_list.toml` — your current shopping list
- `.config/settings.toml` — language setting

These are plain TOML files and can be edited by hand. **Note:** data is stored relative to wherever you run the binary from, so always run it from the same directory. Exported shopping lists are saved as `Shopping-Lists/Shopping_List-YYYY-MM-DD.txt` (subsequent exports the same day get a numbered suffix).

---

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your option.
