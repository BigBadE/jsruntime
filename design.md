Ideas for running lots of scripts

1. Use unbound scripts/templates to fast load library
MODULES

2. Scripts are run in the same process but with different contexts,
the process handles sending data to/from scripts that is put into shared memory,
without blocking
(Easily done with different contexts/isolates, won't have
threading issues)

3. Communicate using shared memory 
(Prevents struggles with timing, overflowing, etc...)
(Need to keep track of the max memory usable, might be hard with images)