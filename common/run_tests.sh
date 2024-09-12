#!/bin/bash

# At least at the time of writing this, the test-threads=1 is required to prevent the test from failing, because they can run in parallel.
cargo test -- --test-threads=1