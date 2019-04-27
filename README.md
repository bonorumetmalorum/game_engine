[![Build Status](https://travis-ci.com/bonorumetmalorum/game_engine.svg?token=L25q4BBpBAoZ4k9LTWsW&branch=master)](https://travis-ci.com/bonorumetmalorum/game_engine)

###Non-concurrent ECS
This version of the ECS does not contain any concurrency features. It is being used to create a physics simulation as it does not suffer from deadlocks.

This version of the code is what was used to benchmark against C++.

To run the unit tests use the command "cargo test"
To run the clock count benchmarks use the command "cargo bench"
-to view the report the index.html file is located at "/ecs/target/criterion/report/index.html"
To run the cache benchamrks use the command "cargo perf" *

finally to run the physics simulation, which at this point in time is exteremly simple change directory to /phys and "cargo run --release"


*In order to run these benchmarks use the command "sudo sh -c 'echo -1 >/proc/sys/kernel/perf_event_paranoid'" to unlock the hardware counters for your system
Furthermore, you must have the PAPI development headers available on your system for successful compilation. In order to install these run the shell script located under "/performance/papi_install.sh"

