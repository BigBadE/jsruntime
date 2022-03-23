Ideas for running lots of scripts

1. Libraries cannot use global variables, 
that way they only need to be loaded/initialized once.
Most code will be located inside those libraries.
(Maybe impossible, but might be doable with templates)

2. Scripts are run in the same process but with different contexts,
the process handles sending data to/from scripts that is put into shared memory,
without blocking
(Easily done with different contexts/isolates, won't have
threading issues)

3. Communicate using shared memory 
(Prevents struggles with timing, overflowing, etc...)
(Need to keep track of the max memory usable, might be hard with images)