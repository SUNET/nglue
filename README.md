# NGLUE

Small glue code to pass data from Nagios to a web API.

## LICENSE: MIT


## Installation

Install Rust from [rustup](https://rustup.rs).

```
cargo bulid --release
sudo cp ./target/release/nglue /usr/local/bin/
```


## Usage

Example call with URL in environment.

`NGLUE_SERVER="http://localhost:8000/" ./target/release/nglue --debug --servicestate OK --description 'Current Users USERS OK - 0 users currently logged in' --hostname 'localhost' --servicestateid '0' --lastservicestateid '0' --lastproblemid '0' --problemid '0' --notification 'YES' --attempt_number 1 --max_attempts 4`
