# CLAUDE.md - Universal Language Server Plugin

## Project Overview

This is a **Universal Language Server Plugin** project that aims to provide a flexible and extensible language server implementation that can support multiple programming languages through a plugin architecture.

### Purpose

The plugin enables IDE features such as:
- Code completion and IntelliSense
- Go to definition and references
- Diagnostics and linting
- Code formatting and refactoring
- Hover information and documentation
- Syntax highlighting

## Project Structure

```
universal-language-server-plugin/
├── src/                    # Source code
│   ├── core/              # Core language server functionality
│   ├── plugins/           # Language-specific plugin implementations
│   ├── protocol/          # LSP protocol implementation
│   ├── utils/             # Utility functions and helpers
│   └── index.ts           # Main entry point
├── tests/                 # Test files
├── docs/                  # Documentation
├── examples/              # Example configurations and plugins
└── package.json           # Project dependencies and scripts
```

## Development Guidelines

### Technology Stack

- **Language**: TypeScript (for type safety and better tooling)
- **Runtime**: Node.js
- **Protocol**: Language Server Protocol (LSP)
- **Testing**: Jest or Vitest
- **Build**: ESBuild or TSC

### Code Style

- Use TypeScript strict mode
- Follow functional programming principles where appropriate
- Prefer composition over inheritance
- Use descriptive variable and function names
- Write comprehensive JSDoc comments for public APIs
- Keep functions small and focused (single responsibility)

### Key Concepts

1. **Plugin Architecture**: Each language should be implemented as a separate plugin
2. **LSP Compliance**: Strictly follow the Language Server Protocol specification
3. **Async Operations**: Use async/await for all I/O operations
4. **Error Handling**: Implement robust error handling with detailed error messages
5. **Performance**: Optimize for fast response times, especially for completion requests

### Testing

- Write unit tests for core functionality
- Write integration tests for plugin implementations
- Aim for >80% code coverage
- Test edge cases and error conditions
- Mock external dependencies

### Plugin Development

When creating a new language plugin:

1. Implement the `LanguagePlugin` interface
2. Register language-specific features (completion, diagnostics, etc.)
3. Handle language-specific parsing and analysis
4. Provide configuration options
5. Include comprehensive tests

Example plugin structure:
```typescript
interface LanguagePlugin {
  name: string;
  languages: string[];
  initialize(): Promise<void>;
  getCompletions(params: CompletionParams): Promise<CompletionItem[]>;
  getDiagnostics(params: DocumentDiagnosticParams): Promise<Diagnostic[]>;
  // ... other LSP methods
}
```

## Common Tasks

### Adding a New Language Plugin

1. Create a new file in `src/plugins/<language>/`
2. Implement the `LanguagePlugin` interface
3. Register the plugin in the plugin registry
4. Add configuration schema
5. Write tests in `tests/plugins/<language>/`
6. Update documentation

### Implementing LSP Features

- Follow the official LSP specification: https://microsoft.github.io/language-server-protocol/
- Use the `vscode-languageserver` npm package for protocol handling
- Implement capabilities progressively (start with basic features)
- Test with multiple LSP clients (VS Code, Neovim, Emacs)

### Performance Optimization

- Use incremental parsing for document updates
- Implement caching for expensive operations
- Debounce/throttle diagnostic requests
- Profile and benchmark critical code paths
- Consider using worker threads for CPU-intensive tasks

## Dependencies

### Core Dependencies
- `vscode-languageserver`: LSP protocol implementation
- `vscode-languageserver-textdocument`: Document management
- Language-specific parsers (tree-sitter, etc.)

### Development Dependencies
- TypeScript
- Testing framework (Jest/Vitest)
- ESLint for linting
- Prettier for formatting

## Configuration

The plugin should support configuration through:
- `.universal-lsp.json` in project root
- LSP initialization options
- Environment variables

Example configuration:
```json
{
  "plugins": ["javascript", "python", "rust"],
  "diagnostics": {
    "enabled": true,
    "debounce": 500
  },
  "completion": {
    "triggerCharacters": [".", ":", ">"]
  }
}
```

## Architecture Decisions

### Why Plugin-Based?
- Allows incremental language support
- Keeps core lightweight
- Enables community contributions
- Facilitates testing and maintenance

### Why TypeScript?
- Type safety reduces runtime errors
- Better IDE support and autocomplete
- Self-documenting code
- Easier refactoring

### Why LSP?
- Industry standard protocol
- Works with multiple editors
- Well-documented specification
- Large ecosystem of tools

## Error Handling

- Use custom error classes for different error types
- Log errors with appropriate severity levels
- Return meaningful error messages to the client
- Never crash the language server
- Implement graceful degradation

## Security Considerations

- Validate all input from clients
- Sandbox plugin execution if possible
- Limit file system access
- Be cautious with code execution
- Validate configuration files

## Contributing

When contributing:
1. Fork the repository
2. Create a feature branch
3. Write tests for new features
4. Ensure all tests pass
5. Update documentation
6. Submit a pull request

## Resources

- [LSP Specification](https://microsoft.github.io/language-server-protocol/)
- [VS Code Extension API](https://code.visualstudio.com/api)
- [Tree-sitter](https://tree-sitter.github.io/tree-sitter/)
- [Language Server Protocol SDK](https://github.com/Microsoft/language-server-protocol)

## Notes for Claude

- When implementing features, always consider LSP compatibility
- Prioritize performance and responsiveness
- Write clear, maintainable code
- Test thoroughly before committing
- Follow existing patterns in the codebase
- Document complex logic with comments
- Consider backward compatibility when making changes
