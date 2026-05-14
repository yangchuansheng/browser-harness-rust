# Snap Chromium and browser-harness (Linux)

## Why Snap browsers break CDP

Ubuntu and several other distributions ship Chromium as a [Snap](https://snapcraft.io/) package. Snap runs apps in a confined environment. Chrome’s remote debugging endpoint must bind on the host network where the `browser-harness` daemon can reach it. Snap’s sandbox and filesystem layout commonly prevent that from working the way a normal `.deb` Chrome install does, so the harness may see no usable DevTools port even when Chromium appears to run.

Symptoms: `browser-harness verify-install` or `browser-harness page-info` shows Chrome running, but the daemon never attaches, or CDP handshake fails without an obvious cause. [Issue #191](https://github.com/browser-use/browser-harness/issues/191) discusses this class of setup problem.

## Install Google Chrome natively (Ubuntu example)

Use Google’s official package (AMD64), not the Snap:

```bash
wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
sudo apt install ./google-chrome-stable_current_amd64.deb
```

ARM or other architectures: download the matching package from [Google Chrome for Linux](https://www.google.com/chrome/linux/).

## Point the harness at the native binary

Put the non-Snap binary **first** in how you resolve “which Chrome,” so a Snap wrapper on `PATH` is not chosen by mistake.

- **`BH_CHROME_PATH`** — preferred name in this project’s docs for selecting a non-Snap Chrome binary.
- **`CHROME_PATH`** — honored the same way for compatibility with other tooling.

Example for `~/.bashrc` or your environment:

```bash
export BH_CHROME_PATH=/usr/bin/google-chrome-stable
```

Then start Chrome from that path for Way 2 (`--remote-debugging-port=…`), or use Way 1 with a profile opened from the native install. Connection details are in [`install.md`](../install.md).

## Verify

```bash
browser-harness verify-install
```

If a Snap binary is still being launched on Linux, prefer the native Chrome package path above or set `BU_CDP_URL` to a dedicated non-Snap browser launched with Way 2 from `install.md`.
