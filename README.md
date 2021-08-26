# asphyxia-rsdvx [WIP]

asphyxia-rsdvx is a interactive command-line tool to get asphyxia@sdvx gaming data.

asphyxia-core/plugins: https://github.com/asphyxia-core/plugins

## Usage

```bash
asphyxia-rsdvx --user={user_id} --db={path-to-db}

>> help
+---------+----------------+--------------------------------+
| Command | Usage          | Description                    |
+---------+----------------+--------------------------------+
| help    | help           | show the help information      |
+---------+----------------+--------------------------------+
| get     | get <music-id> | find music records by music id |
+---------+----------------+--------------------------------+
>> get 1234
+----------+----------+
| music id | score    |
+----------+----------+
| 1234     | 10000000 |
+----------+----------+
```

You can type Ctrl-C or Ctrl-D to exit.

