# ruborute

**Are you 暴龍天 ?**. The ruborute is an interactive command-line tool to get your SDVX gaming data.

Supported gaming server:

- asphyxia@sdvx: https://github.com/asphyxia-core/plugins
- bemaniutils: https://github.com/DragonMinded/bemaniutils

This project also contains another tool [bemaniutils_to_asphyxia](./B_TO_A.md), which is to convert bemaniutils data to asphyxia data. Click the URL for detail.

## Usage

The most recommend usage:

```shell
# there is an example config.toml in the dir exmaple/
ruborute -c config.toml 
```

Arguments information can be found by:

```shell
ruborute --help
```

After launching: 

```
$ ruborute -c config.toml
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
| count  | count <all | level>            | count the grades of one level(or all)      |
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
....
+------+----------+----------------+------------+-------+---------+-------+------------+----------+
50 record(s) founded.
>> vf
Your Volforce: 17.714
>> count all
+-------+----+------+-----+-----+----+-----+-----+--------+
| level | S  | AAA+ | AAA | PUC | UC | HC  | NC  | played |
+-------+----+------+-----+-----+----+-----+-----+--------+
| 16    | 14 | 26   | 21  | 0   | 4  | 66  | 1   | 72     |
+-------+----+------+-----+-----+----+-----+-----+--------+
| 17    | 31 | 73   | 147 | 0   | 3  | 458 | 113 | 574    |
+-------+----+------+-----+-----+----+-----+-----+--------+
| 18    | 1  | 20   | 42  | 0   | 0  | 123 | 207 | 352    |
+-------+----+------+-----+-----+----+-----+-----+--------+
| 19    | 0  | 0    | 0   | 0   | 0  | 1   | 19  | 42     |
+-------+----+------+-----+-----+----+-----+-----+--------+
| 20    | 0  | 0    | 0   | 0   | 0  | 0   | 1   | 8      |
+-------+----+------+-----+-----+----+-----+-----+--------+
>> count 17
+-------+----+------+-----+-----+----+-----+-----+--------+
| level | S  | AAA+ | AAA | PUC | UC | HC  | NC  | played |
+-------+----+------+-----+-----+----+-----+-----+--------+
| 17    | 31 | 73   | 147 | 0   | 3  | 458 | 113 | 574    |
+-------+----+------+-----+-----+----+-----+-----+--------+
```

You can type Ctrl-D to exit.

## Features

- [x] Get music play records by music id.
- [x] Get music play records by music name (fuzzy search supported).
- [x] Compute VF.
- [x] Get the best 50 records.
- [x] Collect more detail statistics (Such as count of a clear type).
- [x] Press "Tab" button to complete the commands.
- [x] History hints supported.
- [x] Type Ctrl-C to interrupt current input.
- [ ] Range get records in VF order.
- [ ] Get music infomation by music id.
- [ ] Get music informaton by music name.
- [ ] Improve the interactivity.
