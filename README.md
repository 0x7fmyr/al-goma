# al-goma — Albin's Grocery Machine

A terminal-based meal planner and grocery list generator. Tell it what dishes you know how to cook, and it'll randomly pick a weekly menu and build a shopping list for you — sorted by category so you can actually navigate a grocery store like a human.

Built in Rust with [ratatui](https://github.com/ratatui/ratatui).

![demo placeholder](demo.gif)

---

## What it does

- Keeps a database of your dishes and their ingredients
- Randomly generates a weekly menu from your dishes
- Builds a shopping list from the menu, deduplicating ingredients automatically
- Categorizes ingredients (vegetables, dairy, protein, dry goods, etc.) so your list is sorted by grocery store section
- Lets you manually add or remove items from the shopping list
- Exports the shopping list to a `.txt` file with optional numbering and category grouping

---

## Building and running

You'll need Rust installed. If you don't have it: [rustup.rs](https://rustup.rs)

```bash
git clone https://github.com/0x7fmyr/al-goma
cd al-goma
cargo run --release
```

Requires a terminal that's at least 75x20.

---

## Controls

### Main menu
| Key | Action |
|-----|--------|
| `↑ / ↓` | Navigate |
| `Enter` | Select |
| `q` | Quit |

### Dish database
| Key | Action |
|-----|--------|
| `↑ / ↓` | Select dish |
| `Enter` | Edit dish |
| `Del` | Delete dish |
| `Ctrl+N` | Rename dish |
| `Ctrl+A` | Add ingredient |
| `Ctrl+K` | Change ingredient category |
| `Ctrl+S` | Save dish |
| `Esc` | Back |

### Shopping list
| Key | Action |
|-----|--------|
| `↑ / ↓` | Navigate |
| `Del` | Remove item |
| `Ctrl+A` | Add item |
| `Ctrl+P` | Export to txt |
| `Esc` | Back |

---

## Data

All data is saved in `.config/` inside the project directory:

- `.config/dishes.toml` — your dish database
- `.config/list.toml` — the current generated menu
- `.config/sh_list.toml` — your shopping list

These are plain TOML files so you can edit them by hand if you want. Exported shopping lists are saved as `Shopping_List-YYYY-MM-DD.txt` in the project root.

---

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your option.
