- Malloc implementations
  - jemalloc
  - tcmalloc
  - dlmalloc
  - ralloc
  - ptmalloc2
  - libumem
  - nedmalloc
  - https://github.com/mjansson/rpmalloc
  - make an independent crate
  - lockless https://locklessinc.com/downloads/
  - https://github.com/emeryberger/Malloc-Implementations/
  - http://hoard.org/ 
  - https://github.com/emeryberger/DieHard
  - https://github.com/ezrosent/allocators-rs
  - https://github.com/GrapheneOS/hardened_malloc

- benchmarking
  - http://www.highlandsun.com/hyc/malloc/
  - https://locklessinc.com/benchmarks_allocator.shtml
  - https://locklessinc.com/downloads/t-test1.c
  - https://github.com/f18m/malloc-benchmarks

- metrics and tuning
  - https://www.gnu.org/software/libc/manual/html_node/Unconstrained-Allocation.html#Unconstrained-Allocation

- important metrics
  - allocation / deallocation time, median and max
  - allocated size, unallocated size

- docs
  - https://gperftools.github.io/gperftools/tcmalloc.html
    - this one has metric docs
  - https://gperftools.github.io/gperftools/index.html
  - http://goog-perftools.sourceforge.net/doc/tcmalloc.html
  - http://pages.cs.wisc.edu/~danb/google-perftools-0.98/tcmalloc.html

- todo
  - tcmalloc needs to build libunwind, libgcc will deadlock
    - per https://github.com/gperftools/gperftools/blob/master/INSTALL#L33

- ideas
  - heap checker https://github.com/gperftools/gperftools/blob/master/src/gperftools/heap-checker.h
  - heap profiler https://github.com/gperftools/gperftools/blob/master/src/gperftools/heap-profiler.h
  - do the things pprof does - mprof
  - t-test1 comparisons

