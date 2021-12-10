# ダジャレ検出Bot


(旧ダジャレ検出Bot)[https://github.com/sizumita/pun-discord-bot] の後継Botです。

Discordで動くBotと、サブモジュールとしてダジャレを検出するinterfaceを擁するdetectorがあります。

現在はルールベースの検出のみをおこなっています（速度的な問題で）。

ここをこうした方がいいよ、こうすると検出量が上がるよなどのissueやPRは大歓迎です。

# 動かし方

docker | docker-composeに対応しています。

生で動かしたい場合は、`cargo build --release --bin dajare`をしてから、同じディレクトリにneologd/make_ipadic.shを動かしてできたlindera-ipadicディレクトリを配置してください。

# 参考にしている文献

https://vaaaaaanquish.hatenablog.com/entry/2020/12/11/122721#f-d9d09a9b
