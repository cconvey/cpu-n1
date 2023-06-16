* Implement N1 ISA (first draft)

  * Start build-requirments doc (in README.md ?)
  * Initial .rst docs support

    * Create skeleton CMake file.
    * Document the ISA in a .rst file.a

      * Convert existing ISA doc to .rst for better table support.
      * Add .rst support to build system (and build-requirements doc).
      * Get the per-instruction table formatted correctly.

    * Get GitHub project page looking right. (May require .rst --> .html/.md checked into repo.)

  * Add end-to-end N1 ISA support for one instruction

    * create the trivial executable (a do-nothing Rust cmdline program)

    * support a single instruction "Foo0" (this doc's placeholder).

      * add "Foo0" support to the simulator
      * add "Foo0" support to LLVM

        * fork LLVM
        * add CPU-N1 as a trivially supported LLVM target
        * add asm support

          * parsing
          * printing

        * add asm --> MC support
        * demonstrate asm --> simulated execution of "Foo0"
        * update project documentation

  * Add end-to-end support for the rest of the ISA.

* Extend ISA to support text input / output.

  * See how this is usually done with embedded-system targets.
  * Add support to the ISA / sim / ABI.
  * Get arbitrary console programs working correctly.

