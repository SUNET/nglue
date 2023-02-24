# NGLUE

Small glue code to pass data from Nagios to a web API.

## LICENSE: MIT


## Installation

First build the Docker container for building.

```
docker build -t nglue-builder .
```

Then run the container once with the source directory mounted as `/io`.

```
docker run --rm -v $PWD:/io:Z nglue-builder
```

Now we have the binary at `./target/release/nglue`, copy it in the same container of Nagios.


## Usage

Example call with URL in environment.

`NGLUE_SERVER="http://localhost:8000/" ./target/release/nglue --debug --servicestate OK --description 'Current Users USERS OK - 0 users currently logged in' --hostname 'localhost' --servicestateid '0' --lastservicestateid '0' --lastproblemid '0' --problemid '0' --notification 'YES' --attempt_number 1 --max_attempts 4`
