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

Registers
---------

All registers are distinct. The ISA has no concept of addressible
sub-registers. However, some mnemonics for sub-registers are provided
for human-friendly documentation (and perhaps human-friendly assembly
support).

+----------------------------+----------------------+-------------------------+----------------------------------------------------------+
| Mnemonic                   | Size                 | Initial value           | Description                                              |
+============================+======================+=========================+==========================================================+
| ``%pc``                    | *word-size*          | 0                       | Program counter. Always holds the memory                 |
|                            |                      |                         | address  of the instruction that will execute            |
|                            |                      |                         | *after* the current one.                                 |
|                            |                      |                         |                                                          |
|                            |                      |                         | Unless stated otherwise, this is                         |
|                            |                      |                         | incremented by *instr-size*                              |
|                            |                      |                         | immediately before the currently loaded                  |
|                            |                      |                         | instruction begins                                       |
|                            |                      |                         | execution.                                               |
|                            |                      |                         |                                                          |
|                            |                      |                         | This may be a read-only register operand                 |
|                            |                      |                         | for most/all instructions,                               |
|                            |                      |                         | but can only be modified by certain                      |
|                            |                      |                         | control-flow instructions.                               |
+----------------------------+----------------------+-------------------------+----------------------------------------------------------+
| ``%sp``                    | *word-size*          | 0                       | Stack pointer. Although its interpretation up to the     |
|                            |                      |                         | user, it’s intended to support is function calls as      |
|                            |                      |                         | defined by the system ABI.                               |
|                            |                      |                         |                                                          |
|                            |                      |                         | The ISA is designed to support a stack that grows        |
|                            |                      |                         | toward lower addresses.                                  |
|                            |                      |                         | addresses.                                               |
|                            |                      |                         |                                                          |
|                            |                      |                         | For conceptual simplicity, the ISA requires that this    |
|                            |                      |                         | value has *word-size* alignment.                         |
+----------------------------+----------------------+-------------------------+----------------------------------------------------------+
| ``%gp0``,                  | *word-size*          | undefined               | General-purpose registers. How many is                   |
| ``%gp1``,                  |                      |                         | specified by *num-gp-regs*.                              |
| …                          |                      |                         |                                                          |
+----------------------------+----------------------+-------------------------+----------------------------------------------------------+
| ``%status``                | *word-size*          | see below               | The status word register. The following are defined.     |
|                            |                      |                         | All other bits are considered reserved and have no       |
|                            |                      |                         | defined behavior.                                        |
+----------------------------+----------------------+-------------------------+----------------------------------------------------------+
| ``%status.cmp``            | 1 bit                | undefined               | The outcome of the most recent comparison instruction.   |
+----------------------------+----------------------+-------------------------+----------------------------------------------------------+
| ``%status.overflow``       | 1 bit                | undefined               | Set/cleared by some ops involving math.                  |
+----------------------------+----------------------+-------------------------+----------------------------------------------------------+
| ``%status.halt``           | 1 bit                | 1                       | ``0`` when the machine is running, ``1`` when it’s       |
|                            |                      |                         | halted.                                                  |
|                            |                      |                         |                                                          |
|                            |                      |                         | A program may set this to ``1`` to indicate that is has  |
|                            |                      |                         | run to completion.                                       |
+----------------------------+----------------------+-------------------------+----------------------------------------------------------+

Suggested assembly language
---------------------------

Assembly grammar (partial)
~~~~~~~~~~~~~~~~~~~~~~~~~~

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

w-reg : ( ``%sp`` | *gp-reg* )

    A register into which most instructions can freely write.

*r-reg* : ( *gp-reg* | ``%sp`` | ``%pc`` )

    A register from which most instructions can freely read.

Assembly instructions
~~~~~~~~~~~~~~~~~~~~~

+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| Mnemonic   | Operand1           | Operand2                          | Operand3                | Implicit reg. access     | Description                                                                          |
+============+====================+===================================+=========================+==========================+======================================================================================+
| ``add``    | *w-reg*            | *r-reg* \| *imm*                  | *r-reg* \| *imm*        | ``%status.overflow`` (w) | Numerically add the two values.  Signed vs. unsigned semantics are user-defined.     |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``mult``   | TODO               | TODO                              | TODO                    | TODO                     | TODO: This needs more thought w.r.t. signed/unsigned and overflow support.           |
+------------+--------------------+-----------------------------------+-------------------------+--------------------------+--------------------------------------------------------------------------------------+
| ``divmod`` | TODO               | TODO                              | TODO                    | TODO                     | TODO: This needs more thought w.r.t. signed/unsigned and overflow support.           |
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
