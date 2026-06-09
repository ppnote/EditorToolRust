# EditorTool (Rust)

跨平台文本编辑器：粘贴时自动删除空白行，支持手动清理与复制。

## 功能

- 粘贴文本时自动去除空白行
- **删除空白行** 按钮：清理编辑框全部内容
- **复制** 按钮：有选中则复制选中内容，否则复制全文

## 环境要求

- [Rust](https://www.rust-lang.org/tools/install) 1.75+（stable）

## 本地运行

```bash
cargo run --release
```

## Windows 编译

```powershell
cargo build --release
# 输出: target\release\editor-tool-rust.exe
```

## macOS 编译

```bash
bash build-macos.sh
open dist/EditorTool.app
```

## GitHub Actions

推送到 `main` 或 `master` 后，Actions 会自动构建 macOS 版 `EditorTool.app`，可在 Artifacts 中下载。

也可在 Actions 页面手动运行 **Build macOS** 工作流。

## 推送至 GitHub

```bash
git remote add origin https://github.com/<你的用户名>/<仓库名>.git
git branch -M main
git push -u origin main
```
