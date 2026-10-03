#!/usr/bin/env sh
# Create the synthetic people used by the example fixtures.
set -e
biomarker person add alex --name "Alex Example" --sex male --dob 1984-06-01 --tag example
biomarker person add sam --name "Sam Sample" --sex female --dob 1991-09-23 --tag example
