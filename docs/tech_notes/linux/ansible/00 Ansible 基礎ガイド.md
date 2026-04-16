# Ansible 基礎ガイド

## この文書の責務

Ansible をこれから学ぶ人に向けて、**Ansible が何を解決するのか**、**どういう単位で構成されるのか**、**このリポジトリの `ubuntu_dev` がどう組み立てられているのか**をまとめて説明します。

この文書は、個別の playbook 手順書ではなく、Ansible を読むための基礎知識を揃えるための学習用ガイドです。

各ファイルをどう読むか、何を表しているか、別の書き方には何があるかまで知りたい場合は、[[01 Ansible ファイル構成と読み方]] を続けて読むと理解しやすくなります。

## 想定読者

- Ansible をほぼ初めて触る人
- `ansible-playbook` は実行したが、inventory や role の意味がまだ曖昧な人
- このリポジトリの `infra/ansible/ubuntu_dev/` を読めるようになりたい人

## 前提

- Linux の基本コマンドをある程度読めること
- YAML の基本的な書式を知っていること
- `sudo` とパッケージ管理の概念を知っていること

## 1. Ansible は何を解決するのか

Ansible は、サーバーや開発環境の構成を**手順書ではなくコードとして表現し、繰り返し同じ形で適用する**ための仕組みです。

手作業で環境構築をすると、誰がどの順序で何を入れたかが曖昧になりやすいです。時間が経つと、最初に入れた手順を自分でも再現できなくなります。Ansible はその問題を避けるために、必要な状態を YAML で宣言し、差分だけを適用します。

ここで重要なのは、Ansible は単なる「コマンド実行ラッパー」ではないという点です。目標は `apt install curl` を実行することではなく、**curl が入っている状態**を再現することです。この「状態を揃える」考え方が Ansible の中心です。

## 2. 最低限の用語

Ansible を読むときは、まず用語の粒度を揃える必要があります。似た言葉が多いので、最初に切り分けて理解しておくと混乱しません。

### inventory

inventory は、Ansible がどのホストに対して処理するかを定義するファイルです。ホスト名、接続方法、グループを持ちます。

このリポジトリでは [localhost.ini](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/inventories/localhost.ini) が inventory で、`localhost ansible_connection=local` となっているため、自分自身の Ubuntu / WSL に対してローカル実行します。

### playbook

playbook は、どのホストに対して、どの role や task をどの順序で適用するかを定義する入口です。

このリポジトリでは [site.yml](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/playbooks/site.yml) が入口で、`common`、`zsh`、`neovim` を順に適用します。

### role

role は、特定の責務をまとめたディレクトリ単位の部品です。Ansible では、巨大な playbook 1 枚に全部を書くより、責務ごとに role に分けるのが一般的です。

このリポジトリでは:

- [roles/common](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/roles/common) は共通パッケージや基本ディレクトリ
- [roles/zsh](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/roles/zsh) は zsh / starship
- [roles/neovim](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/roles/neovim) は Neovim と言語ツールチェーン

という分担です。

### task

task は、Ansible の最小実行単位です。1 つの task は通常 1 つの module を呼びます。

たとえば「apt パッケージを入れる」「ファイルを配置する」「ディレクトリを作る」「Git リポジトリを clone する」が 1 task ずつ表現されます。

### module

module は、Ansible が実際に行う処理の種類です。`apt`、`file`、`template`、`git`、`get_url`、`user` などがあります。

初学者が最初に覚えるべき原則は、**できる限り shell ではなく module を使う**ことです。module は idempotent に書きやすく、変更有無の判定やエラーハンドリングも安定します。

### variable

variable は、値を外出しする仕組みです。バージョン番号、対象ユーザー、フラグ、URL などをまとめます。

このリポジトリでは [all.yml](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/group_vars/all.yml) に、Neovim のバージョンや apt の待機設定が入っています。

### facts

facts は、Ansible が対象ホストから取得する情報です。OS 名、バージョン、アーキテクチャ、環境変数などが含まれます。

`gather_facts: true` が有効な play では、`ansible_distribution` や `ansible_architecture` のような値を task から参照できます。

## 3. Ansible はどう実行されるのか

playbook 実行時に何が起きているかを流れで理解すると、エラーの意味が読みやすくなります。

```text
ansible-playbook 実行
  ↓
inventory を読む
  ↓
対象ホストを決める
  ↓
facts を集める
  ↓
pre_tasks を実行する
  ↓
roles / tasks を上から順に実行する
  ↓
changed / ok / failed を集計する
```

この流れのどこで失敗したかによって、見るべき場所が変わります。

- `Gathering Facts` で失敗するなら接続、Python、権限を疑う
- `undefined variable` なら variable の読み込み位置か評価タイミングを疑う
- `apt` で失敗するなら package manager のロックや repository を疑う
- `become_user` で失敗するなら対象ユーザーやホームディレクトリを疑う

## 4. このリポジトリの `ubuntu_dev` はどう読めばよいか

Ansible の基礎を理解するには、抽象論だけでなく、実物をどう分解して読むかが重要です。このリポジトリでは次の順序で読むのが効率的です。

1. [README.md](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/README.md) を読む  
   何を構築する playbook なのか、どのコマンドで実行するのかを先に掴みます。

2. [site.yml](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/playbooks/site.yml) を読む  
   全体の入口です。どの role がどの順番で実行されるかを見ます。

3. [all.yml](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/group_vars/all.yml) を読む  
   バージョン番号や挙動フラグを把握します。ここを見ると、playbook の可変部分が分かります。

4. [roles/common/tasks/main.yml](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/roles/common/tasks/main.yml) を読む  
   `apt`、`file`、`stat` のような基本 module が多く、Ansible の読み方を学びやすいです。

5. [roles/zsh/tasks/main.yml](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/roles/zsh/tasks/main.yml) と [roles/neovim/tasks/main.yml](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/roles/neovim/tasks/main.yml) を読む  
   外部ダウンロード、テンプレート展開、ユーザー切り替え、headless 実行など、少し応用的なパターンが出てきます。

## 5. task はどう読めばよいか

Ansible の task は、見慣れないと YAML の塊に見えますが、読む観点はかなり固定です。1 task につき、次の順で見れば十分です。

1. `name`  
   その task が何を意図しているかを見る。

2. module 名  
   `apt`、`file`、`template`、`git`、`command`、`shell` のどれかを見る。

3. 引数  
   何を対象にし、どの状態を目指しているかを見る。

4. 実行条件  
   `when`、`become`、`become_user`、`environment`、`creates`、`changed_when` を見る。

5. 後続への受け渡し  
   `register` の有無を見る。後続 task で参照されるかを確認する。

たとえば `apt` task は、「何を入れるか」だけでなく、「キャッシュ更新をするか」「ロック待機をどうするか」まで含めて読む必要があります。`shell` task なら、「なぜ module ではなく shell なのか」を逆に疑うのがよい読み方です。

## 6. 初学者が最初に覚えるべき原則

### idempotent に書く

Ansible の task は、**2 回実行しても壊れず、必要な差分だけ入る**のが理想です。これを idempotent と呼びます。

たとえば `apt` で `state: present` を使うのは、「まだ入っていなければ入れる、入っていれば何もしない」という挙動を得るためです。これが単なる `shell: apt install -y ...` との違いです。

### 可能な限り module を使う

`shell` と `command` は強いですが、強いぶん雑に書けます。雑に書くと再実行性が下がり、`changed` 判定も不正確になります。

原則として:

- パッケージは `apt`
- ファイル配置は `template` / `copy`
- ディレクトリ作成は `file`
- Git clone は `git`
- ユーザー変更は `user`

を優先します。

### variable に寄せる

バージョン番号やユーザー名を task に直書きすると、後で変更点を追いにくくなります。変更可能なものは `group_vars` や `defaults` に寄せる方が保守しやすいです。

### shell を使うなら理由を持つ

Ansible で `shell` が必要になるのは、module では表現しにくい処理を行うときです。たとえば installer script を流す、複数コマンドの待機ループを書く、特殊な CLI を叩く、という場面です。

このときは、`creates` や `changed_when` を併用して、再実行時の振る舞いを明確にする必要があります。

## 7. 学習中によく使う実行オプション

Ansible は、いきなり本番実行するより、オプション付きで挙動を観察しながら学ぶ方が理解しやすいです。

```bash
ansible-playbook playbooks/site.yml -K
ansible-playbook playbooks/site.yml -K --tags common
ansible-playbook playbooks/site.yml -K --check --diff
```

- `-K` は `become` 用の sudo パスワード入力です
- `--tags common` は特定 role / task 群だけ実行したいときに使います
- `--check` は dry-run です
- `--diff` は template 変更時の差分確認に便利です

初学者は、まず `common` だけを対象にして task の流れを追うと理解しやすいです。`zsh` や `neovim` は外部依存が多く、最初の教材としては少し複雑です。

## 8. よくあるつまずき

### 変数が未定義になる

Ansible は、どのタイミングで変数が評価されるかが重要です。`group_vars` の自動読込、`vars_files` の指定、`set_fact` の実行順によって、見えている変数が変わります。

今回の `ubuntu_dev` でも、`pre_tasks` で使う変数が play 開始直後に未解決になり、`undefined variable` で落ちる問題がありました。Ansible のエラーは雑に見えても、**どの task の、どの引数の評価で落ちたか**を読むとかなり具体的です。

### 権限と対象ユーザーが噛み合わない

`become: true` は root 実行への切り替えであり、`become_user` はその先の実行ユーザー切り替えです。`ubuntu_dev_user` が正しく決まっていないと、ホームディレクトリや配置先が壊れます。

WSL では `SUDO_USER` や `USER` の値が実行方法によって変わるので、ユーザー解決は早い段階で明示的に確定させる方が安全です。

### パッケージマネージャーのロックで失敗する

Ubuntu / WSL の初回セットアップ直後は、`unattended-upgrades` が `dpkg` ロックを持っていることがあります。これは playbook が悪いというより、OS 初期化と競合している状態です。

そのため `ubuntu_dev` では、`apt` 実行前に package manager の busy 状態を出力し、一定時間待機してから進むようにしています。

### 外部ダウンロードに失敗する

`get_url` や installer script は、ネットワーク、URL 変更、アーキテクチャ不一致の影響を受けます。`ansible_architecture` やバージョン変数がどう参照されているかを確認する必要があります。

## 9. このリポジトリで Ansible を変更するときの考え方

学習用に読むだけでなく、実際に変更するなら、どこを正本として扱うかを意識する必要があります。

- バージョンやフラグの変更は [all.yml](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/group_vars/all.yml) に寄せる
- 共通処理は [roles/common/tasks/main.yml](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/roles/common/tasks/main.yml) に寄せる
- zsh / Neovim の設定ファイル本文は `templates/` 配下を正本にする
- 仕様変更を入れたら、対応する Linux ドキュメントも一緒に更新する

最後の点は特に重要です。自動化コードだけが変わって文書が古いままだと、学習用資料としても運用資料としてもすぐに役に立たなくなります。

## 10. 学習順のおすすめ

Ansible 初学者がこのリポジトリで学ぶなら、次の順がよいです。

1. この文書を読む  
   用語と全体像を先に揃えます。

2. [README.md](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/README.md) を読む  
   何を構築する playbook かを把握します。

3. [site.yml](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/playbooks/site.yml) を読む  
   入口の構造を掴みます。

4. [roles/common/tasks/main.yml](/C:/Users/n4304/Documents/Learn/dojo/infra/ansible/ubuntu_dev/roles/common/tasks/main.yml) を task 単位で読む  
   module の基礎を学びます。

5. `--tags common` や `--check --diff` で挙動を観察する  
   YAML と実行結果を対応づけます。

6. その後に `zsh` と `neovim` の role を読む  
   より実践的な構成へ進みます。

## 結論

Ansible を学ぶときは、YAML の書き方そのものより、**状態を揃えるとはどういうことか**、**role と variable で責務をどう分けるか**、**再実行しても壊れない形でどう書くか**を先に理解するべきです。

このリポジトリの `ubuntu_dev` は、Ubuntu / WSL の開発環境構築を題材に、Ansible の基本を学ぶ教材としても使えます。まずは `inventory`、`playbook`、`role`、`task`、`module` の関係を読み解き、そのうえで `common` role のような単純なところから実行結果と結びつけて理解すると、無理なく身につきます。
