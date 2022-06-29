# CSVquel

run sql on csvs lol


## Install (Homebrew MacOS)

```sh
brew tap jharrilim/csvquel 'https://github.com/jharrilim/csvquel'
brew install jharrilim/csvquel/cql
```

## Try it out

From within this repo:

```sh
cql -f ./test.csv 'select id from test where id = 1 or id = 2'
```

---


The evaluator and parser are only partially implemented, but implemented enough to run a basic query.
