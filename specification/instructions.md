# Instructions
Here is a list of instructions:

| Instruction | Description  | Hex Code | Format |
|-------------|--------------|----------|--------|
| nop         | Does nothing | 0x0      | V      |
| ldr rn rm   | Loads one byte from the address in rm and stores the value in rn | 0x1 | R |
| str rn rm   | Stores the byte stored in rn to the address in rm | 0x2 | R |
| add rn rm   | Adds rn+rm, storing the result in r0 | 0x3 | R |
| add rn, #imm| Sums rn and the immediate value, storing the result in r0 | 0x4 | I |
| sub rn rm   | Substracts rn-rm, storing the result in r0 | 0x5 | R |
| sub rn, #imm| Subtracts rn and the immediate value, storing the result in r0 | 0x6 | I |
| lsl rn, #imm| Shifts the bits in rn left by the immediate value, storing the result in r0 | 0x7 | I |
| lsr rn, #imm| Shifts the bits in rn right by the immediate value, storing the result in r0 | 0x8 | I |
| and rn, rm  | Bitwise and of registers rn and rm, storing the result in r0 | 0x9 | R |
| or  rn, rm  | Bitwise or of registers rn and rm, storing the result in r1 | 0xA | R |
| xor rn, rm  | Bitwise xor of the registers rn and rm, storing the result in r0 | 0xB | R |
| \<unused\>  | | 0xC | |
| cmp rn, rm  | Compare registers rn and rm and set flags (z, n) | 0xD | R |
| j\<c\>rn    | Jump the number of bytes specified by the value in rn using the code c | 0xE | J |
| ret         | Jump to the address in r0 | 0xF | V |

## Instruction Formats

### V Format

An instruction that takes no arguments.

Format (asm):
```asm
<instruction>
```

Format (bin):
```
<8: instruction>000000000000
```

**instruction:** *(8 bits)* The instruction code

### R Format

An instruction that takes two registers as arguments

Format (asm):
```asm
<instruction> r1 r2
```

Format (bin):
```
<8: instruction><4: register><4: register>
```

**instruction:** *(8 bits)* The instruction code
**register:** *(4 bits)* The register number

### I Format

An instruction that takes a register and an immediate

Format (asm):
```asm
<instruction> r1 #imm
```

Format (bin):
```
<8: instruction><4: register><4: immediate>
```

**instruction:** *(8 bits)* The instruction code
**register:** *(4 bits)* The register number
**immediate:** *(4 bits)* The immediate number

### J Format

An instruction that takes an offset and a code. Can only be used for a jump instruction.

Format (asm):
```asm
<instruction><c> #imm
```

Format (bin):
```
<4: instruction><3: condition code><9: offset>
```

**instruction:** *(4 bits)* The instruction code
**condition code:** *(3 bits)* The condition code (see below)
**offset:** *(9 bits)* The offset to jump (in bits)

#### Condition Codes

| Code | Description |
|------|-------------|
| 000  | Uncoditional Branch |
| 001  | Less than |
| 010  | Greater than |
| 101  | Less than or equal to |
| 110  | Gretaer than or equal to |
| 100  | Equal to |
| 011  | Not equal to |
| 111  | Jump and set r0 to current address |
