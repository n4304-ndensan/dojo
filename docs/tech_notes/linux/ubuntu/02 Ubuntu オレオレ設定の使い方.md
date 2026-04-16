# Ubuntu オレオレ設定の使い方

## この文書の責務

Ubuntu オレオレ設定を、**手動で読むルート**と**Ansible で一括適用するルート**に分けて案内します。

設定の中身そのものは [[01 Ubuntu オレオレ設定一覧]] を参照してください。

## 1. 手動で追う場合

設定の意図を理解しながら 1 つずつ進めるなら、次の順で読むのが自然です。

1. [[../zsh/00 zsh ドキュメントガイド]]
2. [[../zsh/02 zsh 基盤セットアップ]]
3. [[../zsh/03 zshrc 設定とプラグイン]]
4. [[../zsh/04 CLI 操作環境の構築]]
5. [[../zsh/05 tmux 連携と起動速度最適化]]
6. [[../neovim/00 Neovim ドキュメントガイド]]
7. [[../neovim/01 Ubuntu Neovim 環境 OS 基盤セットアップ]]
8. [[../neovim/02 Neovim コア設定]]
9. [[../neovim/03 プラグイン設定（検索・ファイル・UX）]]
10. [[../neovim/04 LSP・補完・整形]]
11. [[../neovim/05 言語別設定]]

## 2. Ansible で一括適用する場合

実際に再現可能な形で構築するなら、Ansible を使うのが速いです。

Ansible 資産は次にあります。

```text
infra/ansible/ubuntu_dev/
```

使い方の詳細は次を参照してください。

```text
infra/ansible/ubuntu_dev/README.md
```

Ansible 自体の基本概念から学びたい場合は、先に [[../ansible/00 Ansible 基礎ガイド]] を読むと理解しやすくなります。

最短手順は以下です。

1. `infra/ansible/ubuntu_dev/group_vars/all.yml` を必要に応じて調整する
2. `infra/ansible/ubuntu_dev/inventories/localhost.ini` を確認する
3. `infra/ansible/ubuntu_dev/` に移動する
4. `ansible-playbook playbooks/site.yml -K` を実行する

実行時は標準出力に、対象ホスト、導入バージョン、apt ロック待機設定、現在のパッケージマネージャー状態が表示されます。

Ubuntu 初期セットアップ直後に `unattended-upgrades` が `dpkg` ロックを掴んでいる場合でも、playbook は一定時間待機してから続行します。待機上限は `group_vars/all.yml` の `ubuntu_dev_apt_lock_timeout` で調整します。

## 3. どちらを選ぶべきか

| 目的 | 選ぶ方法 |
| --- | --- |
| 学習しながら理解したい | 手動ドキュメント |
| すぐに同じ環境を再現したい | Ansible |
| 自分用の変数だけ変えて何台かに配りたい | Ansible |
| 一部だけ設定を摘んで使いたい | 手動ドキュメント |

## 4. 実務上のおすすめ

- まずドキュメントで構成を把握する
- その後は Ansible を正本として扱う
- 設定変更はなるべく Ansible 側の template / vars に寄せる

こうしておくと、WSL、ノート PC、検証用 VM を横断して同じ構成を再現しやすくなります。
