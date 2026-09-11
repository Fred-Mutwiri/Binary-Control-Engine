Due to repeated masking in the bits section, it's good to separate it into its own concept so all masks are in the same place.

the Masking algorithm:

simple enough its to left shift the 1u8 (for a u8 ofcourse) by the value in positions. this is used to isolate or rather extract the specific value in position.

say I want to create a mask for the position 4:

1)  0000_0001 <<  position;
2)  0000_0001 << 4; 

- 0000_0001 << 0 = 0000_0001
- 0000_0001 << 1 = 0000_0010
- 0000_0001 << 2 = 0000_0100
- 0000_0001 << 3 = 0000_1000
- 0000_0001 << 4 = 0001_0000
- 0000_0001 << 5 = 0010_0000
- 0000_0001 << 6 = 0100_0000
- 0000_0001 << 7 = 1000_0000


Now, from the table, left shifting the value by the value in position you place the 1 being shifted in the position. this is why you cannot left shift by 8 in a u8, it will overflow. the value shifted by 8 is not accomodated by a u8.

## Mulitple Bit mask

0001_1100 -- this is a Field Mask

a mask basically describes which bits i want.
so here we want from bit 2 to bit 4. 

# the Algorithm:

bit 2 -> 4 that's a width of  (2, 3, 4 ) = 3
width = 3

  3 
2   = 8  ==>  (1 << 3) = 0000_1000


to create the 111 you subtract 1:
  3
2  - 8 = 7  ==>  (1 << 3) - 1  = 7


  8 : 0000_1000
-
  1 : 0000_0000
________________
  7 : 0000_0111


so then we can basically left shift it by the start position:

  0000_0111 << 2  =  0001_1100


so basically this is the algorithm for validated: 
    ( (1 << 3) - 1 ) << 2

# Note:
validation always comes right before the bit manipulation.
1. check if the width is invalid. for a u8, anything greater than the 8 is invalid.
2. check if the start is invalid. for a u8, if the start bit is greater than the 8 it's definitely invalid. 
3. check if the sum of both start bit and width is greater than 8. if it is, it is invalid. 

- by Invalid I mean out of the given scope. currently right now it's u8 scope.