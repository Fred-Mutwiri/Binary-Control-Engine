
the population_count counts the number of ones in a binary:

value = 1011_0110

can see it receives the value but makes sure it can be modified, also create count to keep track of the number of 1's.

so now the mutable values are:

value = 1011_0110
count = 0

now in a loop of range 0 to 8-1:
 start with 0 do:
   1) value & 1 :

      1011_0110
    &  
      0000_0001
      _________
      0000_0000
    
    2) count = count + (value & 1)
       0 + 0000_0000

       = 0
    
    3) value >> 1:
      1011_0110 >> 1
      = 0101_1011

    Repeat for the complete range.

 Return the count at the end.

 that's a complete algorithm.



Now the second implementation is much more efficient.

so now have both variables:
value = 10110110
count = 0

now the value is 10110110
if value != 0 :
 1) 
    value - 1
     10110110
    - 
     00000001
    __________
     10110101

     so now the new_value = 10110101

 2) value & new_value
    
      10110110
    & 10110101
    ___________
      10110100
    
    now the lowest 1 bit is removed.

 3) value = 10110100

 4) 
    count = count + 1

Now repeat until all the ones are removed, and the value is 0.

so now no need to repetitively go through all the bits.

1. Take the value and subtract 1:
    - The resulting value will have a 0 on the lowest 1 but position and ones to the right
2. AND the resulting value with the original value. this will turn all the ones to the right to 0 and the lowest bit position will remain 0.
3. Assign the resulting value to the variable value
4. then now perform a countinous addition of 1 to the variable count.

finally return the count.