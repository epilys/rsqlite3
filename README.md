# sqlite3 Rewritten in RiiR Rust 🦀🦀🦀

Finally, one of the best written software paired with one of the best writable programming language‽ Fearless and memory safe, since the uncountable amount of `unsafe {}` blocks makes you not care anymore.

## Build and run

```shell
$ cargo +nightly build --release
# ... bunch of stuff I choose not to understand/read
warning: `rsqlite3` (bin "rsqlite3") generated 37 warnings (24 duplicates)
    Finished release [optimized] target(s) in 39.84s
$ ./target/release/rsqlite3
-- Loading resources from /home/epilys/.sqliterc
SQLite version 3.37.0 2021-10-17 10:31:09
Enter ".help" for usage hints.
Connected to a transient in-memory database.
Use ".open FILENAME" to reopen on a persistent database.
sqlite> .open rrrrrruuuuuust.db
sqlite> CREATE TABLE opinions(x);
sqlite> insert into opinions(x) values (hex(randomblob(16)));
sqlite> insert into opinions(x) values (hex(randomblob(16)));
sqlite> insert into opinions(x) values (hex(randomblob(16)));
sqlite> insert into opinions(x) values (hex(randomblob(16)));
sqlite> select * from opinions;
x
--------------------------------
343D4BE24D07A96F8550B0942F664A6C
D6289536E4A8057EB44754358EACD31A
B4CA8E714CB57B11E7336263D214F30F
A6491CA289ABF90EB2D76F5E1F919272
sqlite> PRAGMA journal_mode = wal;
journal_mode
------------
wal
sqlite> PRAGMA integrity_check;
integrity_check
---------------
ok
sqlite> VACUUM;
sqlite> ^D
```

## Wait, what?

This is just the sqlite3 code passed throught the
[`c2rust`](https://c2rust.com/) transpiler, along with some cleanup from me.
I've found no way to hook it to sqlite3's testing harnesses yet so I doubt it's
completely functional; but opening the shell seems to work.

I am suspicious that a lot of code under ifdefs or not is lost through the transpilation, because the binary size difference is substantial:

```shell
$ du -sh $(which sqlite3)
9.5M    /home/epilys/.local/bin/sqlite3
$ du -sh target/release/rsqlite3
5.1M    target/release/rsqlite3
```
