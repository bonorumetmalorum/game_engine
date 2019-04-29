[![Build Status](https://travis-ci.com/bonorumetmalorum/game_engine.svg?token=L25q4BBpBAoZ4k9LTWsW&branch=master)](https://travis-ci.com/bonorumetmalorum/game_engine)

##Concurrent ECS
to run the clock count benchmarks in this version of the ECS run "cargo bench" in the /ecs directory*
-to view the report the index.html file is located at "/ecs/target/criterion/report/index.html"

to run the cache benchmarks in this version of the ECS run "cargo run" in the /performance directory*

to run the unit tests in this version of the ECS run "cargo test" in the /ecs directory



*It is not recommended to run these benchmarks as they are outdated, for up to date benchmarks please consult the non-concurrent version of this ECS
to run the benchmarks you must	have the PAPI development headers installed, to do this run the shell script under "/performance/install_papi.sh".
Once installed the hardware counters must be unlocked, to unlock them run the following command "sudo sh -c 'echo -1 >/proc/sys/kernel/perf_event_paranoid'"

