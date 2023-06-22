CPU-N1 ISA
==========

ISA Parameters
--------------

The specific ISA is a function of these parameters:

+--------------------+------------------------------------------------------+
| Parameter          | Description                                          |
+====================+======================================================+
| *word-size*        |  The number of bits constituting a machine word.     |
|                    |  Note: This must be large enough to store all of     |
|                    |  the flags defined for the ``%status`` register.     |
+--------------------+------------------------------------------------------+
| *instr-size*       |  The number of bytes used to encode a single         |
|                    |  program instruction. *All* machine instructions     |
|                    |  (including operands) have this size.                |
|                    |                                                      |
|                    |  The encoding scheme for each assembly instruction   |
|                    |  is described below. Note that instructions that can |
|                    |  be encoded with fewer bytes than this parameter     |
|                    |  indicates might use padding to reach this size.     |
+--------------------+------------------------------------------------------+
| *num-gp-regs*      |  The number of general-purpose registers.            |
+--------------------+------------------------------------------------------+

Memory
------

-  The ISA uses a flat, non-virtual memory model.
-  Memory addresses are *word-size* in size.
-  Each memory address names a single byte. (Instructions that operate
   on multi-byte regions of memory will generally indicate that range by
   the lowest-address byte in the range.)
-  The memory model does *not* have the concept of pages, nor of memory
   protections.
-  There is no requirement that memory read/write instructions specify
   word-aligned addresses.

Little-endian Encoding
----------------------

Data representation is *little-endian*: Multi-byte numerical values are
stored in memory according to this format:

========================= =============================================
Memory address (in bytes) Fragment of number’s bit-level representaiton
========================= =============================================
*n*                       least-significant byte
*n+1*                     second-from-least-significant byte
…                         …
========================= =============================================

For example, if the number ``0xABCD`` is written to (starting) memory
address ``0x100000000``, then the content of memory would look like
this:

============== =======
Address        Value
============== =======
``0x10000000`` ``0xD``
``0x10000001`` ``0xC``
``0x10000002`` ``0xB``
``0x10000003`` ``0xA``
============== =======

Numeric Values
--------------

Integers
~~~~~~~~

-  Signed integers use 2’s-complement representation.
-  Unsigned integers have plain binary representaiton.
-  All integer values are *word-size* long.

Non-integers
~~~~~~~~~~~~

-  The ISA has no support for floating-point values.

Concurrency
-----------

-  The ISA assumes that only one CPU instance is running on the system,
   and that the CPU’s instructions are the only reason that memory
   content will change during a given simulation. (I.e., there’s no need
   to support C’s / C++’s concept of “volatile” objects.)

   Therefore, the ISA has no concept of memory barriers.

Exceptions / Interrupts
-----------------------

-  The ISA has no explicit support for exceptional conditions.

-  If any noteworthy conditions arise, the expectation is that the ISA
   simulator will somehow inform its user, rather than modeling such
   exceptions as observable / trappable events within the ISA.

-  Bad instructions fall into two categories:

   - Instructions with well-defined error semantics.  I.e., divide by zero.
     An implementation is required to handle these situations in the
     prescribed manner.

   - Invalid instructions, or instructions with "implementation-defined"
     error conditions.  It's up to the simulator / implementation to
     do something useful in these scenarios.

     Note: this category shouldn't exist in a "proper" microprocessor.
     We allow it here to allow a more simplistic ISA.

Registers
---------

All registers are distinct. The ISA has no concept of addressible
sub-registers. However, some mnemonics for sub-registers are provided
for human-friendly documentation (and perhaps human-friendly assembly
support).

+----------------------------+----------------------+---------------+----------------------------------------------------------------------------------------+
| Mnemonic                   | Size                 | Initial value | Description                                                                            |
+============================+======================+===============+========================================================================================+
| ``%pc``                    | *word-size*          | 0             | Program counter. Always holds the memory                                               |
|                            |                      |               | address  of the instruction that will execute                                          |
|                            |                      |               | *after* the current one.                                                               |
|                            |                      |               |                                                                                        |
|                            |                      |               | Unless stated otherwise, this is                                                       |
|                            |                      |               | incremented by *instr-size*                                                            |
|                            |                      |               | immediately before the currently loaded                                                |
|                            |                      |               | instruction begins                                                                     |
|                            |                      |               | execution.                                                                             |
|                            |                      |               |                                                                                        |
|                            |                      |               | This may be a read-only register operand                                               |
|                            |                      |               | for most/all instructions,                                                             |
|                            |                      |               | but can only be modified by certain                                                    |
|                            |                      |               | control-flow instructions.                                                             |
+----------------------------+----------------------+---------------+----------------------------------------------------------------------------------------+
| ``%sp``                    | *word-size*          | 0             | Stack pointer. Although its interpretation up to the                                   |
|                            |                      |               | user, it’s intended to support is function calls as                                    |
|                            |                      |               | defined by the system ABI.                                                             |
|                            |                      |               |                                                                                        |
|                            |                      |               | The ISA is designed to support a stack that grows                                      |
|                            |                      |               | toward lower addresses.                                                                |
|                            |                      |               | addresses.                                                                             |
|                            |                      |               |                                                                                        |
|                            |                      |               | For conceptual simplicity, the ISA requires that this                                  |
|                            |                      |               | value has *word-size* alignment.                                                       |
+----------------------------+----------------------+---------------+----------------------------------------------------------------------------------------+
| ``%gp0``,                  | *word-size*          | undefined     | General-purpose registers. How many is                                                 |
| ``%gp1``,                  |                      |               | specified by *num-gp-regs*.                                                            |
| …                          |                      |               |                                                                                        |
+----------------------------+----------------------+---------------+----------------------------------------------------------------------------------------+
| ``%status``                | *word-size*          | see below     | The status word register. The following are defined.                                   |
|                            |                      |               | All other bits are considered reserved and have no                                     |
|                            |                      |               | defined behavior.                                                                      |
+----------------------------+----------------------+---------------+----------------------------------------------------------------------------------------+
| ``%status.cmp``            | 1 bit                | undefined     | The outcome of the most recent comparison instruction.                                 |
+----------------------------+----------------------+---------------+----------------------------------------------------------------------------------------+
| ``%status.overflow``       | 1 bit                | undefined     | Set/cleared by some ops involving math.                                                |
|                            |                      |               |                                                                                        |
|                            |                      |               | "overflow" might be a poor word for some uses, so this part of the ISA may be changed, |
|                            |                      |               | and/or this register might get renamed to something more appropriate.                  |
+----------------------------+----------------------+---------------+----------------------------------------------------------------------------------------+
| ``%status.halt``           | 1 bit                | 1             | ``0`` when the machine is running, ``1`` when it’s                                     |
|                            |                      |               | halted.                                                                                |
|                            |                      |               |                                                                                        |
|                            |                      |               | A program may set this to ``1`` to indicate that is has                                |
|                            |                      |               | run to completion.                                                                     |
+----------------------------+----------------------+---------------+----------------------------------------------------------------------------------------+

Suggested assembly language
---------------------------

Assembly grammar (partial)
~~~~~~~~~~~~~~~~~~~~~~~~~~

TODO: pick a specific regex language, and update the regex's below to rigorously comply.

*hex-word* : Regex ``0x[0-9a-fA-F]+``

    Must be exactly *$word-size* in length, with leading zeros of necessary.

*signed-dec-word* : Regex ``[+-]?[0-9]+s``

    The twos-complement bit pattern (*$word-size* in length)
    of the specified number.
    The number must lie within the valid range.

*unsigned-dec-word* : Regex ``[0-9]+u``

    The unsigned-integer bit pattern (*$word-dize-bytes* in length)
    of the specified number.
    The number must lie within the valid range.

*imm-u* : ``#`` followed by *unsigned-dec-word*

*imm-s* : ``#`` followed by *signed-dec-word*

*imm* : ``#`` followed by ( *signed-dec-word* | *unsigned-dec-word* )

*gp-reg* :  any valid gp register, e.g. ``%gp3``

*reg* : ( ``%pc`` | ``%sp`` | ``%status`` | *gp-reg* )

*w-reg* : ( ``%sp`` | *gp-reg* )

    A register into which most instructions can freely write.

*r-reg* : ( *gp-reg* | ``%sp`` | ``%pc`` )

    A register from which most instructions can freely read.

Assembly instructions
~~~~~~~~~~~~~~~~~~~~~

Conventions for pseudocode used in assembly descriptions:

* *zxdw(...)* - The bit-pattern produced by zero-extending the *word-size* parameter to a
    bit-pattern of size 2 * *word-size*.

* *sxdw(...)* - The bit-pattern produced by sign-extending the *word-size* parameter to a
    bit-pattern of size 2 * *word-size*.

* *a [ b ... c ]* - Given a bit-pattern *a*, return the slice of that bit-pattern
    from bit-number *b* to bit-number *c*, inclusive, with *b* <= *c*.
    The least-significant bit is numbered 0.

* Pseudocode binary operators ("+", "-", etc.) implicitly use the same bit-width as that
  of their operands.  This is why some descriptions also use *zxdw*, *sxdw*, etc.

* In the "Implicit reg. access" column:

    * Some values in "Implicit register access" column mention *subregisters* within ``%status``.
      This is done for documentation clarity.

      I may have read somewhere that LLVM's model of these registers will need to treat the
      modification of *any subregister within* ``%status`` as a modification of the entire
      register.

    * Strictly speaking, *every* instruction implicitly modifies ``%pc``.  For readability,
      we only list ``%pc`` for instructions that modify ``%pc`` in noteworthy ways.

TODO:

* Add bitwise logical and shift operators.

* Better pseudo-code

  * Provide it for all suitable instructions.
  * Format it it well, e.g. with ``.. code-block::``.

.. |            |                    |                                   |                         |                          |                                                                                      |

+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| Mnemonic   | Operand1           | Operand2                          | Operand3                | Implicit reg. access     | Description                                                                          |
+============+====================+===================================+=========================+==========================+======================================================================================+
| ``add``    | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm*        | ``%status.overflow`` (w) | Op1 ← Op2 + Op3                                                                      |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | %status.overflow ← (the result wrapped due to overflow)                              |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``sub``    | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm*        | ``%status.overflow`` (w) | Op1 ← Op2 - Op3                                                                      |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | .. code-block::                                                                      |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          |   Op1 ← Op2 - Op3                                                                    |
|            |                    |                                   |                         |                          |                                                                                      |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``sub``    | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm*        | ``%status.overflow`` (w) | Op1 ← Op2 - Op3                                                                      |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | %status.overflow ← (the result wrapped due to underflow)                             |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``mul``    | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm*        | --                       | .. code-block::                                                                      |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          |    Op1 ← (zxdw(Op2) * zxdw(Op3))[ word-size ... 0 ]                                  |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | Compute the lower-half result of Op2 * Op3.                                          |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | Note that signed/unsigned distinction isn't needed for this lower-half.              |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``mulhss`` | *w-reg*            | *r-reg* \| *imm-s*                | *r-reg* \| *imm-s*      | --                       | Op1 ← (sxdw(Op2) * sxdw(Op3)[ ((2 *  *word-size*) - 1) ... *word-size* ]             |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | Compute the upper-half result of Op2 * Op3.  Assume Op2 and Op3 are signed.          |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``mulhsu`` | *w-reg*            | *r-reg* \| *imm-s*                | *r-reg* \| *imm-s*      | --                       | Op1 ← (sxdw(Op2) * zxdw(Op3)[ ((2 *  *word-size*) - 1) ... *word-size* ]             |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | Compute the upper-half result of Op2 * Op3.  Assume Op2 is signed, Op3 is unsigned.  |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``mulhuu`` | *w-reg*            | *r-reg* \| *imm-s*                | *r-reg* \| *imm-s*      | --                       | Op1 ← (zxdw(Op2) * zxdw(Op3)[ ((2 *  *word-size*) - 1) ... *word-size* ]             |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | Compute the upper-half result of Op2 * Op3.  Assume Op2 and Op3 are unsigned.        |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``dipss``  | *w-reg*            | *r-reg* \| *imm-s*                | *r-reg* \| *imm-s*      | ``%status.overflow`` (w) | Op1 ← (Op2 / Op3) rounded towards zero.                                              |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | If Op3 == 0, Op1's value is undefined.                                               |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | ``%status.overflow`` ← (Op3 != 0)                                                    |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | Integer division.  Assume Op2 and Op3 are signed.                                    |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``divuu``  | *w-reg*            | *r-reg* \| *imm-u*                | *r-reg* \| *imm-u*      | ``%status.overflow`` (w) | Op1 ← (Op2 / Op3) rounded towards zero.                                              |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | If Op3 == 0, Op1's value is undefined.                                               |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | ``%status.overflow`` ← (Op3 != 0)                                                    |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | Integer division.  Assume Op2 and Op3 are unsigned.                                  |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``remss``  | *w-reg*            | *r-reg* \| *imm-s*                | *r-reg* \| *imm-s*      | ``%status.overflow`` (w) | Op1 ← (Op2 % Op3)                                                                    |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | If Op3 == 0, Op1's value is undefined.                                               |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | ``%status.overflow`` ← (Op3 != 0)                                                    |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | Integer division.  Assume Op2 and Op3 are signed.                                    |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``remuu``  | *w-reg*            | *r-reg* \| *imm-u*                | *r-reg* \| *imm-u*      | ``%status.overflow`` (w) | Op1 ← (Op2 % Op3)                                                                    |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | If Op3 == 0, Op1's value is undefined.                                               |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | ``%status.overflow`` ← (Op3 != 0)                                                    |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | Integer division.  Assume Op2 and Op3 are unsigned.                                  |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``and``    | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm*        | --                       | Op1 ← Op2 & Op3                                                                      |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``or``     | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm*        | --                       | Op1 ← Op2 | Op3                                                                      |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``xor``    | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm*        | --                       | Op1 ← Op2 ^ Op3                                                                      |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``not``    | *w-reg*            | *r-reg* \| *imm*                  | --                      | --                       | Op1 ← ~Op2                                                                           |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``lsl``    | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm-u*      |                          | Op1 ← Op2 << Op3                                                                     |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | Compute the logical-left-shift of Op2, shifted by the number of bits indicated by    |
|            |                    |                                   |                         |                          | Op3.                                                                                 |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | If Op3 > *word-size*, the instruction is illegal.                                    |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``rsl``    | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm-u*      |                          | Op1 ← Op2 >> Op3                                                                     |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | Compute the logical-right-shift of Op2, shifted by the number of bits indicated by   |
|            |                    |                                   |                         |                          | Op3.                                                                                 |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | If Op3 > *word-size*, the instruction is illegal.                                    |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``mov``    | *w-reg*            | *r-reg* \| *imm*                  | --                      | --                       | Copy the value Op2 into register Op1.                                                |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``load``   | *w-reg*            | *r-reg* \| *imm*                  | --                      | --                       | Copy the memory value *pointed to by* Op2 into register Op1.                         |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``store``  | *r-reg* \| *imm*   | *r-reg* \| *imm*                  | --                      | --                       | Copy the value Op2 to the memory location pointed to by Op1                          |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``cmpeq``  | *r-reg* \| *imm*   | *r-reg* \| *imm*                  | --                      | ``%status.cmp`` (w)      | Set ``$status.cmp`` to 1 if the operands have identical bit patterns; 0 of not.      |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``cmpltu`` | *r-reg* \| *imm-u* | *r-reg* \| *imm-u*                | --                      | ``%status.cmp`` (w)      | Set ``$status.cmp`` to 1 if Op1 < Op2 (assuming *unsigned int* encoding); 0 if not.  |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``cmplts`` | *r-reg* \| *imm-s* | *r-reg* \| *imm-s*                | --                      | ``%status.cmp`` (w)      | Set ``$status.cmp`` to 1 if Op1 < Op2 (assuming *two-comp* encoding); 0 if not.      |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``push``   | *r-reg* \| *imm*   | --                                | --                      | ``%sp`` (rw)             | Decrement ``%sp`` by *word-size*, and then copy the value of Op1 to                  |
|            |                    |                                   |                         |                          | mem[0:( *word-size* - 1)]                                                            |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | System behavior is undefined if this causes ``%sp`` to underflow.                    |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``pop``    | *w-reg*            | --                                | --                      | ``%sp`` (rw)             | Copy mem[0:( *word-size* - 1)] into register Op1, and then increment ``%sp`` by      |
|            |                    |                                   |                         |                          | *word-size*.                                                                         |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | System behavior is undefined if this causes ``%sp`` to overflow.                     |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``jmp``    | *r-reg* \| *imm*   | --                                | --                      | ``%pc`` (w)              | Set ``%pc`` to the specified value.                                                  |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``br``     | *r-reg* \| *imm*   | --                                | --                      | ``%pc`` (rw)             | Add the value of Op1 to ``%pc``.                                                     |
|            |                    |                                   |                         |                          |                                                                                      |
|            |                    |                                   |                         |                          | System behavior is undefined if this causes ``%pc`` to overflow.                     |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``brcond`` | *r-reg* \| *imm*   | --                                | --                      | ``%status.cmp`` (r)      | Like the ``br`` instruction of ``%status.cmp`` is set; otherwise do nothing.         |
|            |                    |                                   |                         | ``%pc`` (rw)             |                                                                                      |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``halt``   | --                 | --                                | --                      | ``%status.halt`` (w)     | Stop system execution.  The exact behavior is system-defined.                        |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+

Initial State
-------------
When a simluted CPU with this ISA starts up, the initial state is as
follows:

* register values: as specified in the table above.
* memory:  Simulator-defined.

  For early work, I suggest:

    * The memory starting at address 0 contains the program to execute.
    * The program contains hard-coded memory addresses of the memory region(s)
      to be used for program input / output values.

Assembly to Machine Instruction Encoding
----------------------------------------
TODO

Design Rationalle for ISA Details
---------------------------------
TODO:

* Why no zero register

* Why no interrupts

* Memory system

    * Why no virtual memory

    * Why no page table w/protections

* Multiplication: inspired by RISC-V "M"

    * Why RISC-V M's approach vs. alternatives

    * Explain why the signed/unsigned distinction matters for the high-significance half of the
      multiplication result, but not for the low-significance half.

      See:
        * https://tomverbeure.github.io/rtl/2018/08/12/Multipliers.html#cpu-multipliers

        * My less elegant exploration: https://github.com/cconvey/misc/blob/main/demo-that-lower-half-of-mult-results-are-signedness-indepentent.cpp


* Multiplication and div/rem: Why require two instructions?

    * RISC-V M does it, so apparently it's not that big of a limitation.

    * Pedagogy if/when we support performance optimization:

        * It gives an opportunity to motivate scheduling optimization
          in the LLVM backend: give a performance bonus for e.g. performing
          `MULSS` immediately followed by `MULHSS`.

        * Counterpoint: having these ops populate *two* result registers could
          be useful for teaching about the downsides of excessive conflicts
          during register allocation.

* Why no floating-point support

    * Pedagogy: keep things simple.

    * Pedagofy: motivate the use of LLVM's libcompiler-rt for certain lowerings.

* Why no SIMD

* Why no explicit support for concurrency

* Why only one size of value (*word-size*)

Acknowledgements
----------------

* https://mark.theis.site/riscv/

  Excellent approach and format for documenting assembly instructions.
  This document strives to copy that format, as the sincerest form of flattery.

* https://tomverbeure.github.io/rtl/2018/08/12/Multipliers.html#cpu-multipliers

  A good explanation on why the ``mul`` instruction doesn't have signed/unsigned
  variants.


