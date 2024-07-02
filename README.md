# ICFPC2024 (Pigimarl)

- `comm_app`: A simple wrapper for communicating with the ICFPC 2024 server.
- `common`: Utilities for problem-solving, including a transpiler from Python to ICFP language and mutual conversion between human-readable strings/integers and ICFP language form.
- `efficiency`: Solvers for efficiency levels 6 to 10, which require z3 to solve constraints.
- `lambdaman`:
    - `rand`: RNG seed searcher.
    - `*.py`s: Programs that solve the corresponding lambdaman instances (when transpiled into ICFP language).
- `lang`: An interpreter for ICFP language.
- `problems`: Input files.
- `solver`, `spaceship`: Solvers for the spaceship problem. Most of them are based on simple greedy strategy.
- `visualizer`: Visualizers for 3 problems (3D, spaceship, lambdaman) and the AST of ICFP language.