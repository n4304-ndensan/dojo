# Ansible ファイル構成と読み方

## この文書の責務

Ansible の学習者に向けて、**どのファイルが何を表しているのか**、**どの順序で読めばよいのか**、**同じ目的を別の書き方で表現するならどう書けるのか**を、このリポジトリの `infra/ansible/ubuntu_dev/` を題材に説明します。

この文書は、Ansible の基本概念を説明した [[00 Ansible 基礎ガイド]] の続編です。概念だけでなく、実ファイルを前にしたときに「何を見るべきか」を掴めるようにすることを目的とします。

## 想定読者

- `inventory` や `playbook` の名前は知っているが、ファイル単位での役割分担がまだ曖昧な人
- `ubuntu_dev` の Ansible を自分で読んで直せるようになりたい人
- Ansible の「別解」や書き方の選択肢も一緒に知りたい人

## 前提

- [[00 Ansible 基礎ガイド]] を読んでいること
- YAML の基本文法を読めること
- Linux 上のファイル配置や `sudo` の概念を知っていること

## 1. まず全体をどう分解するか

Ansible のディレクトリを読むときは、最初から task の細部に入るより、**入口、対象、変数、責務分割、設定ファイル本体**の順で見る方が理解しやすいです。

`ubuntu_dev` を例にすると、見る順番は次のようになります。

```text
ansible.cfg
  ↓
inventory
  ↓
playbook
  ↓
group_vars
  ↓
roles/
  ├── defaults
  ├── tasks
  └── templates
```

この順序には理由があります。`ansible.cfg` は Ansible の実行前提を決め、inventory は対象ホストを決め、playbook は入口を決め、group_vars は可変部分を決め、role は実装本体を持つからです。

## 2. `ansible.cfg` は「実行前提」を読むファイル

このリポジトリでは [ansible.cfg](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/ansible.cfg) が最上流にあります。

```ini
[defaults]
inventory = inventories/localhost.ini
roles_path = roles
host_key_checking = False
retry_files_enabled = False
interpreter_python = auto_silent
```

### このファイルが指しているもの

`ansible.cfg` は、Ansible コマンド自体の挙動を決めます。ここで特に重要なのは `inventory` と `roles_path` です。

- `inventory = inventories/localhost.ini`  
  既定でどの inventory を使うかを決めています。

- `roles_path = roles`  
  playbook から `common` や `zsh` と書いたときに、どこを role の探索パスにするかを決めています。

つまり `ansible.cfg` は、「このディレクトリで Ansible を実行したときの標準ルール」を決めるファイルです。

### 読むときの観点

`ansible.cfg` は、次の観点で読むと十分です。

1. inventory はどこを向いているか
2. role はどこから読まれるか
3. SSH や Python 解決の挙動に特別な設定があるか
4. 実行環境依存の危険な緩和設定があるか

### ほかの書き方

同じことを別の形で表す方法はいくつかあります。

- `ansible.cfg` を置かず、毎回 `-i inventories/localhost.ini` を付けて実行する
- `ANSIBLE_CONFIG` 環境変数で別の設定ファイルを明示する
- `roles_path` を書かず、playbook から role への相対パスを直接扱う

ただし学習用や小規模構成では、今のように `ansible.cfg` で標準値を固定した方が読みやすいです。

## 3. inventory は「どこに適用するか」を読むファイル

このリポジトリでは [localhost.ini](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/inventories/localhost.ini) が inventory です。

```ini
[ubuntu_dev]
localhost ansible_connection=local
```

### このファイルが指しているもの

inventory は、Ansible の適用対象ホストと、そのホストがどのグループに属するかを定義します。

ここでは:

- グループ名は `ubuntu_dev`
- 対象ホストは `localhost`
- 接続方法は `ansible_connection=local`

なので、「SSH で別サーバーに入る」のではなく、「今いる Ubuntu / WSL 自身に対してローカル実行する」構成です。

### 読むときの観点

inventory を読むときは、次を見ます。

1. ホストは誰か
2. グループは何か
3. 接続方式は local か SSH か
4. ホスト単位の追加変数があるか

### ほかの書き方

inventory は INI だけではありません。YAML でも書けます。

```yaml
all:
  children:
    ubuntu_dev:
      hosts:
        localhost:
          ansible_connection: local
```

INI は短くて読みやすいですが、構造が複雑になってくると YAML の方が見通しが良くなります。小規模なローカル環境なら、今の INI 形式で十分です。

## 4. playbook は「入口と順序」を読むファイル

このリポジトリでは [site.yml](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/playbooks/site.yml) が入口です。

```yaml
- name: Provision Ubuntu development environment
  hosts: ubuntu_dev
  gather_facts: true
  become: true
  vars_files:
    - ../group_vars/all.yml

  pre_tasks:
    - name: Resolve target user
      ansible.builtin.set_fact:
        ubuntu_dev_user: "{{ lookup('ansible.builtin.env', 'SUDO_USER') | default(lookup('ansible.builtin.env', 'USER')) | default(ansible_user_id) }}"

    - name: Show playbook execution summary
      ansible.builtin.debug:
        msg:
          - "..."

  roles:
    - role: common
      tags:
        - common
        - zsh
        - neovim
    - role: zsh
      tags:
        - zsh
    - role: neovim
      tags:
        - neovim
```

### このファイルが指しているもの

playbook は「何を、どこに、どの順番で適用するか」の入口です。

- `hosts: ubuntu_dev` で inventory のどのグループへ適用するかを決める
- `gather_facts: true` で facts を集める
- `become: true` で昇格実行する
- `vars_files` で外部変数を明示的に読む
- `pre_tasks` で本体実行前の共通前処理を行う
- `roles` で責務ごとの実装を順に呼ぶ

### 読むときの観点

playbook を読むときは次の順で見ると早いです。

1. `hosts`
2. `vars_files`
3. `pre_tasks`
4. `roles`
5. `tags`

`pre_tasks` は特に重要です。ここにユーザー解決や前提チェックが入っていると、role の本体を読む前に実行前提を理解できます。

### ほかの書き方

playbook にはいくつかの別解があります。

#### `vars_files` を使わず `group_vars` 自動読込に任せる

Ansible は通常、inventory や playbook の近くにある `group_vars/` を自動で読みます。今回は実行経路の差異で未読込になるケースがあったため、明示的に `vars_files` を書いています。

#### `roles:` ではなく `tasks:` に直接書く

小さい playbook なら可能です。

```yaml
- hosts: ubuntu_dev
  tasks:
    - name: Install curl
      ansible.builtin.apt:
        name: curl
        state: present
```

ただし規模が大きくなると責務分離が崩れるため、今の role 分割の方が保守しやすいです。

#### `pre_tasks` の代わりに最初の role を専用化する

共通初期化用の role を作って、最初に `bootstrap` role を呼ぶ形もあります。今回は playbook の入口で必ず見える方が理解しやすいので `pre_tasks` に置いています。

## 5. `group_vars/all.yml` は「可変部分」を読むファイル

このリポジトリでは [all.yml](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/group_vars/all.yml) に主な変数がまとまっています。

```yaml
ubuntu_dev_user: "..."
ubuntu_dev_change_default_shell: true

ubuntu_dev_starship_version: "1.24.2"
ubuntu_dev_neovim_version: "0.12.1"
ubuntu_dev_apt_lock_timeout: 600
...
```

### このファイルが指しているもの

このファイルは「同じ構成の中で変更対象になりやすい値」をまとめています。バージョン番号、フラグ、タイムアウトのような値はここに置くと、task 本体が読みやすくなります。

### 読むときの観点

1. バージョン番号
2. 真偽値フラグ
3. タイムアウトや retry 設定
4. リスト形式のツール群

このファイルを見ると、「この playbook は何を可変に設計しているか」が分かります。逆に言えば、task に直書きされている値は「作者が固定値として扱いたいもの」です。

### ほかの書き方

変数の置き場には複数の候補があります。

- `group_vars/all.yml`  
  グループ全体で共通の値

- `host_vars/<hostname>.yml`  
  ホストごとに違う値

- role の `defaults/main.yml`  
  その role の既定値

- playbook の `vars:`  
  その play にだけ閉じた値

一般に、**role の外から上書きしたい値は `group_vars` や `host_vars`、role の内部既定値は `defaults`** に置くと分かりやすいです。

## 6. role は「責務単位の実装」を読むディレクトリ

`roles/` 配下は、Ansible の実装本体です。ここでは「何をするか」を責務ごとに分けています。

このリポジトリでは:

- [roles/common](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/roles/common)
- [roles/zsh](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/roles/zsh)
- [roles/neovim](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/roles/neovim)

の 3 つが中心です。

role の中では、通常次のディレクトリを読みます。

- `defaults/`
- `tasks/`
- `templates/`

必要に応じて `handlers/`、`vars/`、`files/`、`meta/` もありますが、`ubuntu_dev` では主に上の 3 つを使っています。

## 7. `defaults/main.yml` は「role の既定値」を読むファイル

たとえば [roles/common/defaults/main.yml](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/roles/common/defaults/main.yml) には、ホームディレクトリやパッケージ一覧の既定値があります。

### このファイルが指しているもの

`defaults` は、その role が単体で成立するための最低限の既定値です。呼び出し側が上書きしなくても動くようにしておく場所です。

たとえば:

- `ubuntu_dev_home`
- `ubuntu_dev_local_bin`
- `ubuntu_dev_base_packages`

のような値が定義されています。

### 読むときの観点

1. この role が前提にしている変数は何か
2. そのうち既定値を持つものは何か
3. 上書き前提のものは何か

### ほかの書き方

同じ値を `vars/main.yml` に置く方法もありますが、`vars` は優先度が高く、外から上書きしづらいです。初学者向けには、**変更可能性のある値は `defaults` に置く**と覚える方が安全です。

## 8. `tasks/main.yml` は「role の処理本体」を読むファイル

たとえば [roles/common/tasks/main.yml](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/roles/common/tasks/main.yml) には、apt の待機、共通パッケージ導入、ディレクトリ作成、`fd` 互換リンク作成が入っています。

### このファイルが指しているもの

`tasks/main.yml` は、その role の処理の主系列です。`name` の順で実行されるので、上から下に読むだけで流れを追えます。

### 読むときの観点

task を読むときは、次の順で見るのが安定です。

1. `name`
2. module
3. 引数
4. `when`
5. `register`
6. `become` / `become_user`

たとえば `common` role なら:

- package manager の busy 状態を確認する
- `apt` で共通パッケージを入れる
- ユーザー用ディレクトリを作る
- `fdfind` の存在を見て `fd` シンボリックリンクを張る

という流れが読み取れます。

### ほかの書き方

#### `main.yml` 1 枚に書き切る

今の構成がこれです。役割がまだ小さいときは最も読みやすいです。

#### `import_tasks` / `include_tasks` で分割する

task が増えたら、以下のように分割できます。

```yaml
- import_tasks: packages.yml
- import_tasks: directories.yml
- import_tasks: links.yml
```

`import_tasks` は静的展開、`include_tasks` は動的読込という違いがあります。学習の初期段階では `import_tasks` の方が追いやすいです。

#### `block` / `rescue` / `always` を使う

失敗時の補足処理やログ出力が必要な場合は次のような書き方もあります。

```yaml
- block:
    - name: Do something
      ansible.builtin.command: ...
  rescue:
    - name: Show fallback info
      ansible.builtin.debug:
        msg: "failed"
  always:
    - name: Final message
      ansible.builtin.debug:
        msg: "done"
```

ただし通常の構成管理では、まず単純な task 列で書けるかを優先した方がよいです。

## 9. `templates/` は「配布する設定ファイルの正本」を読むディレクトリ

たとえば:

- [roles/zsh/templates/zshrc.j2](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/roles/zsh/templates/zshrc.j2)
- [roles/zsh/templates/starship.toml.j2](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/roles/zsh/templates/starship.toml.j2)

のようなファイルがあります。

### このファイルが指しているもの

`templates/` 配下の `.j2` ファイルは、対象ホストに配布する設定ファイルの雛形です。Jinja2 の変数展開が使えるため、ユーザー名やパス、バージョンに応じた動的なファイルを生成できます。

### 読むときの観点

1. 完成後にどこへ置かれるファイルか
2. どの変数を差し込んでいるか
3. 純粋な静的ファイルなのか、テンプレート化の理由があるのか

### ほかの書き方

#### `copy` を使う

完全に静的なファイルなら `template` ではなく `copy` でもよいです。

```yaml
- name: Copy static file
  ansible.builtin.copy:
    src: my.conf
    dest: /etc/my.conf
```

#### `lineinfile` や `blockinfile` を使う

既存ファイルの一部だけを編集したい場合は便利です。

```yaml
- name: Ensure one line exists
  ansible.builtin.lineinfile:
    path: ~/.bashrc
    line: 'export PATH="$HOME/.local/bin:$PATH"'
```

ただし学習用・再現性重視の構成では、**設定ファイル全体を template で正本管理する**方が分かりやすいことが多いです。

## 10. この構成であえて採っていないもの

Ansible にはまだ多くの要素がありますが、`ubuntu_dev` では意図的に使っていないものもあります。

### `handlers`

サービス再起動のように、「変更があったときだけ最後にまとめて実行する」処理に使います。今回は systemd service の再起動が中心ではないので使っていません。

### `files/`

静的ファイルをそのまま配る置き場です。今回は主に template 中心なので最小限です。

### `meta/`

role の依存関係や Galaxy 情報を持ちます。社内・個人用の小規模 role では省略されることも多いです。

## 11. 初学者が「別の書き方」を学ぶときの優先順位

Ansible は書き方の自由度が高いので、初学者ほど「何でもできる」ことが逆に負担になります。学習の順番を固定した方がよいです。

1. まずは今の構成のような素直な `role + tasks/main.yml + defaults/main.yml + templates/` を読めるようになる
2. 次に `import_tasks` や YAML inventory のような分割・表現差を学ぶ
3. その後に `handlers`、`block/rescue`、`include_role`、`host_vars`、複数 inventory を学ぶ

最初から高度な機能を全部覚える必要はありません。Ansible の本質は、構成管理の責務分離と再実行性にあります。

## 結論

Ansible のファイルを読むときは、個々の YAML 記法より先に、**このファイルは「前提」「対象」「入口」「変数」「実装」「配布物」のどれを担当しているのか**を掴むべきです。

`ubuntu_dev` の構成は、`ansible.cfg`、inventory、playbook、`group_vars`、role の `defaults / tasks / templates` という、Ansible の基本形が素直に揃っています。まずはその責務分割を理解し、その後で「別の書き方」を比較すると、選択肢が増えても迷いにくくなります。
