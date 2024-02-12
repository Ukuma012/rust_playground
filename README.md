# 機能

- POST /login： セッションに user_id のデータがなければ user_id を作成して、セッションにセットする
- PATCH /change： 新規に user_id を作成し、セッションの user_id と置き換える
- POST /add： セッションの count に 1 を追加する
- DELETE /reset： セッションの count を 0 にする
