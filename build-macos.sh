#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

echo "Building macOS release binary..."
cargo build --release

BIN="target/release/editor-tool-rust"
APP_DIR="dist/EditorTool.app"
CONTENTS="$APP_DIR/Contents"
MACOS="$CONTENTS/MacOS"

rm -rf dist
mkdir -p "$MACOS"

cp "$BIN" "$MACOS/editor-tool-rust"
chmod +x "$MACOS/editor-tool-rust"

cat > "$CONTENTS/Info.plist" <<'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>zh-Hans</string>
    <key>CFBundleExecutable</key>
    <string>editor-tool-rust</string>
    <key>CFBundleIdentifier</key>
    <string>com.editortool.app</string>
    <key>CFBundleName</key>
    <string>文本编辑器</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleVersion</key>
    <string>0.1.0</string>
    <key>LSMinimumSystemVersion</key>
    <string>11.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

echo "Done: $APP_DIR"
echo "Run with: open dist/EditorTool.app"
