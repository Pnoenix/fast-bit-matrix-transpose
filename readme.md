# Base idea
Transposes a 32x32 bit matrix *really* (Some might even say *blazingly*) fast. This is done through a modified divide and conquer approach. The general "formula" for transposing a matrix, is to move every (x, y) value to the (y, x) position. In code that is loosely inspired by everyones favourite snake language, it might look something like this:
```
matrix = [["A", "B"], ["C", "D"]]

for x in range(0, 2):
   for y in range(0, 2):
      temp = matrix[x][y]
      
      matrix[x][y] = matrix[y][x]
      matrix[y][x] = temp
```
While this algorithm functions just fine, there is also another way to do it. I have failed to find a specific name for this algorithm, but how it functions, is by recursively splitting the matrix into smaller and smaller squares, and then transposing those squares. On a side note, it turns out that the order in which you transpose these squares — big to small, small to big or anything in between — doesn't actually matter. Anyways, refer to this image stolen from [here](https://gudok.xyz/transpose/) ([archive](https://web.archive.org/web/20260215173009/https://gudok.xyz/transpose/)), for a visual explanation of the transpose (Due to a lack of other images on this page, please look at this image again when you feel there is too much text, and try to imagine that there are more images): 
![Matrix tranpose example](blocktree.png) 
While these two ways of transposing a matrix, would result in the same amount of operations, if the matrix consisted of bigger variables, like bytes or ints — that is not the case if the matrix is a *bit matrix*. The algorithm shown at the start, would have to isolate each bit separately, which is quite inefficient. Wouldn't it be nice if there was an algorithm, that would be better fit for bitwise operations? Imagine for a second, a 32x32 bit matrix. Below is shown the row at index 0, and the row at index 16:
```
    00: 0001000100010001_1100110011110000
01..16: ...
    16: 1111111101010101_1111111111111111
17..32: ...
```
A very creative bit matrix, with almost not repeating patterns! The first step in transposing the matrix, would be to swap the top right 16x16 square with the bottom left 16x16 square. What this means, in terms of the bits, is that one would want to swap the low 16 bits of row 0 through 15 with the high bits of row 16 through 31. To do this, one could swap the bits in row 0 with the bits in row 16, swap the bits in row 1 with the bits in row 17 and so on. To isolate the low bits of row 0, a mask can be used:
```
 Row 0: 0001000100010001_1100110011110000
  Mask: 0000000000000000_1111111111111111
     &
Equals: 0000000000000000_1100110011110000
```
Now these bits need to be placed in the high bits of row 16. This can be done by masking away the high bits of row 16, then shifting the low bits 16 places to the left, and or'ing these two together:
```
Row 16: 1111111101010101_1111111111111111
  Mask: 0000000000000000_1111111111111111
     &
Equals: 0000000000000000_1111111111111111
```
Now for shifting:
```
 Row 0: 0000000000000000_1100110011110000 
 << 16:
Equals: 1100110011110000_0000000000000000
```
And finally or'ing them together:
```
 Row 0: 1100110011110000_0000000000000000
Row 16: 0000000000000000_1111111111111111
     |
Equals: 1100110011110000_1111111111111111
```
As can now be seen, the low bits of row 0, have been so kind as to move into the high bits of row 16! How nice of them! To move the high bits of row 16 to the low bits of row 0, the opposite can be done — that is, to mask with the inverted mask (0b1111111111111111_0000000000000000), to shift row 16 instead of row 0, and to shift to the right instead of the left (For a complete transpose using this method, see the last section). That was quite easy, wasn't it? Now for swapping the 8x8 squares. The method is almost the same, but for good measure, I will show it too. Now row 0 is not to be swapped with row 16, but row 8 instead, and row 16 is to be swapped with row 24. Using another *completely* random matrix, it would look like this:
```
    00: 00010001_00010001_11001100_11110000
01..08: ...
    08: 11111111_01010101_11111111_11111111
09..32: ...
```
Note that the mask is now selecting every 8 bits, not every 16 bits:
```
 Row 0: 00010001_00010001_11001100_11110000
  Mask: 00000000_11111111_00000000_11111111
     &
Equals: 00000000_00010001_00000000_11110000
```
And for row 8:
```
 Row 8: 11111111_01010101_11111111_11111111
  Mask: 00000000_11111111_00000000_11111111
     &
Equals: 00000000_01010101_00000000_11111111
```
Shifting by 8 instead of 16 now:
```
 Row 0: 00000000_00010001_00000000_11110000 
  << 8:
Equals: 00010001_00000000_11110000_00000000
```
and finally or'ing:
```
 Row 0: 00010001_00000000_11110000_00000000
 Row 8: 00000000_01010101_00000000_11111111
     |
Equals: 00010001_01010101_11110000_11111111
```
As demonstrated above, it would take significantly less instructions to transpose the matrix this way, as opposed to masking one bit at a time, shifting it, and then masking the other row where it is supposed to go into, and then finally or'ing it in. Furthermore, when using SIMD registers, multiple rows can be operated on at once, which only makes it faster. The base idea was stolen from the Hacker's Delight book (See Resources), and adapted to fit better for SIMD.

# Implementation details related to SIMD registers
Due to this bit matrix transpose using SIMD registers, the data is swizzled around so that it can all be operated upon at once. This means that for the 4x4 transpose (In the image), one would imagine two SIMD registers one for the top 8x4 of the bits, and one for the bottom 8x4 of the bits. Using the image as a reference, the different swaps, would need the different rows to be in these different registers, for the swaps to be correct:

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
The rows would also need to be swapped around for a 32x32 row matrix in the same way, simply at a larger scale. Register 1 would hold row 0..16 for the 16x16 swap, row 0..8 and 16..24 for the 8x8 swap, row 0..4, 8..12, 16..20 and 24..28 for the 4x4 swap, and so on.

# Performance
The code was tested on an AMD Ryzen 7 5800H. When using 128-bit SIMD registers, each transposition is about 13.4 nanoseconds, and when using 256-bit SIMD registers (AVX2), each transposition is about 10.5 to 10.7 nanoseconds. The code was also tested on an AMD Ryzen 9 9950x3D, which achieved results of 8.52 nanoseconds per transpose, when using 512-bit SIMD registers. 
Although I am not entirely sure, I think that there isn't really a single bottleneck that can be optimized away. I looked at the LLVM MCA timeline output, and it looks like most of the time is simply used on waiting for other instructions to finish — those instructions taking one clock cycle to compute each, so not much to be done there as far as I'm aware. 

# Resources
[Hacker Delight](http://www.icodeguru.com/Embedded/Hacker's-Delight/048.htm) 
[archive](https://web.archive.org/web/20140125122743/http://icodeguru.com/Embedded/Hacker's-Delight/048.htm)<br>
[Bit-Matrix transpose](https://github.com/dsnet/matrix-transpose?tab=readme-ov-file) 
[archive]()<br>
[What it takes to transpose a matrix](https://gudok.xyz/transpose/) 
[archive](https://web.archive.org/web/20260215173009/https://gudok.xyz/transpose/)

# Licensing
The project is licensed under the MIT license, and can be used freely. 

# Full example of bits in registers
Below is a full example of how the bits move around in the registers, during the transpose. The example uses a 4x4 matrix, because I am writing it all by hand, but the concept is the same, regardless of size. Each bit is labeled A through P, so that it can be tracked. While what is shown below, is how the transpose is implemented in the code, the compiler does some intrinsic magic, which means that the assembly output does not perfectly reflect the code.

```
Layout:
Operation number  
||  
||  Where result is stored
||  |                  |
↓↓  ↓                  ↓ 
XX: Stored in Register x (Op): <-- The operation being done
A B C D <-- First operand
E F G H
Op      <-- Operation 
I J K L <-- Second operand (If needed)
M N O P
Equals:
A B C D <-- Result
E F G H

Input: 
A B C D
E F G H
I J K L
M N O P

Register 1:
A B C D
E F G H

Register 2:
I J K L
M N O P

00: Results stored in Register 3 (Register 1 & 1100):
A B C D
E F G H
&
1 1 0 0
1 1 0 0
=
A B 0 0
E F 0 0

01: Results stored in Register 4 (Register 2 & 1100):
I J K L
M N O P
&
1 1 0 0
1 1 0 0
=
I J 0 0
M N 0 0

02: Results stored in Register 4 (Register 4 >> 2):
I J 0 0
M N 0 0
>> 2
0 0 I J
0 0 M N

03: Results stored in Register 3 (Register 3 | Register 4):
A B 0 0
E F 0 0
|
0 0 I J
0 0 M N
=
A B I J
E F M N

04: Results stored in Register 2 (Register 2 & 0011):
I J K L
M N O P
&
0 0 1 1
0 0 1 1
=
0 0 K L
0 0 O P

05: Results stored in Register 1 (Register 1 & 0011):
A B C D
E F G H
&
0 0 1 1
0 0 1 1
=
0 0 C D
0 0 G H

06: Results stored in Register 1 (Register 1 << 2):
0 0 C D
0 0 G H
<< 2
C D 0 0
G H 0 0

07: Results stored in Register 4 (Register 1 | Register 2):
C D 0 0
G H 0 0
|
0 0 K L
0 0 O P
=
C D K L
G H O P

Intermediate results (Note that the original values in Register 1 and 2 have been overwritten)

Register 1:
C D 0 0
G H 0 0
Register 2:
0 0 K L
0 0 O P
Register 3:
A B I J
E F M N
Register 4:
C D K L
G H O P

Note that the values in Register 3 and 4 hold the results of the 2x2 swap, where the top right and bottom left 2x2 squares have been swapped. Now only a 1x1 swap is needed. When swizzling (The next step), imagine the rows/lanes in the SIMD registers as extending each other, as were they combined into an array:

08: Results stored in Register 1 (Swizzle Register 3 and Register 4, take row 0 and 2):
A B I J << This row taken
E F M N
Swizzle
C D K L << This row taken
G H O P
=
A B I J
C D K L

09: Results stored in Register 2 (Swizzle Register 3 and Register 4, take row 1 and 3):
A B I J
E F M N << This row taken
Swizzle
C D K L
G H O P << This row taken
=
E F M N
G H O P

10: Results stored in Register 3 (Register 1 & 1010):
A B I J
C D K L
&
1 0 1 0
1 0 1 0
=
A 0 I 0
C 0 K 0

11: Results stored in Register 4 (Register 2 & 1010):
E F M N
G H O P
&
1 0 1 0
1 0 1 0
=
E 0 M 0
G 0 O 0

12: Results stored in Register 4 (Register 4 >> 1):
E 0 M 0
G 0 O 0
>> 1
0 E 0 M
0 G 0 O

13: Results stored in Register 3 (Register 3 | Register 4):
A 0 I 0
C 0 K 0
|
0 E 0 M
0 G 0 O
=
A E I M
C G K O

14: Results stored in Register 2 (Register 2 & 0101):
E F M N
G H O P
&
0 1 0 1
0 1 0 1
=
0 F 0 N
0 H 0 P

15: Results stored in Register 1 (Register 1 & 0101):
A B I J
C D K L
&
0 1 0 1
0 1 0 1
=
0 B 0 J
0 D 0 L

16: Results stored in Register 1 (Register 1 << 1):
0 B 0 J
0 D 0 L
<< 1
B 0 J 0
D 0 L 0

17: Results stored in Register 4 (Register 1 | Register 2):
B 0 J 0
D 0 L 0
|
0 F 0 N
0 H 0 P
=
B F J N
D H L P

18: Results stored in Register 1 (Swizzle Register 3 and Register 4, take row 0 and 2):
A E I M << This row taken
C G K O
Swizzle
B F J N << This row taken
D H L P
=
A E I M
B F J N

19: Results stored in Register 2 (Swizzle Register 3 and Register 4, take row 1 and 3):
A E I M 
C G K O << This row taken
Swizzle
B F J N 
D H L P << This row taken
=
C G K O
D H L P

Final results:
Register 1:
A E I M
B F J N
Register 2:
C G K O
D H L P

Output:
A E I M
B F J N
C G K O
D H L P
```

Each swap consists of two "parts" (Using the term parts loosely). The first part, where the top transposition is done, and the second part where the bottom transposition is done. A top transposition can be seen from step 00 to and including 03 — here the bottom left 2x2 square is transposed to the top right 2x2 square. The results of this "top transpose" are temporarily stored in Register 3. After this, a bottom transposition is performed from step 04 to and including 07. During this transposition, the 1 and 2 registers are used as temporary registers, whereas the 3 and 4 registers are used as temporary registers during the top transposition. The results of the bottom transposition are stored in Register 4. Register 3 and 4 can now be moved back to Register 1 and 2, and the process of swapping temporary/storing registers repeats.