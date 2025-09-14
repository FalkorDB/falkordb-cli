Release binaries
----------------

This repository includes a GitHub Actions workflow that builds and publishes release binaries for multiple platforms when a GitHub Release is published (release event). The workflow listens for the `release` event and runs when a release is published.

What the workflow does
- Builds Linux (glibc) and Linux (musl) binaries on Ubuntu runner
- Builds Windows binary (PE) on Ubuntu runner via cross-target (requires appropriate toolchains)
- Packages artifacts and computes SHA256 checksums
- Creates a GitHub Release for the tag and uploads the artifacts

Triggering a release
1. Create a Release on GitHub (UI) and publish it, or create one via the API. Make sure the release has a tag (for example `v0.1.0`).

2. When the Release is published, GitHub runs the workflow. The workflow will build the binaries and attach them to the Release that triggered the workflow (it uses the `github.event.release.tag_name` value).

You can also create and publish a release from the command line using the GitHub CLI:

```bash
# create and publish a release with gh
gh release create v0.1.0 --title "v0.1.0" --notes "Release notes here"
```

Notes and recommended improvements
- Building macOS artifacts currently requires a runner with macOS (the workflow prints a placeholder). To include macOS builds, add a matrix job that uses `runs-on: macos-latest` and repeats the build steps there.
- For a more full-featured release pipeline (Homebrew, GitHub Homebrew taps, snap, deb/rpm packaging), consider using `goreleaser` and its `.goreleaser.yml` configuration.
- Cross-compiling for Windows and musl may require installing additional toolchains in the workflow; if you encounter linker errors, switch to per-platform runners (windows-latest, macos-latest) and build natively.

Publish details
----------------
The CI now runs a build matrix where each job uploads its packaged artifacts as workflow artifacts. A final `publish` job waits for the matrix to complete, downloads all artifacts, aggregates them into a single `release/` folder, recomputes a combined `COMBINED_SHA256SUMS.txt`, optionally signs that file with a GPG key (if you set the `GPG_PRIVATE_KEY` secret), and then uploads all aggregated files to the GitHub Release that triggered the workflow.

To enable GPG signing add a repository secret named `GPG_PRIVATE_KEY` that contains an ASCII-armored private key. The workflow will import that key and produce `COMBINED_SHA256SUMS.txt.asc` which will be attached to the Release.

If you'd like, I can add a `goreleaser` config and a macOS matrix job next.
