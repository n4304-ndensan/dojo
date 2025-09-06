# Super SSH

## 問題情報

- **Author:** Jeffery John
- **Description:**  
  Using a Secure Shell (SSH) is going to be pretty important.
  Additional details will be available after launching your challenge instance.
  Can you ssh as ctf-player to titan.picoctf.net at port 54405 to get the flag? You'll also need the password 1db87a14. If asked, accept the fingerprint with yes. If your device doesn't have a shell, you can use: https://webshell.picoctf.org If you're not sure what a shell is, check out our Primer: https://primer.picoctf.com/#_the_shell

---

## 解法

1. ターミナルまたは WebShell を開く。
2. 次のコマンドを実行する：

   ```bash
   ssh ctf-player@titan.picoctf.net -p 54405
   ```

3. 初回接続時に表示される fingerprint の確認には `yes` と入力する。

   - fingerprint はサーバの公開鍵のハッシュ値で、接続先の正当性を確認するためのもの。
   - 同じサーバに再接続する場合は表示されない。

4. パスワードを求められるので `1db87a14` を入力。
5. ログイン後にフラグが表示される。

---

## 関連技術

- SSH
- サーバフィンガープリント
