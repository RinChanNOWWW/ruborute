# ruborute

**Are you 暴龍天 ?**. The ruborute is a interactive command-line tool to get asphyxia@sdvx gaming data.

asphyxia-core/plugins: https://github.com/asphyxia-core/plugins

## Usage

Recommend: write `ruborute --user={user_id} --record={path-to-recorddb} --music={path-to-musicdb}` into a cmd or powershell script.

```
$ ruborute --user={user_id} --record={path-to-recorddb} --music={path-to-musicdb}
xxx music loaded.
your play data has been loaded.
you have xxx records.
# implemented commands now:
>> help
+--------+--------------------------------+--------------------------------------------+
| name   | usage                          | description                                |
+--------+--------------------------------+--------------------------------------------+
| help   | help                           | show the help information.                 |
+--------+--------------------------------+--------------------------------------------+
| record | record <music-id | music-name> | get music record by the music id or name.  |
+--------+--------------------------------+--------------------------------------------+
| best50 | best50                         | get the best 50 records in volforce order. |
+--------+--------------------------------+--------------------------------------------+
| vf     | vf                             | compute and print your volforce.           |
+--------+--------------------------------+--------------------------------------------+
>> record 1226
Music 1226: <Black night>
+----------+----------------+------------+-------+---------+-------+------------+----------+
| music id | music name     | difficulty | level | score   | grade | clear type | volforce |
+----------+----------------+------------+-------+---------+-------+------------+----------+
| 1226     | Black night    | MXM        | 18    | 9816513 | AAA+  | HC         | 18.383   |
+----------+----------------+------------+-------+---------+-------+------------+----------+
1 record(s) founded.
>> record bof
+----------+--------------------+------------+-------+---------+-------+------------+----------+
| music id | music name         | difficulty | level | score   | grade | clear type | volforce |
+----------+--------------------+------------+-------+---------+-------+------------+----------+
| 229      | Booths of Fighters | EXH        | 17    | 9600467 | AA+   | Complete   | 15.831   |
+----------+--------------------+------------+-------+---------+-------+------------+----------+
| 229      | Booths of Fighters | HVN        | 19    | 9278005 | A+    | Crash      | 8.020    |
+----------+--------------------+------------+-------+---------+-------+------------+----------+
2 record(s) founded.
>> best50
+------+----------+----------------+------------+-------+---------+-------+------------+----------+
| rank | music id | music name     | difficulty | level | score   | grade | clear type | volforce |
+------+----------+----------------+------------+-------+---------+-------+------------+----------+
| #1   | 1226     | Black night    | MXM        | 18    | 9816513 | AAA+  | HC         | 18.383   |
+------+----------+----------------+------------+-------+---------+-------+------------+----------+
| #2   | 551      | Firestorm      | EXH        | 18    | 9813581 | AAA+  | HC         | 18.378   |
+------+----------+----------------+------------+-------+---------+-------+------------+----------+
| #3   | 1300     | REDO the NIGHT | GRV        | 18    | 9812641 | AAA+  | HC         | 18.376   |
+------+----------+----------------+------------+-------+---------+-------+------------+----------+
| #4   | 1139     | Decoy          | MXM        | 17    | 9929078 | S     | HC         | 18.077   |
+------+----------+----------------+------------+-------+---------+-------+------------+----------+
| #5   | 933      | 見世物ライフ     | MXM        | 17    | 9925958 | S     | HC         | 18.072   |
+------+----------+----------------+------------+-------+---------+-------+------------+----------+
....
+------+----------+----------------+------------+-------+---------+-------+------------+----------+
50 record(s) founded.
>> vf
Your Volforce: 17.714
```

You can type Ctrl-C or Ctrl-D to exit.

## Features

- [x] Get music play records by music id.
- [x] Get music play records by music name (fuzzy search supported).
- [x] Compute VF.
- [x] Get the best 50 records.
- [ ] Range get records in VF order.
- [ ] Get music infomation by music id.
- [ ] Get music informaton by music name.
- [ ] Collect more detail statistics (Such as count of a clear type).
