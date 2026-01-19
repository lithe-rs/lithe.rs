THE FRAMEWORK ARCHITECTURE

Concept
Polyglot Meta Framework w smart compiler as a bridge

backend -> Rust / Go
frontend -> typescript

Example files
Profile.rs -> server logic
Profile.lzr -> client

The build

1 Parsing.
It scans the Rust file and extracts structs and public functions.
2 Server Compilation.
It generates real HTTP API routes for the functions found in the Rust file and compiles the code into a high performance native binary.
3 Client Compilation.
It generates a bridge module in memory. When the frontend calls a Rust function, the compiler replaces that call with a network fetch request to the generated API. It also injects initial data into the HTML for instant page loading and bundles the TypeScript into optimized vanilla JavaScript.

## Error Handling & Diagnostics

The Lithe compiler uses a fault-tolerant state-machine parser. 

1. **Detection**: If a block-level tag (like `<script>` or `<style>`) is opened but not closed before EOF, the parser generates a `Diagnostic` object instead of crashing.
2. **Reporting**:
   - **CLI**: Prints a colored code-frame showing exactly where the tag was opened.
   - **LSP**: The VS Code extension maps these Diagnostics to the "Problems" panel in real-time.
3. **Recovery**: The parser will treat the remainder of the file as part of the unclosed block to allow the IDE to continue providing syntax highlighting for the content already written.
