# ghit

**ghit** is a CLI that uses AI to automatically generate meaningful Git commit messages and streamline your dev workflow.

## Features

- **AI-Generated Commit Messages**: Uses OpenAI's GPT models to analyze your changes and create descriptive commit messages
- **Workflow Automation**: Add files, generate message, commit, and push in one command
- **Smart Diff Analysis**: Automatically stages all modified files and analyzes multi-file changes
- **Configurable Verbosity**: Choose between quiet (one-line), normal (subject + body), or verbose commit messages
- **Interactive Confirmation**: Review and approve generated messages before committing
- **Simple Configuration**: Easy setup with a single TOML config file

## Quick Start

### Installation

#### Option 1: Install from Releases (Recommended)

Download the latest binary for your platform from the [releases page](https://github.com/evisdrenova/ghit/releases):

**macOS/Linux:**

```bash
curl -L https://github.com/evisdrenova/ghit/releases/latest/download/ghit-$(uname -s)-$(uname -m).tar.gz | tar -xz
sudo mv ghit /usr/local/bin/
```

**Windows:**
Download `ghit-windows.zip` from releases and add to your PATH.

#### Option 2: Install from Source

```bash
git clone https://github.com/evisdrenova/ghit.git
cd ghit
cargo build --release
sudo cp target/release/ghit /usr/local/bin/
```

#### Option 3: Install with Cargo

```bash
cargo install ghit
```

### Configuration

Create a `ghit.toml` file in your home directory (`~/.ghit.toml` or `C:\Users\YourName\ghit.toml`):

```toml
model = "gpt-4.1"  # any openai model
api_key = "your-openai-api-key-here"
message_level = "normal"  # quiet, normal, or verbose
```

## Usage

### Basic Commands

**Automatic workflow (most common):**

```bash
# Auto-stage all changes, generate message, commit, and push
ghit --auto

# Or simply (default behavior)
ghit
```

**Generate message only:**

```bash
# Generate commit message for staged changes (doesn't commit)
ghit --generate
```

**Stage and generate:**

```bash
# Add specific files and generate message (doesn't commit)
ghit --stage-and-generate src/main.rs src/lib.rs

# Add all changes and generate message
ghit --stage-and-generate
```

### Working with Specific Files

```bash
# Auto-commit only specific files
ghit --auto src/main.rs README.md

# Push to a specific branch
ghit --auto --branch feature-branch

# Stage specific files and generate message
ghit --stage-and-generate src/components/ tests/
```

## Configuration Options

| Option          | Description                                             | Default  |
| --------------- | ------------------------------------------------------- | -------- |
| `model`         | OpenAI model to use (`gpt-3.5-turbo`, `gpt-4`, etc.)    | `none`   |
| `api_key`       | Your OpenAI API key                                     | Required |
| `message_level` | Commit message verbosity (`quiet`, `normal`, `verbose`) | `normal` |

### Message Verbosity Levels

Configure in `~/.ghit.toml`:

- **`quiet`**: One-line commit message only
- **`normal`**: Subject line + short description (default)
- **`verbose`**: Detailed subject + explanatory body

## Example Workflow

1. **Make your changes** to any files in your repository
2. **Run ghit:**
   ```bash
   ghit --auto
   ```
3. **Review the generated message:**

   ```
   Generated commit message:
   Subject: feat: add user authentication with JWT tokens
   Body:
   - Implement JWT-based authentication system
   - Add login and registration endpoints
   - Include middleware for protected routes
   - Update user model with password hashing

   Create this commit? [Y/n]:
   ```

4. **Confirm** by pressing Enter or 'y'
5. **Done!** Your changes are committed and pushed automatically

## What Makes Good Commit Messages?

ghit analyzes your code changes and generates commit messages that follow best practices:

- **Clear subject lines** that summarize the change
- **Conventional commit format** (feat:, fix:, docs:, etc.)
- **Detailed explanations** of what changed and why
- **Multi-file awareness** that understands related changes across files

### Example Generated Messages

**For a bug fix:**

```
fix: resolve memory leak in image processing

- Fix buffer overflow in resize function
- Add proper cleanup for temporary arrays
- Update error handling for edge cases
```

**For a new feature:**

```
feat: implement dark mode toggle

- Add theme context provider
- Create toggle component with smooth transitions
- Persist user preference in localStorage
- Update all components to support both themes
```

## Command Line Options

```
ghit [OPTIONS] [FILES...]

OPTIONS:
    -a, --auto                 Add files, generate message, commit, and push automatically
    -b, --branch <BRANCH>      Branch to push to (defaults to current branch)
    -g, --generate             Only generate commit message for staged changes
    -s, --stage-and-generate   Add files and generate message (don't commit)
    -h, --help                 Print help message

EXAMPLES:
    ghit                       # Auto-commit all changes (default)
    ghit --auto               # Same as above
    ghit --branch main        # Push to specific branch
    ghit src/main.rs          # Auto-commit specific file
    ghit --generate           # Just generate message for staged changes
    ghit -s src/ tests/       # Stage files and generate message
```

## Troubleshooting

**"No changes found to commit"**

- Make sure you have modified files in your repository
- Check `git status` to see if there are any changes

**"Failed to load config"**

- Ensure `ghit.toml` exists in your home directory (`~/.ghit.toml`)
- Verify your OpenAI API key is correct and has credits

**"OpenAI API error"**

- Check your API key is valid and has available credits
- Verify your internet connection
- Try a different model (e.g., `gpt-3.5-turbo` instead of `gpt-4`)

**Permission denied when installing**

- Use `sudo` when copying to `/usr/local/bin/`
- Or install to a directory in your PATH that you own

## Contributing

Contributions are welcome! Here's how to get started:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and test them
4. Use ghit to commit your changes!
5. Push and create a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

If you find ghit useful, please consider:

- Starring the repository
- Reporting bugs or requesting features
- Contributing improvements
- Sharing with other developers
