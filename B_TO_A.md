# bemaniutils to asphyxia

You can use this tool to convert bemaniutils data to asphyxia data.

## Usage

0. Already played SDVX with Asphyxia.

1. Write a config.toml

```
[asyphyxia]
record_path = "a new file to hold the data"
refid = "the refid generate by Asphyxia"
[bemaniutils]
db_address = "localhost"
db_name = "bemani"
db_password = "bemani"
db_port = 3306
db_user = "bemani"
game_version = 6
username = "username on bemaniutils server"
```

2. Run the tool

```
bemaniutils_to_asphyxia -c config.toml
```

3. Copy the records data in your `record_path` file to Asphyxia's savedata.db.

One record is like:

```
{"collection":"music","mid":1068,"type":4,"score":9914182,"clear":3,"grade":10,"__refid":"AB973E24894A6D58","_id":"35","buttonRate":0,"longRate":0,"volRate":0,"createdAt":{"$$date":1648522796801},"updateAt":{"$$date":1648522796801},"__a":"sdvx@asphyxia","__s":"plugins_profile"}
```