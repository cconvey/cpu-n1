CPU-N1 ISA
==========

ISA Parameters
--------------

The specific ISA is a function of these parameters:

+--------------------+------------------------------------------------------+
| Parameter          | Description                                          |
+====================+======================================================+
|``word_size_bytes`` |  The number of bits constituting a machine word.     |
|                    |  Note: This must be large enough to store all of     |
|                    |  the flags defined for the ``%status`` register.     |
+--------------------+------------------------------------------------------+
|``instr_size_bytes``|  The number of bytes used to encode a single         |
|                    |  program instruction. *All* machine instructions     |
|                    |  (including operands) have this size.                |
|                    |                                                      |
|                    |  The encoding scheme for each assembly instruction   |
|                    |  is described below. Note that instructions that can |
|                    |  be encoded with fewer bytes than this parameter     |
|                    |  indicates might use padding to reach this size.     |
+--------------------+------------------------------------------------------+
|``num_gp_regs``     |  The number of general-purpose registers.            |
+--------------------+------------------------------------------------------+

Memory
------

-  The ISA uses a flat, non-virtual memory model.
-  Memory addresses are ``$word_size_bytes`` in size.
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
-  All integer values are ``$word_size_bytes`` long.

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

+------------------+----------------------+-------------------------+----------------------------------------------------------+
| Mnemonic         | Size                 | Initial value           | Description                                              |
+==================+======================+=========================+==========================================================+
| ``%pc``          | ``$word_size_bytes`` | 0                       | Program counter. Always holds the memory                 |
|                  |                      |                         | address  of the instruction that will execute            |
|                  |                      |                         | *after* the current one.                                 |
|                  |                      |                         |                                                          |
|                  |                      |                         | Unless stated otherwise, this is                         |
|                  |                      |                         | incremented by ``$instr_size_bytes``                     |
|                  |                      |                         | immediately before the currently loaded                  |
|                  |                      |                         | instruction begins                                       |
|                  |                      |                         | execution.                                               |
|                  |                      |                         |                                                          |
|                  |                      |                         | This may be a read-only register operand                 |
|                  |                      |                         | for most/all instructions,                               |
|                  |                      |                         | but can only be modified by certain                      |
|                  |                      |                         | control-flow instructions.                               |
+------------------+----------------------+-------------------------+----------------------------------------------------------+
| ``%sp``          | ``$word_size_bytes`` | 0                       | Stack pointer. Although its interpretation up to the     |
|                  |                      |                         | user, it’s intended to support is function calls as      |
|                  |                      |                         | defined by the system ABI.                               |
|                  |                      |                         |                                                          |
|                  |                      |                         | The ISA is designed to support a stack that grows        |
|                  |                      |                         | toward lower addresses.                                  |
|                  |                      |                         | addresses.                                               |
|                  |                      |                         |                                                          |
|                  |                      |                         | For conceptual simplicity, the ISA requires that this    |
|                  |                      |                         | value has ``$word_size_bytes`` alignment.                |
+------------------+----------------------+-------------------------+----------------------------------------------------------+
| ``%gp0``,        | ``$word_size_bytes`` | undefined               | General-purpose registers. How many is                   |
| ``%gp1``,        |                      |                         | specified by ``$num_gp_regs``.                           |
| …                |                      |                         |                                                          |
+------------------+----------------------+-------------------------+----------------------------------------------------------+
| ``%status``      | ``$word_size_bytes`` | see below               | The status word register. The following are defined.     |
|                  |                      |                         | All other bits are considered reserved and have no       |
|                  |                      |                         | defined behavior.                                        |
+------------------+----------------------+-------------------------+----------------------------------------------------------+
| ``%status.cmp``  | 1 bit                | undefined               | The outcome of the most recent comparison instruction.   |
+------------------+----------------------+-------------------------+----------------------------------------------------------+
| ``%status.halt`` | 1 bit                | 1                       | ``0`` when the machine is running, ``1`` when it’s       |
|                  |                      |                         | halted.                                                  |
|                  |                      |                         |                                                          |
|                  |                      |                         | A program may set this to ``1`` to indicate that is has  |
|                  |                      |                         | run to completion.                                       |
+------------------+----------------------+-------------------------+----------------------------------------------------------+

Assembly Instructions
---------------------

Operands
~~~~~~~~

Instruction operands fall into these general categories: - Immediate
value. Used either for math, or to name a memory address. - Register
name. Indicates either: - the register whose initial value shall be
used, or - the register into which a new value shall be store.

Assembly Instruction Table
~~~~~~~~~~~~~~~~~~~~~~~~~~

Suggested assembly instructions, along with their semantics.

We use the following conventions in this table:
 * ``s1`` means source operand #1
 * ``s2`` means source operand #2
 * ``d`` means destination operand.
 * ``(...)`` indicates what kind(s) of operand is/are valid here:

   * ``(gp-reg)`` the name of any general-purpose register, such as ``%r0`` or ``%r13``
   * ``(reg)`` the name of any register, including ``%sp``, ``%pc``, and ``%status``
   * ``(imm)`` an immediate numerical value
   * ``(...|...)`` any one of the list options, e.g. ``(gp-reg|imm)``.

+------------+-------------+-------------+-------------+------------------------------------------------------------------------+
| Mnemonic   | Operand1    | Operand2    | Operand3    | Description                                                            |
+============+=============+=============+=============+========================================================================+
| ``add``    | (gp-reg)    | (reg|imm)   | (reg|imm2)  | TODO                                                                   |
+------------+-------------+-------------+-------------+------------------------------------------------------------------------+
| ``mult``   |             |             |             | TODO                                                                   |
+------------+-------------+-------------+-------------+------------------------------------------------------------------------+
| ``load``   |             |             |             | TODO                                                                   |
+------------+-------------+-------------+-------------+------------------------------------------------------------------------+
| ``store``  |             |             |             | TODO                                                                   |
+------------+-------------+-------------+-------------+------------------------------------------------------------------------+
| ``cmpeq``  |             |             |             | TODO                                                                   |
+------------+-------------+-------------+-------------+------------------------------------------------------------------------+
| ``cmplt``  |             |             |             | TODO                                                                   |
+------------+-------------+-------------+-------------+------------------------------------------------------------------------+
| ``push``   |             |             |             | TODO                                                                   |
+------------+-------------+-------------+-------------+------------------------------------------------------------------------+
| ``pop``    |             |             |             | TODO                                                                   |
+------------+-------------+-------------+-------------+------------------------------------------------------------------------+
| ``jmp``    |             |             |             | TODO                                                                   |
+------------+-------------+-------------+-------------+------------------------------------------------------------------------+
| ``br``     |             |             |             | TODO                                                                   |
+------------+-------------+-------------+-------------+------------------------------------------------------------------------+
| ``brcond`` |             |             |             | TODO                                                                   |
+------------+-------------+-------------+-------------+------------------------------------------------------------------------+
| ``halt``   |             |             |             | TODO                                                                   |
+------------+-------------+-------------+-------------+------------------------------------------------------------------------+

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
