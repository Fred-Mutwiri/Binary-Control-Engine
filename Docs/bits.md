get_bit(value, position) -> u8

now the algorithm for getting the bit value / How do we get the specific value

You have a binary, say 1 0 1 1 0 1 1 0 and you want to access the bit at position 2.
These are the bit positions:

bit 0  => 1
bit 1  => 0
bit 2  => 1
bit 3  => 1
bit 4  => 0
bit 5  => 1
bit 6  => 1
bit 7  => 0

now:
1. create the mask for the specific position by left shifting 1, by the specified positions
 let mask = 1u8 << position;
- this gives 0000_0100 where position is 2. it was 0000_0001 at position 0.
2. perform a BITWISE AND for both the value and the mask.
 value & mask;
 - this specifically extracts the bit at position 2.

  1 0 1 1 0 1 1 0
&
  0 0 0 0 0 1 0 0
  ---------------
  0 0 0 0 0 1 0 0

  so the extracted bit is 0000_0100 whose integral equivalent is 4.

/// Wait, so what will be returned? 1 or the value 4?

the function defination get bit is a bit ambigous in that regard so we create a more intentional function.

get_bit_bit( value: u8, position: u8 ) -> Option<u8> 
- given a value and a position, return either a value or None

let mask = 1u8 << position;
- here we create a mask by left shifting 1 by the number of position.

let bit_value = (value & mask) >> position;
- so after extracting the bit at the specific value, right shift the value by the position to get only the extracted value.
- iusing previous example: 
        0000_0100 
then 
        0000_0100 >> 2 = 0000_0001; 
so you only remain with the bit value extracted. either a 0  or a 1.



the Same principles apply for the rest of the operations. 
masking is the common for all.

getting a bit uses the Bitwise AND btwn the value and the mask
setting a bit uses the Bitwise OR btwn the value and the mask
clearing a bit uses the Bitwise NOT on the mask, then AND the value with the modified mask.
toggling a bit uses the Bitwise XOR btwn the value and the mask

