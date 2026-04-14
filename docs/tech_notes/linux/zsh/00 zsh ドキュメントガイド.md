# zsh ドキュメントガイド

## この文書の責務

Ubuntu 24.04 上に zsh を中心とした CLI 操作環境を構築する一連の手順書群への入口です。zsh 単体の設定ではなく、「CLI 全体の設計」として検索・移動・並列作業・編集までを一貫して扱います。

## 前提

- OS: Ubuntu 24.04 LTS
- シェル: zsh（bash から移行）
- 設計方針: 起動速度・操作効率・拡張性・再現性をすべて満たす

## 文書一覧と読む順序

| # | 文書 | 責務 |
| --- | --- | --- |
| 1 | [[01 シェルの基礎と zsh を選ぶ理由]] | シェルとは何か、bash / zsh / fish の比較、zsh を選ぶ根拠 |
| 2 | [[02 zsh 基盤セットアップ]] | zsh のインストール、デフォルトシェル変更、zinit、starship |
| 3 | [[03 zshrc 設定とプラグイン]] | .zshrc の構造、プラグイン、補完、キーバインド、エイリアス |
| 4 | [[04 CLI 操作環境の構築]] | fzf、zoxide、ripgrep、fd、eza による検索・移動・表示の統合 |
| 5 | [[05 tmux 連携と起動速度最適化]] | tmux の基本操作、セッション管理、zinit 遅延ロード、起動速度計測 |

初回セットアップでは 1 → 5 の順に進めてください。

## CLI 全体の設計思想

zsh は「シェル単体」ではなく、CLI 操作環境全体の入力インターフェースとして位置づけます。

```text
zsh ─────── 入力インターフェース（コマンド入力・補完・履歴）
starship ── 状態可視化（ブランチ・言語バージョン・コマンド実行時間）
fzf ─────── 検索（履歴・ファイル・ディレクトリ）
zoxide ──── 移動（学習型ディレクトリジャンプ）
tmux ────── 並列作業（ウィンドウ・ペイン・セッション）
neovim ──── 編集（→ [[00 Neovim ドキュメントガイド]] を参照）
```

この構成により、GUI を使わなくても開発作業のほぼすべてを CLI で完結できます。

## ツール一覧

| ツール | 役割 | 置き換え対象 |
| --- | --- | --- |
| zsh | シェル | bash |
| zinit | プラグイン管理 | oh-my-zsh |
| starship | プロンプト | 手書きの PS1 / powerlevel10k |
| fzf | ファジー検索 | `history \| grep` / `find` |
| zoxide | ディレクトリ移動 | `cd` / `autojump` / `z.sh` |
| ripgrep (`rg`) | テキスト検索 | `grep` |
| fd | ファイル検索 | `find` |
| eza | ファイル一覧 | `ls` |
| tmux | ターミナル多重化 | 複数ターミナルウィンドウ |

## Neovim との関係

この zsh 文書群と [[00 Neovim ドキュメントガイド]] は相互補完の関係にあります。Neovim の OS 基盤セットアップで導入する ripgrep、fd、fzf などのツールは zsh 側でも使います。セットアップ順序としては、**zsh 環境を先に整えてから Neovim に進む**のが自然です。

## 参考

- [zsh 公式](https://www.zsh.org/)
- [zinit](https://github.com/zdharma-continuum/zinit)
- [starship](https://starship.rs/)
- [fzf](https://github.com/junegunn/fzf)
- [zoxide](https://github.com/ajeetdsouza/zoxide)
