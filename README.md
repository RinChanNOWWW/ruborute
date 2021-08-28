# ruborute [WIP]

**Are you 暴龍天 ?**. The ruborute is a interactive command-line tool to get asphyxia@sdvx gaming data.

asphyxia-core/plugins: https://github.com/asphyxia-core/plugins

## Usage

```bash
$ ruborute --user={user_id} --record={path-to-recorddb} --music={path-to-musicdb}
xxx music loaded.
your play data has been loaded.
you have xxx records.
# implemented commands now:
>> help
+--------+--------------------------------+-------------------------------------------+
| name   | usage                          | description                               |
+--------+--------------------------------+-------------------------------------------+
| help   | help                           | show the help information.                |
+--------+--------------------------------+-------------------------------------------+
| record | record <music-id | music-name> | get music record by the music id or name. |
+--------+--------------------------------+-------------------------------------------+
>> record 229
Music 229: <Booths of Fighters>
+----------+--------------------+------------+-------+---------+
| music id | music name         | difficulty | level | score   |
+----------+--------------------+------------+-------+---------+
| 229      | Booths of Fighters | EXH        | 17    | 9600467 |
+----------+--------------------+------------+-------+---------+
| 229      | Booths of Fighters | HVN        | 19    | 9278005 |
+----------+--------------------+------------+-------+---------+
2 record(s) founded.
>> record booths of fighters
+----------+--------------------+------------+-------+---------+
| music id | music name         | difficulty | level | score   |
+----------+--------------------+------------+-------+---------+
| 229      | Booths of Fighters | EXH        | 17    | 9600467 |
+----------+--------------------+------------+-------+---------+
| 229      | Booths of Fighters | HVN        | 19    | 9278005 |
+----------+--------------------+------------+-------+---------+
2 record(s) founded.
>> 
```

You can type Ctrl-C or Ctrl-D to exit.

## Features

- [x] Get music play records by music id.
- [x] Get music play records by music name (fuzzy search supported).
- [ ] Get music infomation by music id.
- [ ] Get music informaton by music name.
- [ ] Compute VF.
- [ ] Get the best 50 records.
- [ ] Range get records in VF order.
- [ ] Collect more detail statistics (Such as the clear type).
