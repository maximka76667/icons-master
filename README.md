# icons-master

A CLI tool to manage [Lucide React](https://lucide.dev) icon exports in your TypeScript project.

Automatically fetches icon metadata, determines the correct category, and keeps your export files alphabetically sorted.

## Installation

```bash
npm install @maximka76667/icons-master
```

> If you install it globally with `-g` flag you could run it without `npx`

The correct binary for your platform is downloaded automatically on first run.

## Usage

### Add an icon

```bash
npx icons-master add <icon-name> [folder]
```

### Remove an icon

```bash
npx icons-master remove <icon-name> [folder]
```

- `<icon-name>` — icon name in kebab-case (e.g. `arrow-up`, `chevron-down`)
- `[folder]` — output directory for TypeScript files (default: `icons`)

## Examples

```bash
npx icons-master add arrow-up
# → Added ArrowUp to icons/arrows.ts

npx icons-master add arrow-up src/icons
# → Added ArrowUp to src/icons/arrows.ts

npx icons-master remove arrow-up
# → Removed ArrowUp from icons/arrows.ts
```

The generated TypeScript files look like this:

```ts
export { ArrowDown, ArrowLeft, ArrowRight, ArrowUp } from "lucide-react";
```

## Supported Platforms

| Platform | Architecture          |
| -------- | --------------------- |
| Windows  | x64                   |
| macOS    | x64 (Intel)           |
| macOS    | ARM64 (Apple Silicon) |
| Linux    | x64                   |
| Linux    | ARM64                 |
