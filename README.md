# SurrealDB Example 2

Run via:

```bash
surreal start --log info --user root --pass root memory
surreal sql --conn http://localhost:8000 --user root --pass root --ns carltonj2000 --db db1 --pretty
```

## SQL

```sql
create person set name='carlton';
create person set name='jeffrey';
create person set name='cheryl';
select * from person;
update person:<id> set project=[this:one, and:the, next:one]
```

## Creating History

```bash
cargo new rust-surrealdb-ex2
sudo apt install clang
cargo add tokio --features=full
```

## Code History

The code in this repository is based on the 
[Embedded SurrealDB experiments](https://youtu.be/_EAQDgu-z8Q)
videos.
