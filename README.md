# vVM

vcokltfre's Virtual Machine (vVM) is a bytecode VM for me to write interesting micro-languages atop.

## License

vVM is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Instructions

### `EXIT` - `0x00`

Exit the program.

Mnemonic: `EXIT`

Stack usage:

| Position | Usage     |
| -------- | --------- |
| top      | exit code |

### `EXIT_IMM` - `0x01`

Exit the program with an immediate exit code.

Mnemonic: `EXIT <code>`

### `PUSHI` - `0x10`

Push a signed integer onto the stack.

Mnemonic: `PUSHI <int>`

### `PUSHU` - `0x11`

Push an unsigned integer onto the stack.

Mnemonic: `PUSHU <uint>`

### `PUSHF` - `0x12`

Push a floating-point number onto the stack.

Mnemonic: `PUSHF <float>`

### `PUSHB` - `0x13`

Push a boolean onto the stack.

Mnemonic: `PUSHB <bool>` (where `<bool>` is `true` or `false`)

### `PUSHS` - `0x14`

Push a string onto the stack.

Mnemonic: `PUSHS <string>`

### `POP` - `0x15`

Pop the top value off the stack.

Mnemonic: `POP`

Stack usage:

| Position | Usage        |
| -------- | ------------ |
| top      | value to pop |

### `DUP` - `0x16`

Duplicate the top value on the stack.

Mnemonic: `DUP`

Stack usage:

| Position | Usage        |
| -------- | ------------ |
| top      | value to dup |

### `SWAP` - `0x17`

Swap the top two values on the stack.

Mnemonic: `SWAP`

Stack usage:

| Position | Usage   |
| -------- | ------- |
| top      | value 1 |
| next     | value 2 |

### `ADD` - `0x20`

Add the top two values on the stack.

Mnemonic: `ADD`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | rhs   |
| next     | lhs   |

Result: lhs + rhs

### `ADDI` - `0x21`

Add the immediate integer to the top value on the stack.

Mnemonic: `ADDI <int>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | lhs   |

Result: lhs + <int>

### `ADDU` - `0x22`

Add the immediate unsigned integer to the top value on the stack.

Mnemonic: `ADDU <uint>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | lhs   |

Result: lhs + <uint>

### `ADDF` - `0x23`

Add the immediate floating-point number to the top value on the stack.

Mnemonic: `ADDF <float>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | lhs   |

Result: lhs + <float>

### `SUB` - `0x24`

Subtract the top two values on the stack.

Mnemonic: `SUB`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | rhs   |
| next     | lhs   |

Result: lhs - rhs

### `SUBI` - `0x25`

Subtract the immediate integer from the top value on the stack.

Mnemonic: `SUBI <int>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | lhs   |

Result: lhs - <int>

### `SUBU` - `0x26`

Subtract the immediate unsigned integer from the top value on the stack.

Mnemonic: `SUBU <uint>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | lhs   |

Result: lhs - <uint>

### `SUBF` - `0x27`

Subtract the immediate floating-point number from the top value on the stack.

Mnemonic: `SUBF <float>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | lhs   |

Result: lhs - <float>

### `MUL` - `0x28`

Multiply the top two values on the stack.

Mnemonic: `MUL`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | rhs   |
| next     | lhs   |

Result: lhs \* rhs

### `MULI` - `0x29`

Multiply the immediate integer with the top value on the stack.

Mnemonic: `MULI <int>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | lhs   |

Result: lhs \* <int>

### `MULU` - `0x2A`

Multiply the immediate unsigned integer with the top value on the stack.

Mnemonic: `MULU <uint>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | lhs   |

Result: lhs \* <uint>

### `MULF` - `0x2B`

Multiply the immediate floating-point number with the top value on the stack.

Mnemonic: `MULF <float>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | lhs   |

Result: lhs \* <float>

### `DIV` - `0x2C`

Divide the top two values on the stack.

Mnemonic: `DIV`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | rhs   |
| next     | lhs   |

Result: lhs / rhs

### `DIVI` - `0x2D`

Divide the top value on the stack by the immediate integer.

Mnemonic: `DIVI <int>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | lhs   |

Result: lhs / <int>

### `DIVU` - `0x2E`

Divide the top value on the stack by the immediate unsigned integer.

Mnemonic: `DIVU <uint>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | lhs   |

Result: lhs / <uint>

### `DIVF` - `0x2F`

Divide the top value on the stack by the immediate floating-point number.

Mnemonic: `DIVF <float>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | lhs   |

Result: lhs / <float>

### `MOD` - `0x30`

Calculate the modulus of the top two values on the stack.

Mnemonic: `MOD`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | rhs   |
| next     | lhs   |

Result: lhs % rhs

### `MODI` - `0x31`

Calculate the modulus of the top value on the stack with the immediate integer.

Mnemonic: `MODI <int>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | lhs   |

Result: lhs % <int>

### `MODU` - `0x32`

Calculate the modulus of the top value on the stack with the immediate unsigned integer.

Mnemonic: `MODU <uint>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | lhs   |

Result: lhs % <uint>

### `EXP` - `0x33`

Raise the top but one value on the stack to the power of the top value.

Mnemonic: `EXP`

Stack usage:

| Position | Usage    |
| -------- | -------- |
| top      | exponent |
| next     | base     |

Result: base ** exponent

### `EXPI` - `0x34`

Raise the top value on the stack to the power of the immediate integer.

Mnemonic: `EXPI <int>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | base  |

Result: base ** <int>

### `EXPU` - `0x35`

Raise the top value on the stack to the power of the immediate unsigned integer.

Mnemonic: `EXPU <uint>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | base  |

Result: base ** <uint>

### `EXPF` - `0x36`

Raise the top value on the stack to the power of the immediate floating-point number.

Mnemonic: `EXPF <float>`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | base  |

Result: base ** <float>

### `LOAD` - `0x40`

Load a variable from memory with the name on the top of the stack.

Mnemonic: `LOAD`

Stack usage:

| Position | Usage          |
| -------- | -------------- |
| top      | variable name  |

### `LOAD_IMM` - `0x41`

Load a variable from memory with the immediate name.

Mnemonic: `LOAD_IMM <name>`

### `STORE` - `0x42`

Store a variable to memory with the name on the top of the stack.

Mnemonic: `STORE`

Stack usage:

| Position | Usage          |
| -------- | -------------- |
| top      | variable name  |
| next     | value to store |

### `STORE_IMM` - `0x43`

Store a variable to memory with the immediate name.

Mnemonic: `STORE_IMM <name>`

Stack usage:

| Position | Usage          |
| -------- | -------------- |
| top      | value to store |

### `FREE` - `0x44`

Free a variable from memory with the name on the top of the stack.

Mnemonic: `FREE`

Stack usage:

| Position | Usage         |
| -------- | ------------- |
| top      | variable name |

### `FREE_IMM` - `0x45`

Free a variable from memory with the immediate name.

Mnemonic: `FREE_IMM <name>`

### `CMPEQ` - `0x50`

Compare the top two values on the stack for equality.

Mnemonic: `CMPEQ`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | rhs   |
| next     | lhs   |

Result: lhs == rhs

### `CMPNE` - `0x51`

Compare the top two values on the stack for inequality.

Mnemonic: `CMPNE`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | rhs   |
| next     | lhs   |

Result: lhs != rhs

### `CMPGT` - `0x52`

Compare if the next value on the stack is greater than the top value.

Mnemonic: `CMPGT`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | rhs   |
| next     | lhs   |

Result: lhs > rhs

### `CMPLT` - `0x53`

Compare if the next value on the stack is less than the top value.

Mnemonic: `CMPLT`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | rhs   |
| next     | lhs   |

Result: lhs < rhs

### `CMPGE` - `0x54`

Compare if the next value on the stack is greater than or equal to the top value.

Mnemonic: `CMPGE`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | rhs   |
| next     | lhs   |

Result: lhs >= rhs

### `CMPLE` - `0x55`

Compare if the next value on the stack is less than or equal to the top value.

Mnemonic: `CMPLE`

Stack usage:

| Position | Usage |
| -------- | ----- |
| top      | rhs   |
| next     | lhs   |

Result: lhs <= rhs

### `JMP` - `0x60`

Jump to the immediate label.

Mnemonic: `JMP <label>`

### `JMPIF` - `0x61`

Jump to the immediate label if the top value on the stack is true.

Mnemonic: `JMPIF <label>`

Stack usage:

| Position | Usage        |
| -------- | ------------ |
| top      | condition    |

### `CALL` - `0x62`

Call the immediate function label.

Mnemonic: `CALL <label>`

### `CALLNATIVE` - `0x63`

Call the immediate native function.

Mnemonic: `CALLNATIVE <name>`

### `RET` - `0x64`

Return from the current function call.

Mnemonic: `RET`

### `LABEL` - `0x70`

Define a label at the current instruction pointer.

Mnemonic: `LABEL <label>`
