# TachyonFX FTL

![screenshots](screenshot.png)

## Live Editor

**[Try it now on GitHub Pages](https://junkdog.github.io/tachyonfx-ftl/)**

TachyonFX FTL is a browser-based editor and previewer for [TachyonFX](https://github.com/junkdog/tachyonfx) effects. It allows you to create, visualize, and iterate on terminal UI effects in real-time using TachyonFX's DSL (Domain-Specific Language).

## Features

- **Live Preview**: See your TachyonFX effects come to life as you type
- **Code Editor**: Fully-featured code editing with syntax highlighting and auto-completion powered by [Ace Editor](https://ace.c9.io/)
- **Web Terminal**: Rendered using [Ratzilla](https://github.com/orhun/ratzilla), a terminal UI library for WebAssembly
- **Customizable Canvas**: Define your own ANSI-escaped ASCII as the base for your effects
- **Shareable Effects**: Effects can be shared via URL parameters, making it easy to showcase your creations
- **Split-pane Interface**: Drag to resize the editor and preview areas
- **Gruvbox Theme**: Consistent dark theme throughout the application

## How to Use

1. Visit the [TachyonFX FTL](https://junkdog.github.io/tachyonfx-ftl/) web application
2. Write your TachyonFX effect code in the left panel
3. Customize the canvas input at the bottom if needed
4. Click "Run" or press Ctrl+S/Cmd+S to apply the effect
5. Share your creation by copying the URL

## Development

This project is a [Ratzilla](https://github.com/orhun/ratzilla) app built with Rust and compiled to WebAssembly. It uses:

- **TachyonFX**: For the core effects library and DSL
- **Ratatui**: Terminal UI components
- **Ratzilla**: WebAssembly-compatible terminal rendering
- **Ace Editor**: Code editing capabilities
- **WebAssembly/Trunk**: For building and packaging

To develop locally:

```bash
# Clone the repository
git clone https://github.com/junkdog/tachyonfx-ftl.git
cd tachyonfx-ftl

# Build and serve the application
cd tachyonfx-ftl-web
trunk serve
```

## License

Copyright (c) Adrian Papari <junkdog@angelhill.net>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
