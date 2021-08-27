# ruborute [WIP]

**Are you 暴龍天 ?**. The ruborute is a interactive command-line tool to get asphyxia@sdvx gaming data.

asphyxia-core/plugins: https://github.com/asphyxia-core/plugins

## Usage

```bash
ruborute --user={user_id} --record={path-to-recorddb} --music={path-to-musicdb}
# implemented commands now:
>> help
+--------+-------------------+------------------------------+
| name   | usage             | description                  |
+--------+-------------------+------------------------------+
| help   | help              | show the help information    |
+--------+-------------------+------------------------------+
| record | record <music-id> | get music record by music id |
+--------+-------------------+------------------------------+
>> record 229
get records of music: Booths of Fighters...
+----------+--------------------+------------+-------+---------+
| music id | music name         | difficulty | level | score   |
+----------+--------------------+------------+-------+---------+
| 229      | Booths of Fighters | EXH        | 17    | 9600467 |
+----------+--------------------+------------+-------+---------+
| 229      | Booths of Fighters | HVN        | 19    | 9278005 |
+----------+--------------------+------------+-------+---------+
```

You can type Ctrl-C or Ctrl-D to exit.

