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

## 下载编译版本

编译结果**不会出现在 Code 页面**，请从 **Releases** 或 **Actions → Artifacts** 下载。

### Releases（推荐）

打开 [Releases 页面](https://github.com/ppnote/EditorToolRust/releases)：

| 构建工作流 | Release 名称 | 下载文件 |
|-----------|-------------|---------|
| Build Windows | Windows 构建 #N | `EditorTool-windows.zip` → `editor-tool-rust.exe` |
| Build macOS | macOS 构建 #N | `EditorTool-macos.zip` → `editor-tool-rust` + `EditorTool.app` |

> 需要推送包含 Release 发布的 workflow 后才会出现对应 Release。

### Actions → Artifacts（备用）

1. **必须先登录 GitHub**
2. 打开对应 Actions 运行页，**滚动到页面最底部**
3. 点击 Artifacts 中的 zip 下载

Windows 示例：[Build Windows #4](https://github.com/ppnote/EditorToolRust/actions/runs/27282395860) → **EditorTool-windows**（2.21 MB）

macOS 示例：[Build macOS #3](https://github.com/ppnote/EditorToolRust/actions/runs/27211148569) → **EditorTool-macos**（4.89 MB）

### macOS 运行说明

```bash
chmod +x editor-tool-rust
./editor-tool-rust
```

或双击 `EditorTool.app`。首次打开若提示「无法验证开发者」，请在 **系统设置 → 隐私与安全性** 中允许运行。

推送到 `main` 后会自动构建；也可在 Actions 页面手动运行 **Build Windows** / **Build macOS**。

## 推送至 GitHub

```bash
git remote add origin https://github.com/<你的用户名>/<仓库名>.git
git branch -M main
git push -u origin main
```
