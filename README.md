# EditorTool (Rust)

跨平台文本编辑器：粘贴时自动删除空白行，支持手动清理与复制。

## 功能

- 粘贴文本时自动去除空白行
- **删除空白行** 按钮：清理编辑框全部内容
- **复制** 按钮：有选中则复制选中内容，否则复制全文

## 环境要求

- [Rust](https://www.rust-lang.org/tools/install) 1.75+（stable）

在仓库根目录打开终端，直接执行以下命令（无需 `cd`）。

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

## GitHub Actions 下载

构建成功后，可执行文件**不会出现在仓库 Code 页面**，需要到 Actions 里下载 Artifacts。

以 [Build macOS #2](https://github.com/ppnote/EditorToolRust/actions/runs/27210897344) 为例：

1. 打开 Actions 运行记录页面
2. 滚动到页面底部 **Artifacts** 区域
3. 点击 **EditorTool-macos** 下载 zip（约 2.5 MB）
4. 解压后得到：
   - `editor-tool-rust` — 独立可执行文件（终端运行）
   - `EditorTool.app` — macOS 应用程序（双击运行）

终端运行：

```bash
chmod +x editor-tool-rust
./editor-tool-rust
```

或双击 `EditorTool.app`。

> macOS 首次打开若提示「无法验证开发者」，请在 **系统设置 → 隐私与安全性** 中允许运行。

推送到 `main` 或 `master` 后会自动构建；也可在 Actions 页面手动运行 **Build macOS**。

## 推送至 GitHub

```bash
git remote add origin https://github.com/<你的用户名>/<仓库名>.git
git branch -M main
git push -u origin main
```
