# Pomodoro Dungeon Crawler

> "Defeat your tasks, one dungeon at a time."

A desktop (and soon mobile) application built with Tauri and Astro that turns your todo list into monsters you must defeat with Pomodoro focus sessions.

## 🎮 Mechanics

- Each task is a monster with HP
- Work for 25 minutes (Pomodoros) to deal damage
- Lose focus (alt-tab) and the monster counterattacks
- Gain XP and gold upon completing tasks
- Weekly bosses for large missions

## 🛠️ Stack

- **Frontend**: Astro + Vanilla JS
- **Backend**: Rust (Tauri)
- **Persistence**: SQLite
- **Targets**: Windows, Linux, macOS (and thinking about Android/iOS )

## 🚀 Development

```bash
# Clone
git clone https://github.com/mazmorrdev/pomodoro-dungeon-crawler
cd pomodoro-dungeon-crawler

# Install dependencies
npm install

# Run in development mode
cargo tauri dev

# Build for production
cargo tauri build
```

## 📁 Structure

```
src/
├── components/     # Timer, monster, task list
├── store/          # State (XP, gold, progress)
├── styles/         # Tailwind + custom CSS
└── lib/            # Utilities (Pomodoro, damage calc)
src-tauri/
├── src/            # Rust backend
└── Cargo.toml
```

## 📝 TODO

- [ ] Functional Pomodoro timer
- [ ] Monster system (HP, attack, defense)
- [ ] Focus loss penalty
- [ ] SQLite persistence
- [ ] Shop with skins/armor
- [ ] Weekly boss mode
- [ ] Achievements system
- [ ] Builds for Windows/Linux/macOS

## 📄 License

AGPL-3.0
