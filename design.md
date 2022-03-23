Ideas for running lots of scripts

1. Libraries cannot use global variables, 
that way they only need to be loaded/initialized once.
Most code will be located inside those libraries.

2. Scripts are run in the same process but with different contexts,
the process handles sending data to/from scripts that is put into shared memory,
without blocking 

4. 