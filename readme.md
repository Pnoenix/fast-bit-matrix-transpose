# Base idea
Transposes a 32x32 bit matrix *really* (Some might even say *blazingly*) fast. This is done through a modified divide and conquer approach. Refer to this image stolen from [here](https://github.com/dsnet/matrix-transpose?tab=readme-ov-file) which has been slightly modified, because it looked weird on github (Note that the image shows a transpose around the top-right to bottom-left diagonal. Implementation is a transpose around the top-left to bottom-right diagonal):
![Bit matrix tranpose example](matrix-transpose-method.png)

# Implementation details related to SIMD registers
Due to this bit matrix transpose using SIMD registers, the data is swizzled around so that it can all be operated upon at once. This means that for the first transpose (In the image), one would imagine two SIMD registers one for the top 8x4 of the bits, and one for the bottom 8x4 of the bits. Let's call them register 1 and 2. A mask, 0b11110000, is applied to register 1, selecting the left most bits. Then the same mask is applied to register 2. The result of applying this mask to register 2, is then shifted to the right by 4. This is or'ed together with the result of applying the mask to register 1. This transposes the the top row, of the bottom left 4x4 square, to the top row of the top right 4x4 square:

```
Initial state:
Register 1: 11010010
Register 2: 10010110

Masking:
Register 1: 11010010
       and: 11110000
    equals: 11010000

Register 2: 10010110
       and: 11110000
    equals: 10010000
 shifted 4: 00001001

Or'ing:
Register 1: 11010000
Register 2: 00001001
        or: 11011001

Completed state:
Register 1: 11011001
Register 2: N/A
```

This example only shows a single row, but for a 4 lane SIMD register, this would happen to 4 rows at a time. Note that the top lane of register 1, is compared to the top lane of register 2, and the second-top-most lane of register 1 is compared with the second-top-most lane of register 2. For the very first part of the transpose, this functions well, however when the transpose is at the 2x2 level instead of 4x4 (Still refering to the image), this doesn't work anymore. One would want to compare row 0 and 1 with row 2 and 3 now, instead of row 0 to 3 with row 4 to 7. This worked before, because the rows were in different registers, however now both row 0 and 1 and row 2 and 3 are in register 1. Therefore swizzling is perfomed, to move row 0, 1, 4 and 5 to register 1, and row 2, 3, 6 and 7 to register 2. For the entire process, the rows are swizzled around like this:

```
4x4:
Register 1   Register 2
Row 0        Row 4
Row 1        Row 5
Row 2        Row 6
Row 3        Row 7


2x2
Register 1   Register 2
Row 0        Row 2
Row 1        Row 3
Row 4        Row 6
Row 5        Row 7

1x1
Register 1   Register 2
Row 0        Row 1
Row 2        Row 3
Row 4        Row 5
Row 6        Row 7
```

The same and'ing and or'ing is happening in each process, but the mask and shift amount is changed per step:
```
4x4:
Mask:  0b11110000
Shift: 4

2x2:
Mask:  0b11001100
Shift: 2

1x1:
Mask:  0b10101010
Shift: 1
```
So far only the top register result has been shown, but the process is the same for the bottom register, however the mask is inverted, and the shift is left instead of right. Of course, the rows are in the wrong order when the transpose is finished, however this can be fixed with another swizzle.

# Performance
The code has been run on an AMD Ryzen 7 5800H. When using 128-bit SIMD registers, each transposition is about 13.4 nanoseconds, and when using 256-bit SIMD registers (AVX2), each transposition is about 10.5 to 10.7 nanoseconds. The code was also tested on an AMD Ryzen 9 9950x3D, which achieved results of 8.52 nanoseconds per transpose, when using 512-bit SIMD registers. 
Although I am not entirely sure, I think that there isn't really a single bottleneck that can be optimized away. I looked at the LLVM MCA timeline output, and it looks like most of the time is simply used on waiting for other instructions to finish — those instructions taking one clock cycle to compute each, so not much to be done there as far as I'm aware. 

# Resources
[Hacker Delight](http://www.icodeguru.com/Embedded/Hacker's-Delight/048.htm) (In case this is taken down, the book is called `Hackers Delight`, and the chapther is 7-3)
[Bit-Matrix transpose](https://github.com/dsnet/matrix-transpose?tab=readme-ov-file)

# Licensing