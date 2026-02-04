#!/bin/bash

# Run all benchmarks in single-threaded mode
RAYON_NUM_THREADS=1 cargo bench "$@"
