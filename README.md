# yt-dl

A simple command-line YouTube downloader written in Rust. Wraps [yt-dlp](https://github.com/yt-dlp/yt-dlp) with an interactive CLI so you don't have to remember flags.

## Features

- Download YouTube videos (mkv) or extract audio only (m4a)
- Custom output folder, with your Downloads folder as default
- Loop — download multiple videos without restarting
- No Python required (uses standalone yt-dlp)

## Requirements

- [yt-dlp standalone exe](https://github.com/yt-dlp/yt-dlp/releases) — download `yt-dlp.exe` and place it in the same folder as `yt-dl.exe`

## Setup

### Option 1 — Download the release (recommended)

1. Download `yt-dl.exe` from the [releases page](../../releases)
2. Download `yt-dlp.exe` from the [yt-dlp releases page](https://github.com/yt-dlp/yt-dlp/releases) and place it in the same folder
3. Run `yt-dl.exe`

### Option 2 — Build from source

1. Build the project:
   ```
   cargo build --release
   ```
2. Copy `target/release/yt-dl.exe` to a folder of your choice
3. Download `yt-dlp.exe` from the [yt-dlp releases page](https://github.com/yt-dlp/yt-dlp/releases) and place it in the same folder
4. Run `yt-dl.exe`

## Usage

```
||Youtube downloader||
Youtube link (type exit if want to leave): https://www.youtube.com/watch?v=...
Path to download (press enter for default): 
Audio only? (y/n): y
```

- Leave the path blank to use your Downloads folder
- Type `exit` at the URL prompt to quit

## License

MIT - license
