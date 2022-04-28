# tsd-rs

timestamp diff rust

This tool is mainly used for time correction of timestamps in perf commands.

``` bash
sudo perf sched timehist | perl -pe 'BEGIN{$offset=shift} s/^ *([0-9]+.[0-9]+)/$1+$offset/e' $TIMESTAMP_OFFSET
```

## how to install
``` bash
cargo install --git https://github.com/umaumax/tsd-rs
```

## how to use
``` bash
$ tsd-rs --verbose
monotonic now:    1540307.715213000s
realtime  now: 1651117273.027979000s
1649576965.312766000
```
