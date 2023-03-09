# playwithrust

## windows install

> get rustup-init.exe

```bash
curl https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe -o rustup-init.exe
```

> install with GNU build tools

```bash
rustup-init.exe --default-host x86_64-pc-windows-gnu
```

> add LSP (Language Server Protocol) [video](https://www.youtube.com/watch?v=ifaLk5v3W90) [detail](https://emacs-lsp.github.io/lsp-mode/page/lsp-rust-analyzer/)

```bash
# check what is installed
rustup component list | findstr installed
# find rust-analyzer
rustup component list | findstr analyzer
# install rust-analyzer
rustup component add rust-analyzer-x86_64-pc-windows-gnu
```
