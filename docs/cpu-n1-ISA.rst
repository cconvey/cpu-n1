CPU-N1 ISA
==========

ISA Parameters
--------------

The specific ISA is a function of these parameters:

+--------------------+------------------------------------------------------------------------------------+
| Parameter          | Description                                                                        |
+====================+====================================================================================+
| *word-size*        | The number of bits constituting a machine word.                                    |
|                    |                                                                                    |
|                    | This value is *not* required to be a power of 2.                                   |
|                    | (Although that may change if it turns out to cause too many problems.)             |
|                    |                                                                                    |
|                    | Note: This must be large enough to store all of                                    |
|                    | the flags defined for the ``%status`` register.                                    |
+--------------------+------------------------------------------------------------------------------------+
| *instr-size*       | The number of bytes used to encode a single                                        |
|                    | program instruction. *All* machine instructions                                    |
|                    | (including operands) have this size.                                               |
|                    |                                                                                    |
|                    | For simplicity of the ISA, *instr-size* is required to be an integer multiple of   |
|                    | *word-size*.                                                                       |
|                    |                                                                                    |
|                    | The encoding scheme (descibed below) may impose additional limitations on this     |
|                    | value.                                                                             |
+--------------------+------------------------------------------------------------------------------------+
| *num-gp-regs*      | The number of general-purpose registers.                                           |
+--------------------+------------------------------------------------------------------------------------+

Memory
------

-  The ISA uses a flat, non-virtual memory model.

-  Each memory address names a single byte. (Instructions that operate
   on multi-byte regions of memory will generally indicate that range by
   the lowest-address byte in the range.)

-  Memory addresses are *word-size* bytes in size.

   This implies that the maximum amount of addressable memory in this ISA is
   2^(*word-size*) bytes.

   Implementations are not required to treat all addresses equally.
   For example, some implementations may treat certain address ranges as
   used for MMIO, ROM, etc.  Or, as in some(?) x86-64 procesors, treat
   the highest-order bits as irrelevant to addressing, which allows
   some programs to repurpose those bits.  However, the ISA currently
   has no specific support for making these details discoverable or
   enforceable; that remains implementation-defined.

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

The "Operand-code" column isn't related to assembly programming.
It indicates the numeric value used to identify that particular register
in the ISA's instruction encoding.

.. TODO:
   - Move the docs for the '...u' convention above this table, because the table uses them.

+----------------------------+----------------------+---------------+--------------+----------------------------------------------------------------------------------------+
| Mnemonic                   | Size                 | Initial value | Operand-code | Description                                                                            |
+============================+======================+===============+==============+========================================================================================+
| ``%pc``                    | *word-size*          | 0             | 1u           | Program counter. Always holds the address of the instruction to be executed.           |
|                            |                      |               |              | *after* the current one.                                                               |
+----------------------------+----------------------+---------------+--------------+----------------------------------------------------------------------------------------+
| ``%sp``                    | *word-size*          | 0             | 2u           | Stack pointer. Although its interpretation up to the                                   |
|                            |                      |               |              | user, it’s intended to support is function calls as                                    |
|                            |                      |               |              | defined by the system ABI.                                                             |
|                            |                      |               |              |                                                                                        |
|                            |                      |               |              | The ISA is designed to support a stack that grows                                      |
|                            |                      |               |              | toward lower addresses.                                                                |
|                            |                      |               |              | addresses.                                                                             |
|                            |                      |               |              |                                                                                        |
|                            |                      |               |              | For conceptual simplicity, the ISA requires that this                                  |
|                            |                      |               |              | value has *word-size* alignment.                                                       |
+----------------------------+----------------------+---------------+--------------+----------------------------------------------------------------------------------------+
| ``%status``                | *word-size*          | see below     | 3u           | The status word register. The following are defined.                                   |
|                            |                      |               |              | All other bits are considered reserved and have no                                     |
|                            |                      |               |              | defined meaning.                                                                       |
+----------------------------+----------------------+---------------+--------------+----------------------------------------------------------------------------------------+
| ``%status.cmp``            | 1 bit                | undefined     | N/A          | The outcome of the most recent comparison instruction.                                 |
+----------------------------+----------------------+---------------+--------------+----------------------------------------------------------------------------------------+
| ``%status.overflow``       | 1 bit                | undefined     | N/A          | Set/cleared by some ops involving math.                                                |
|                            |                      |               |              |                                                                                        |
|                            |                      |               |              | "overflow" might be a poor word for some uses, so this part of the ISA may be changed, |
|                            |                      |               |              | and/or this register might get renamed to something more appropriate.                  |
+----------------------------+----------------------+---------------+--------------+----------------------------------------------------------------------------------------+
| ``%status.halt``           | 1 bit                | 1             | N/A          | ``0`` when the machine is running, ``1`` when it’s                                     |
|                            |                      |               |              | halted.                                                                                |
|                            |                      |               |              |                                                                                        |
|                            |                      |               |              | A program may set this to ``1`` to indicate that is has                                |
|                            |                      |               |              | run to completion.                                                                     |
+----------------------------+----------------------+---------------+--------------+----------------------------------------------------------------------------------------+
| ``%gp0``,                  | *word-size*          | undefined     | 256u,        | General-purpose registers. How many is                                                 |
| ``%gp1``,                  |                      |               | 257u,        | specified by *num-gp-regs*.                                                            |
| …                          |                      |               | ...          |                                                                                        |
+----------------------------+----------------------+---------------+--------------+----------------------------------------------------------------------------------------+

Suggested assembly language
---------------------------

Operand grammar (partial)
~~~~~~~~~~~~~~~~~~~~~~~~~

Notes:

    * The grammars below for numeric literals allow multiple representations of
      the same number.  E.g., ``0042``, ``042``, and ``42``.

      For each of these grammars, we also define a *normal form*, which gives
      a predictable upper-bound on the string lengths.

      We do this to allow our machine-code encoding, which uses human-friendly
      string renditions of these numbers, to have a predictable instruction
      length.

*hex-word* : Regex ``0x[0-9a-fA-F]+``

    Hexadecimal representation of a number in the range 0 ... 2 ::sup::`wordsize`.

    Leading zeros are permitted, but the overall string-length may be limited by
    assemblers, etc.

    Normal form:

        * The total number of hexadecimal digits is precisely ``ceil( log_16(wordsize * 8) )``.

          This may require adding or removing leading zeroes from the original form.

    Exact  normal-form string length: ::

        len("0x") + ceil( log_16(wordsize * 8) )

*signed-dec-word* : Regex ``[+-]?[0-9]+s``

    The twos-complement bit pattern (*$word-size* in length)
    of the specified number.
    The number must lie within the valid range.

    Leading zeros are permitted, but the overall string-length may be limited by
    assemblers, etc.

    E.g.: ``+42s``, ``42s``, ``-42s``, ``-00042s``, ``0s``, ``-0s``, ``+0s``.

    Normal form:

        * Any string equivalent to numeric zero becomes ``+0s``.

        * If the string doesn't start with ``-``, then it starts with ``+``.

        * All leading zeroes are removed (when the string isn't equivalent to
          numeric zero).

    Maximum normal-form string length:

        .. code-block::

            max(len("+"), len("-")) + ceil( (wordsize*8 - 1) * log_10(2) ) + len("s")

*unsigned-dec-word* : Regex ``[0-9]+u``

    The unsigned-integer bit pattern (*$word-dize-bytes* in length)
    of the specified number.
    The number must lie within the valid range.

    Leading zeros are permitted, but the overall string-length may be limited by
    assemblers, etc.

    E.g.: ``42u``, ``0u``, ``00042u``.

    Normal form:

        * Any string equivalent to numeric zero becomes ``0u``.

        * For all other numbers, all leading zeroes are removed.

    Maximum normal-form string length:

        .. code-block::

            ceil( (wordsize*8  * log_10(2) ) + len("u")

*imm-u* : *unsigned-dec-word*

*imm-s* : *signed-dec-word*

*imm* : ( *signed-dec-word* | *unsigned-dec-word* )

*gp-reg* :  any valid gp register, e.g. ``%gp3``

    Note: The register number may not contain leading zeroes.  E.g., ``%gp03`` is *not* legal.

*reg* : ( ``%pc`` | ``%sp`` | ``%status`` | *gp-reg* )

*w-reg* : ( ``%sp`` | *gp-reg* )

    A register into which most instructions can freely write.

*r-reg* : ( *gp-reg* | ``%sp`` | ``%pc`` )

    A register from which most instructions can freely read.

Assembly instructions
~~~~~~~~~~~~~~~~~~~~~

Conventions for pseudocode used in assembly descriptions and instruction encoding:

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

* The "Opcode" column indicates the opcode used for the corresponding machine instruction.
  Although not technically part of the assembly code, we provide it here to avoid another
  table with one row per instruction.

.. TODO:
..
.. * Better pseudo-code
..
..   * Provide it for all suitable instructions.
..   * Format it it well, e.g. with ``.. code-block::``.

.. |            |                    |                                   |                         |                          |                                                                                      |

+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| Mnemonic   | Opcode   | Operand1           | Operand2                          | Operand3                | Implicit reg. access     | Description                                                                          |
+============+==========+====================+===================================+=========================+==========================+======================================================================================+
| ``add``    | 1u       | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm*        | ``%status.overflow`` (w) | Op1 ← Op2 + Op3                                                                      |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | %status.overflow ← (the result wrapped due to overflow)                              |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``sub``    | 2u       | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm*        | ``%status.overflow`` (w) | Op1 ← Op2 - Op3                                                                      |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | %status.overflow ← (the result wrapped due to underflow)                             |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``mul``    | 3u       | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm*        | --                       | .. code-block::                                                                      |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          |    Op1 ← (zxdw(Op2) * zxdw(Op3))[ word-size ... 0 ]                                  |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | Compute the lower-half result of Op2 * Op3.                                          |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | Note that signed/unsigned distinction isn't needed for this lower-half.              |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``mulhss`` | 4u       | *w-reg*            | *r-reg* \| *imm-s*                | *r-reg* \| *imm-s*      | --                       | Op1 ← (sxdw(Op2) * sxdw(Op3)[ ((2 *  *word-size*) - 1) ... *word-size* ]             |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | Compute the upper-half result of Op2 * Op3.  Assume Op2 and Op3 are signed.          |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``mulhsu`` | 5u       | *w-reg*            | *r-reg* \| *imm-s*                | *r-reg* \| *imm-s*      | --                       | Op1 ← (sxdw(Op2) * zxdw(Op3)[ ((2 *  *word-size*) - 1) ... *word-size* ]             |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | Compute the upper-half result of Op2 * Op3.  Assume Op2 is signed, Op3 is unsigned.  |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``mulhuu`` | 6u       | *w-reg*            | *r-reg* \| *imm-s*                | *r-reg* \| *imm-s*      | --                       | Op1 ← (zxdw(Op2) * zxdw(Op3)[ ((2 *  *word-size*) - 1) ... *word-size* ]             |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | Compute the upper-half result of Op2 * Op3.  Assume Op2 and Op3 are unsigned.        |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``dipss``  | 7u       | *w-reg*            | *r-reg* \| *imm-s*                | *r-reg* \| *imm-s*      | ``%status.overflow`` (w) | Op1 ← (Op2 / Op3) rounded towards zero.                                              |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | If Op3 == 0, Op1's value is undefined.                                               |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | ``%status.overflow`` ← (Op3 != 0)                                                    |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | Integer division.  Assume Op2 and Op3 are signed.                                    |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``divuu``  | 8u       | *w-reg*            | *r-reg* \| *imm-u*                | *r-reg* \| *imm-u*      | ``%status.overflow`` (w) | Op1 ← (Op2 / Op3) rounded towards zero.                                              |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | If Op3 == 0, Op1's value is undefined.                                               |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | ``%status.overflow`` ← (Op3 != 0)                                                    |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | Integer division.  Assume Op2 and Op3 are unsigned.                                  |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``remss``  | 9u       | *w-reg*            | *r-reg* \| *imm-s*                | *r-reg* \| *imm-s*      | ``%status.overflow`` (w) | Op1 ← (Op2 % Op3)                                                                    |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | If Op3 == 0, Op1's value is undefined.                                               |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | ``%status.overflow`` ← (Op3 != 0)                                                    |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | Integer division.  Assume Op2 and Op3 are signed.                                    |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``remuu``  | 10u      | *w-reg*            | *r-reg* \| *imm-u*                | *r-reg* \| *imm-u*      | ``%status.overflow`` (w) | Op1 ← (Op2 % Op3)                                                                    |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | If Op3 == 0, Op1's value is undefined.                                               |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | ``%status.overflow`` ← (Op3 != 0)                                                    |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | Integer division.  Assume Op2 and Op3 are unsigned.                                  |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``and``    | 11u      | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm*        | --                       | Op1 ← Op2 & Op3                                                                      |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``or``     | 12u      | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm*        | --                       | Op1 ← Op2 | Op3                                                                      |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``xor``    | 13u      | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm*        | --                       | Op1 ← Op2 ^ Op3                                                                      |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``not``    | 14u      | *w-reg*            | *r-reg* \| *imm*                  | --                      | --                       | Op1 ← ~Op2                                                                           |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``lsl``    | 15u      | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm-u*      |                          | Op1 ← Op2 << Op3                                                                     |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | Compute the logical-left-shift of Op2, shifted by the number of bits indicated by    |
|            |          |                    |                                   |                         |                          | Op3.                                                                                 |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | If Op3 > *word-size*, the instruction is illegal.                                    |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``rsl``    | 16u      | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm-u*      |                          | Op1 ← Op2 >> Op3                                                                     |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | Compute the logical-right-shift of Op2, shifted by the number of bits indicated by   |
|            |          |                    |                                   |                         |                          | Op3.                                                                                 |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | If Op3 > *word-size*, the instruction is illegal.                                    |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``mov``    | 17u      | *w-reg*            | *r-reg* \| *imm*                  | --                      | --                       | Copy the value Op2 into register Op1.                                                |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``load``   | 18u      | *w-reg*            | *r-reg* \| *imm*                  | --                      | --                       | Copy the memory value *pointed to by* Op2 into register Op1.                         |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``store``  | 19u      | *r-reg* \| *imm*   | *r-reg* \| *imm*                  | --                      | --                       | Copy the value Op2 to the memory location pointed to by Op1                          |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``cmpeq``  | 20u      | *r-reg* \| *imm*   | *r-reg* \| *imm*                  | --                      | ``%status.cmp`` (w)      | Set ``$status.cmp`` to 1 if the operands have identical bit patterns; 0 of not.      |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``cmpltu`` | 21u      | *r-reg* \| *imm-u* | *r-reg* \| *imm-u*                | --                      | ``%status.cmp`` (w)      | Set ``$status.cmp`` to 1 if Op1 < Op2 (assuming *unsigned int* encoding); 0 if not.  |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``cmplts`` | 22u      | *r-reg* \| *imm-s* | *r-reg* \| *imm-s*                | --                      | ``%status.cmp`` (w)      | Set ``$status.cmp`` to 1 if Op1 < Op2 (assuming *two-comp* encoding); 0 if not.      |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``push``   | 23u      | *r-reg* \| *imm*   | --                                | --                      | ``%sp`` (rw)             | Decrement ``%sp`` by *word-size*, and then copy the value of Op1 to                  |
|            |          |                    |                                   |                         |                          | mem[0:( *word-size* - 1)]                                                            |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | System behavior is undefined if this causes ``%sp`` to underflow.                    |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``pop``    | 24u      | *w-reg*            | --                                | --                      | ``%sp`` (rw)             | Copy mem[0:( *word-size* - 1)] into register Op1, and then increment ``%sp`` by      |
|            |          |                    |                                   |                         |                          | *word-size*.                                                                         |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | System behavior is undefined if this causes ``%sp`` to overflow.                     |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``jmp``    | 25u      | *r-reg* \| *imm*   | --                                | --                      | ``%pc`` (w)              | Set ``%pc`` to the specified value.                                                  |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``br``     | 26u      | *r-reg* \| *imm*   | --                                | --                      | ``%pc`` (rw)             | Add the value of Op1 to ``%pc``.                                                     |
|            |          |                    |                                   |                         |                          |                                                                                      |
|            |          |                    |                                   |                         |                          | System behavior is undefined if this causes ``%pc`` to overflow.                     |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``brcond`` | 27u      | *r-reg* \| *imm*   | --                                | --                      | ``%status.cmp`` (r)      | Like the ``br`` instruction of ``%status.cmp`` is set; otherwise do nothing.         |
|            |          |                    |                                   |                         | ``%pc`` (rw)             |                                                                                      |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``halt``   | 28u      | --                 | --                                | --                      | ``%status.halt`` (w)     | Stop system execution.  The exact behavior is system-defined.                        |
+------------+----------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+

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

Every machine instruction is encoded as a sequence of 4 words, with word 0 as the lowest-address word.

    * word 0: operation
    * word 1: operand #1
    * word 2: operand #2, or all zeros if not applicable.
    * word 3: operand #3, or all zeros if not applicable.

Operation word (word #0)
~~~~~~~~~~~~~~~~~~~~~~~~

+--------------------------------+------------------------------------------------------------------------------------------------------+
| bit range                      | interpretation                                                                                       |
+================================+======================================================================================================+
| [ (*word-size*-1) ... 8 ]      | An unsigned integer indicating an instruction from the assembly language table, above.               |
|                                |                                                                                                      |
|                                | See the "Opcode" column in the instruction table below for the mapping.                              |
+--------------------------------+------------------------------------------------------------------------------------------------------+
| [ 7 ... 6 ]                    | Reserved for future use.  Must be 00b.                                                               |
+--------------------------------+------------------------------------------------------------------------------------------------------+
| [ 5 ... 4 ] (for operand #1)   | For each of the 3 operand slots, this indicates the operand kind:                                    |
|                                |                                                                                                      |
|                                |    00b : No operand provided; corresponding operand slot should contain all zeros.                   |
|                                |                                                                                                      |
| [ 3 ... 2 ] (for operand #2)   |    01b : The corresponding operand slot holds an immediate value                                     |
|                                |                                                                                                      |
|                                |    10b : The corresponding operand slot holds a register name; see below for register-name encoding. |
| [ 1 ... 0 ] (for operand #3)   |                                                                                                      |
|                                |    11b : Reserved for future use.                                                                    |
+--------------------------------+------------------------------------------------------------------------------------------------------+

Operand words (words #1 ... #3)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Each operand word is encoded as follows, depending on the operand kind:

+--------------------------------+------------------------------------------------------------------------------------------------------+
| operand kind                   | interpretation                                                                                       |
+================================+======================================================================================================+
| no operand (00b)               | The instruction doesn't use this operand.  Must be all zeros.                                        |
+--------------------------------+------------------------------------------------------------------------------------------------------+
| immediate value (01b)          | The slot is populated with the immediate value's bit pattern.                                        |
+--------------------------------+------------------------------------------------------------------------------------------------------+
| register name (10b)            | The slot contains the name of some register.  See the "Operand code" column in the registers table.  |
+--------------------------------+------------------------------------------------------------------------------------------------------+

Design Rationalle for ISA Details
---------------------------------
TODO:

* Why unused operand slots must contain all zeros.

* Why ISA reserves bits  [7..6] in the opcode word.

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


