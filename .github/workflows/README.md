# Generic Rust Release Workflow

This is a generic GitHub Actions workflow for building and releasing Rust projects across multiple platforms. It can be easily adapted for any Rust project with minimal configuration changes.

## Features

- ✅ **Cross-platform builds**: Linux (x86_64), macOS (Intel & Apple Silicon), Windows (x86_64)
- ✅ **Robust file handling**: Automatic cleanup and overwrite protection
- ✅ **Release management**: Automatic deletion and recreation of existing releases
- ✅ **Version handling**: Automatic version extraction from Cargo.toml
- ✅ **Artifact management**: Organized binary artifacts with version suffixes
- ✅ **Comprehensive logging**: Debug output for troubleshooting
- ✅ **Error resilience**: Continue-on-error handling for edge cases

## Quick Setup

### 1. Copy the workflow file

Copy `rust-release.yml` to your project's `.github/workflows/` directory.

### 2. Configure the environment variables

Edit the `env` section at the top of the workflow file:

```yaml
env:
  # Project-specific configuration - customize these for your project
  PROJECT_NAME: your-project-name          # Must match your Cargo.toml name
  PROJECT_DISPLAY_NAME: Your Project Name  # Human-readable name for releases
  RUST_TOOLCHAIN: 1.88.0                  # Rust toolchain version
  # GitHub repository for changelog link (format: owner/repo)
  CHANGELOG_REPO: your-username/your-repo-name
```

### 3. Configure your Cargo.toml

Ensure your `Cargo.toml` has the correct binary configuration:

```toml
[package]
name = "your-project-name"  # Must match PROJECT_NAME
version = "0.1.0"

[[bin]]
name = "your-project-name"  # Must match PROJECT_NAME
path = "src/main.rs"
```

### 4. Create a release

Push a tag to trigger the workflow:

```bash
git tag v0.1.0
git push origin v0.1.0
```

## Configuration Options

### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `PROJECT_NAME` | Binary name (must match Cargo.toml) | `sm-menu` |
| `PROJECT_DISPLAY_NAME` | Human-readable project name | `SM-Menu` |
| `RUST_TOOLCHAIN` | Rust toolchain version | `1.88.0` |
| `CHANGELOG_REPO` | GitHub repo for changelog link | `owner/repo-name` |

### Supported Platforms

The workflow builds for these platforms by default:

- **Linux**: x86_64-unknown-linux-gnu
- **macOS Intel**: x86_64-apple-darwin  
- **macOS Apple Silicon**: aarch64-apple-darwin
- **Windows**: x86_64-pc-windows-msvc

### Adding/Removing Platforms

To modify the target platforms, edit the `matrix` section:

```yaml
matrix:
  target: [
    "x86_64-unknown-linux-gnu",
    "x86_64-apple-darwin", 
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc",
    # Add more targets here
  ]
  include:
    - os: ubuntu-latest
      target: x86_64-unknown-linux-gnu
      platform: linux-x86_64
      extension: ""
    # Add more platform mappings here
```

Note: The `platform` field is used for artifact naming and must be unique for each target.

## Binary Naming Convention

The workflow creates binaries with the following naming pattern:

- **Development builds**: `{PROJECT_NAME}-{platform}-{arch}-v{version}`
- **Release artifacts**: `{PROJECT_NAME}-{platform}-{arch}.exe` (Windows only)

Examples:
- `sm-menu-linux-x86_64-v0-1-0`
- `sm-menu-macos-arm64-v0-1-0`
- `sm-menu-windows-x86_64-v0-1-0.exe`

## Release Notes

The workflow automatically generates release notes with:

- Project name and version
- Download links for all platforms
- Installation instructions
- Build information (Rust edition, toolchain)
- Link to CHANGELOG.md

## Troubleshooting

### Common Issues

1. **Binary name mismatch**: Ensure `PROJECT_NAME` matches your Cargo.toml `name` and `[[bin]]` name
2. **Missing CHANGELOG_REPO**: Update the repository path for changelog links
3. **Toolchain issues**: Adjust `RUST_TOOLCHAIN` if you need a specific Rust version
4. **Permission errors**: Ensure your repository has `contents: write` permissions

### Debug Information

The workflow includes comprehensive debug output:

- Current directory structure
- Artifact directory contents  
- Final release files listing
- Version extraction details

### File Conflicts

The workflow handles file conflicts robustly:

- Automatically removes existing binaries before building
- Cleans artifact directories before copying
- Deletes existing GitHub releases before creating new ones

## Advanced Customization

### Custom Build Steps

Add custom build steps before the binary build:

```yaml
- name: Custom pre-build step
  run: |
    # Your custom commands here
    echo "Running custom build preparation"
```

### Additional Artifacts

To include additional files in releases:

```yaml
- name: Prepare additional files
  run: |
    cp README.md release-files/
    cp LICENSE release-files/
```

### Custom Release Notes

Modify the "Generate release notes" step to customize the release description.

## Integration with Other Tools

### With Dependabot

The workflow works seamlessly with Dependabot for dependency updates.

### With Pre-commit Hooks

Compatible with pre-commit hooks for code formatting and linting.

### With Conventional Commits

Can be extended to parse conventional commits for automatic changelog generation.

## Security Considerations

- Uses `contents: write` permission (required for releases)
- Includes `continue-on-error` for release deletion (safe fallback)
- Validates file paths to prevent directory traversal
- Uses force removal only when necessary

## Performance Optimization

- **Caching**: Cargo dependencies are cached across builds
- **Parallel builds**: Matrix strategy builds all platforms simultaneously  
- **Incremental builds**: Rust incremental compilation is enabled
- **Artifact cleanup**: Automatic cleanup prevents storage bloat

## License

This workflow template can be freely used and modified for any project.

## Contributing

To improve this workflow template:

1. Test changes with multiple Rust projects
2. Ensure backward compatibility
3. Update documentation for new features
4. Add validation for configuration errors

## Examples

See the following projects using this workflow:

- [SM-Menu](https://github.com/SimpleMotion-Digital/999997-SD-34-SM-Menu)
- (Add your project here)

## Support

For issues with this workflow template:

1. Check the troubleshooting section above
2. Review the GitHub Actions logs for detailed error information  
3. Ensure all configuration variables are set correctly
4. Verify your Cargo.toml configuration matches the requirements
