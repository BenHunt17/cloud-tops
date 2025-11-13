### Development notes

### Cpu

## States

- ARM state: uses 32 bit instruction set
- THUMB state: uses 32 bit instruction set

## Pipelining

For faster execution, the processor can execute instruction while the next instructions are fetched and decoded simultaneously
I shouldn't need to emulate this directly but I do need to account for it:

- When reading the PC during the execution of an instruction, I need to make sure it is the intruction address + 8 bytes as in the real hardware, the decode and fetch would've occured meaning the PC would be 2 instructions onwards. (+ 4 bytes for THUMB mode)

- When a branch happens, the pipeline is "flushed" (emptied) as obviously the values at each stage are outdated. Therefore the pipeline has to fill again and 3 cycles are lost.

## Flags

From what I can understand, the sticky overflow, Jazelle mode, endian and abort flags don't apply to the ARM7TDMI
