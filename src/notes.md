try multiple-entrant read of stl file
auto-record benchmark values
try optimizing vec by setting estimated initial capacity
make sure to run benchmarks in release mode, to get fully optimized
rewrite the stl parser as an iterator over triangles
dedupe vertices to make a mesh
store it as an adjacency matrix?
handle binary stl files
-add indexing during parsing so we know where it failed.- bonus points: provide file context
add a way to measure progress?