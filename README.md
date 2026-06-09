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

## 下载 macOS 版本

编译结果**不会出现在 Code 页面**，请从下面两种方式下载。

### 方式一：Releases（推荐，最直观）

打开 [Releases 页面](https://github.com/ppnote/EditorToolRust/releases)，下载最新的 **EditorTool-macos.zip**。

> 需要仓库推送包含 Release 发布的 workflow 后才会出现。若还没有 Release，请用方式二。

### 方式二：Actions → Artifacts

以 [Build #3](https://github.com/ppnote/EditorToolRust/actions/runs/27211148569) 为例（已构建成功，4.89 MB）：

1. **必须先登录 GitHub**（未登录看不到下载按钮）
2. 打开 Actions 运行页，**滚动到页面最底部**
3. 找到 **Artifacts** 表格，点击 **EditorTool-macos** 下载 zip
4. 解压后得到：
   - `editor-tool-rust` — 独立可执行文件
   - `EditorTool.app` — macOS 应用程序

终端运行：

```bash
chmod +x editor-tool-rust
./editor-tool-rust
```

或双击 `EditorTool.app`。

> 这是 **Mac 程序**，不能在 Windows 上运行。  
> macOS 首次打开若提示「无法验证开发者」，请在 **系统设置 → 隐私与安全性** 中允许运行。

推送到 `main` 后会自动构建并发布 Release；也可在 Actions 页面手动运行 **Build macOS**。

## 推送至 GitHub

```bash
git remote add origin https://github.com/<你的用户名>/<仓库名>.git
git branch -M main
git push -u origin main
```
