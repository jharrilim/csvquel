# CSVquel

run sql on csvs lol


## Try it out

From within this repo:

```sh
cargo run -- -f ./test.csv 'select id from test where id = 1 or id = 2'
```

---

The evaluator and parser are only partially implemented, but implemented enough to run a basic query.
