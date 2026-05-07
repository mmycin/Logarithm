# Logarithm

<div align="center">

**A modern, blazing-fast log file viewer built with Tauri and Leptos**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app/)
[![Leptos](https://img.shields.io/badge/Leptos-0.7-purple.svg)](https://leptos.dev/)

[Features](#features) • [Installation](#installation) • [Usage](#usage) • [Development](#development) • [Architecture](#architecture) • [License](#license)

</div>

---

## Overview

Logarithm is a powerful desktop application designed for developers who need to quickly inspect, filter, and analyze log files. Built with modern web technologies and Rust, it offers exceptional performance and a beautiful, intuitive interface.

### Why Logarithm?

- **🚀 Blazing Fast**: Native performance with Rust backend
- **🎨 Beautiful UI**: Modern, clean interface with dark/light themes
- **🔍 Advanced Filtering**: Level, search, datetime, line range, and severity filters
- **🤖 AI-Powered**: Integrated AI assistant (Logan) for log analysis
- **⚡ Real-time**: Instant filtering and search with fuzzy matching
- **🎯 Smart Parsing**: Automatic detection of timestamps, levels, and messages
- **💾 Lightweight**: Small footprint, minimal resource usage
- **🔒 Privacy-First**: All processing happens locally on your machine

---

## Features

### Core Features

#### 📂 **Log File Management**
- Open and view multiple log files simultaneously
- Tab-based interface for easy switching between files
- Automatic parsing of common log formats
- Support for large files (10k+ lines)
- Drag-and-drop file opening

#### 🔍 **Advanced Filtering**
- **Level Filtering**: Filter by log level (TRACE, DEBUG, INFO, WARN, ERROR, FATAL)
- **Custom Levels**: Define your own custom level filters
- **Search**: Full-text search with multiple modes:
  - Case-sensitive matching
  - Fuzzy search
  - Regex support
  - Invert match
  - Search in timestamps
- **Date/Time Range**: Filter logs by datetime range
- **Line Range**: Filter by specific line numbers
- **Severity**: Minimum severity level filtering
- **Advanced Options**: Hide lines without levels, inherit levels for continuation lines

#### 🎨 **Visual Features**
- **Syntax Highlighting**: Color-coded log levels with badges
- **Continuation Lines**: Visual distinction for multi-line log entries
- **Line Grouping**: Automatic grouping of related log lines
- **Focus Mode**: Click to focus on specific log lines
- **Multi-Select**: Ctrl+Click to select multiple lines
- **Context Menu**: Right-click for quick actions

#### 🤖 **Logan AI Assistant**
- **Integrated AI Chat**: Built-in AI assistant for log analysis
- **Multiple Providers**: Support for OpenAI, Anthropic, Google Gemini, and Groq
- **Context Chips**: Add specific log lines as context for AI queries
- **File Mentions**: Reference entire log files with @ mentions
- **Smart Actions**:
  - Add to context: Include log lines in AI conversation
  - Explain: Get AI explanation of specific log entries
  - Copy: Copy log lines to clipboard
- **Markdown Support**: Rich formatting in AI responses

#### ⚙️ **Customization**
- **Themes**: Dark and light mode support
- **Resizable Panels**: Adjust filter panel and AI chat width
- **Keyboard Shortcuts**: Efficient keyboard navigation
- **Persistent Settings**: Remembers your preferences

---

## Installation

### Prerequisites

- **Node.js** (v18 or higher)
- **Rust** (1.70 or higher)
- **bun** (recommended) or npm

### Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/mmycin/Logarithm.git
   cd Logarithm
   ```

2. **Install dependencies**
   ```bash
   bun install
   cargo install
   ```

3. **Run in development mode**
   ```bash
   cargo tauri dev
   ```

4. **Build for production**
   ```bash
   cargo tauri build
   ```

### Platform-Specific Builds

#### Windows
```bash
cargo tauri build --target x86_64-pc-windows-msvc
```

#### macOS
```bash
cargo tauri build --target x86_64-apple-darwin
# For Apple Silicon
cargo tauri build --target aarch64-apple-darwin
```

#### Linux
```bash
cargo tauri build --target x86_64-unknown-linux-gnu
```

---

## Usage

### Opening Log Files

1. **File Menu**: Click `File → Open` or press `Ctrl+O`
2. **Drag & Drop**: Drag a `.log` file into the window
3. **Recent Files**: Access recently opened files from the File menu

### Filtering Logs

#### Level Filter
- Click on level badges (ALL, TRACE, DEBUG, INFO, WARN, ERROR, FATAL)
- Use "Custom" to define comma-separated custom levels
- Toggle "Inherit" to apply levels to continuation lines

#### Search
- Enter search query in the search box
- Toggle options:
  - **Aa**: Case-sensitive matching
  - **.***: Fuzzy search
  - **Re**: Regex mode
  - **¬**: Invert match
- Enable "Include timestamp" to search in datetime fields

#### Date & Time Range
- Click "Date & Time" section
- Set "From" and "To" datetime values
- Click "Clear date range" to reset

#### Line Range
- Enter start line in "From" field
- Enter end line in "To" field
- Leave empty for unbounded range

### Using Logan AI

1. **Setup**
   - Click the AI chat icon in the bottom bar
   - Select your AI provider (OpenAI, Anthropic, Gemini, Groq)
   - Enter your API key
   - Choose a model (or use default)
   - Click "Start Chatting"

2. **Adding Context**
   - Right-click on a log line → "Add to Logan context"
   - Select multiple lines (Ctrl+Click) → Right-click → "Add to Logan context"
   - Context chips appear in the input area

3. **Mentioning Files**
   - Type `@` in the chat input
   - Select a file from the autocomplete list
   - The entire file content will be included as context

4. **Quick Actions**
   - Right-click on a log line → "Explain with Logan AI"
   - Logan will automatically analyze the log entry

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+O` | Open log file |
| `Ctrl+W` | Close active tab |
| `Ctrl+F` | Focus search box |
| `Ctrl+T` | Toggle theme (dark/light) |
| `Ctrl+B` | Toggle filter panel |
| `Ctrl+L` | Toggle AI chat panel |
| `Ctrl+/` | Show keyboard shortcuts |
| `Ctrl+Click` | Multi-select log lines |

---

## Development

### Project Structure

```
Logarithm/
├── src/                          # Frontend (Leptos/Rust)
│   ├── ai/                       # AI assistant module
│   │   ├── components/           # UI components
│   │   ├── handlers/             # Event handlers
│   │   ├── state/                # State management
│   │   └── types/                # Type definitions
│   ├── components/               # Core UI components
│   │   ├── bottom_bar.rs
│   │   ├── file_bar.rs
│   │   ├── filter_panel.rs
│   │   ├── filter_section.rs
│   │   ├── filter_tab.rs
│   │   ├── severity.rs
│   │   └── title_bar.rs
│   ├── filters/                  # Filter components
│   ├── markdown/                 # Markdown rendering
│   │   ├── colors.rs
│   │   ├── escape.rs
│   │   ├── inline.rs
│   │   └── parser.rs
│   ├── shared/                   # Shared utilities
│   │   ├── constants/            # Color tokens
│   │   ├── storage/              # LocalStorage helpers
│   │   ├── types/                # Shared types
│   │   └── utils/                # Utility functions
│   ├── viewer/                   # Log viewer module
│   │   ├── components/           # Viewer components
│   │   ├── filters/              # Filter logic
│   │   ├── rendering/            # Rendering utilities
│   │   └── types/                # Viewer types
│   ├── app.rs                    # Main app component
│   └── main.rs                   # Entry point
├── src-tauri/                    # Backend (Tauri/Rust)
│   └── src/
│       ├── ai/                   # AI integration
│       │   ├── gemini.rs         # Google Gemini
│       │   └── openai.rs         # OpenAI
│       ├── commands/             # Tauri commands
│       │   ├── file_commands.rs  # File operations
│       │   └── log_commands.rs   # Log parsing/filtering
│       ├── types/                # Type definitions
│       │   ├── ai_types.rs
│       │   └── log_types.rs
│       ├── lib.rs                # Library entry
│       └── main.rs               # Binary entry
├── public/                       # Static assets
├── index.html                    # HTML template
├── package.json                  # Node dependencies
├── Cargo.toml                    # Rust dependencies
└── README.md                     # This file
```

### Technology Stack

#### Frontend
- **Leptos**: Reactive UI framework for Rust
- **WebAssembly**: Compiled Rust running in the browser
- **Tailwind CSS**: Utility-first CSS framework
- **Fira Code**: Monospace font for log display

#### Backend
- **Tauri**: Desktop application framework
- **Rust**: Systems programming language
- **Tokio**: Async runtime
- **Reqwest**: HTTP client for AI APIs
- **Serde**: Serialization/deserialization

### Building from Source

1. **Install Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Node.js and bun**
   ```bash
   # Install Node.js from https://nodejs.org/
   npm install -g bun
   ```

3. **Install Tauri CLI**
   ```bash
   cargo install tauri-cli
   ```

4. **Clone and build**
   ```bash
   git clone https://github.com/mmycin/Logarithm.git
   cd Logarithm
   bun install
   bun tauri build
   ```

### Development Commands

```bash
# Run in development mode with hot reload
bun tauri dev

# Build for production
bun tauri build

# Run frontend only (for UI development)
trunk serve

# Check Rust code (frontend)
cargo check

# Check Rust code (backend)
cargo check --manifest-path src-tauri/Cargo.toml

# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets --all-features
```

---

## Architecture

### Frontend Architecture

Logarithm uses a modular, component-based architecture with clear separation of concerns:

#### **Module Organization**
- **Types**: Data structures and type definitions
- **State**: State management with Leptos signals
- **Handlers**: Event handlers and business logic
- **Components**: UI components (presentational)

#### **Key Design Patterns**
- **Single Responsibility**: Each module has one clear purpose
- **DRY (Don't Repeat Yourself)**: No code duplication
- **KISS (Keep It Simple, Stupid)**: Simple, clear logic
- **Reactive Programming**: Leptos signals for state management
- **Type Safety**: Strong typing throughout

### Backend Architecture

The Tauri backend is organized into focused modules:

#### **Commands**
- `parse_log`: Parse log file text into structured entries
- `filter_entries`: Apply filters to log entries
- `open_url`: Open URLs in system browser
- `read_file_by_path`: Read file contents
- `ai_chat`: Send messages to AI providers

#### **AI Integration**
- Modular provider system (Gemini, OpenAI)
- 30-second timeout protection
- Automatic retry logic
- Error handling and reporting

### Data Flow

```
User Input → Leptos Component → Signal Update → Reactive Re-render
                                      ↓
                              Tauri Command (if needed)
                                      ↓
                              Rust Backend Processing
                                      ↓
                              Return Result
                                      ↓
                              Update Signal → Re-render
```

---

## Configuration

### AI Provider Setup

#### OpenAI
1. Get API key from https://platform.openai.com/api-keys
2. Select provider: "OpenAI"
3. Enter API key
4. Choose model (default: gpt-4o-mini)

#### Google Gemini
1. Get API key from https://makersuite.google.com/app/apikey
2. Select provider: "Gemini"
3. Enter API key
4. Choose model (default: gemini-1.5-flash)

### Settings Storage

Settings are stored locally using browser LocalStorage:
- AI provider and model preferences
- API keys (encrypted in browser storage)
- Theme preference
- Panel sizes

---

## Contributing

Contributions are welcome! Please follow these guidelines:

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Commit your changes** (`git commit -m 'Add amazing feature'`)
4. **Push to the branch** (`git push origin feature/amazing-feature`)
5. **Open a Pull Request**

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Run clippy and fix warnings (`cargo clippy`)
- Write clear commit messages
- Add tests for new features
- Update documentation

### Reporting Issues

Please use GitHub Issues to report bugs or request features. Include:
- Clear description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Screenshots (if applicable)
- System information (OS, version)

---

## Roadmap

### Planned Features

- [ ] **Export Functionality**: Export filtered logs to file
- [ ] **Log Streaming**: Real-time log file monitoring
- [ ] **Custom Parsers**: User-defined log format parsers
- [ ] **Bookmarks**: Save and recall filter configurations
- [ ] **Search History**: Recent search queries
- [ ] **Performance Metrics**: Display parsing and filtering stats
- [ ] **Plugin System**: Extensible architecture for custom features
- [ ] **Cloud Sync**: Sync settings across devices
- [ ] **Collaborative Features**: Share log analysis with team
- [ ] **Advanced AI Features**: 
  - Pattern detection
  - Anomaly detection
  - Automated insights
  - Log summarization

---

## FAQ

### Q: What log formats are supported?
**A:** Logarithm automatically detects common log formats with timestamps and log levels. It works with most standard formats including:
- ISO 8601 timestamps
- Space-separated fields
- Common log levels (TRACE, DEBUG, INFO, WARN, ERROR, FATAL)

### Q: How large of a log file can Logarithm handle?
**A:** Logarithm is optimized for files with 10,000+ lines. Performance depends on your system, but it handles most typical log files efficiently.

### Q: Is my data sent to external servers?
**A:** No. All log parsing and filtering happens locally on your machine. Only AI chat messages are sent to the selected AI provider (OpenAI, Anthropic, etc.) when you explicitly use the AI features.

### Q: Can I use Logarithm without an AI API key?
**A:** Yes! All core features (log viewing, filtering, searching) work without AI. The AI assistant is an optional enhancement.

### Q: How do I update Logarithm?
**A:** Download the latest release from GitHub and install it. Your settings will be preserved.

### Q: Does Logarithm support log streaming?
**A:** Not yet, but it's on the roadmap! Currently, you need to reload the file to see new entries.

---

## Troubleshooting

### Build Issues

**Problem**: `cargo build` fails with dependency errors  
**Solution**: Update Rust toolchain: `rustup update`

**Problem**: `bun install` fails  
**Solution**: Clear cache: `bun store prune` then retry

**Problem**: Tauri build fails on Windows  
**Solution**: Install Visual Studio Build Tools with C++ support

### Runtime Issues

**Problem**: Log file won't open  
**Solution**: Ensure file has `.log` extension and is readable

**Problem**: AI chat not working  
**Solution**: Verify API key is correct and you have internet connection

**Problem**: Filters not applying  
**Solution**: Check filter syntax and ensure "Clear all" wasn't clicked

---

## Performance Tips

1. **Large Files**: For very large files (100k+ lines), use filters to reduce visible entries
2. **Search**: Use specific search terms rather than broad queries
3. **AI Context**: Limit context chips to relevant lines (5-10 max)
4. **Multiple Files**: Close unused tabs to free memory

---

## Security

### Data Privacy
- All log processing happens locally
- API keys stored in browser LocalStorage (not transmitted)
- No telemetry or analytics
- No external dependencies for core features

### API Key Safety
- Keys are stored locally only
- Never logged or transmitted except to chosen AI provider
- Use environment variables for development
- Rotate keys regularly

---

## Acknowledgments

Built with amazing open-source technologies:
- [Tauri](https://tauri.app/) - Desktop application framework
- [Leptos](https://leptos.dev/) - Reactive UI framework
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Trunk](https://trunkrs.dev/) - WASM build tool
- [Tailwind CSS](https://tailwindcss.com/) - CSS framework

Special thanks to the Rust and Tauri communities for their excellent documentation and support.

---

## Contact

- **Author**: mmycin
- **Repository**: [github.com/mmycin/Logarithm](https://github.com/mmycin/Logarithm)
- **Issues**: [github.com/mmycin/Logarithm/issues](https://github.com/mmycin/Logarithm/issues)

---

<div align="center">


[⬆ Back to Top](#logarithm)

</div>
