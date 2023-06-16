Design Goals
============
The CPU-N1 ISA's overall goal is to help people learn to write
a new CPU backend for the LLVM compiler.  To that end, the ISA
strives to follow these design rules:

Pedagogical Value
------------------
First and foremost, the ISA details should be suitable for helping
LLVM newbies learn the basics of adding a new backend.

Extensibility
------------------
The ISA should allow incremental additions / changes, to support
an incremental / spiral learning style for LLVM backend.

Simplicity of the ISA *and* Implementation
------------------------------------------
Both the ISA, and this repository's source code, should be easily
understandable for new users.  For example, even if the ISA is extended
beyond its simple core, new users should still be able to quickly
understand the details and implementation of the minimal, core ISA.


