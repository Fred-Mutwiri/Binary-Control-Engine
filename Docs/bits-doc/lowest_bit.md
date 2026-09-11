alright now
given a value = 10110110

1. value == 0
    skip this and return None
    means that it has no set bit

2. given a range of 8 positions
    a) create a mask by shifting the 1 by the value of position

        if position == 0
            0000_0001 << 0  = 0000_0001
    
    b) AND the value and the mask and check if there is a value which is not 0. if the check is true return the mask for the given position and break out of the loop. 

3. if there loop ends without a valid mask, return None

this is a lowest set bit algorithm.


alright a bit more efficient algorithm:

1) check if the value is 0. Then return None.
2) calculate the two's complement, which is the negative of the number
3) AND both the value and the negated_value.


value = 10110110

negated_value = {
    a) !value = 01001001
    b) !value + 1 = 01001001 + 1 = 01001010
}

negated_value = 01001010

value & negated_value

  10110110
& 01001010
___________
  00000010


