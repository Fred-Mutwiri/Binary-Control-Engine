alright now looking at how to reverse bits

value = 10110110

result = 0


position in a loop of range 0 .. 8 == [ 0,1 2, 3, 4, 5, 6, 7 ] :{
    position = 0: 
    {
        bit = (value >> position) & 1;

        10110110 >> position  = 10110110 >> 0 = 10110110
        10110110
        & 00000001
        ____________
        00000000

        result |= bit << (7- position);

        bit= 00000000
        result = result | bit << (7 - 0)
        result = 00000000 | 00000000 << 7

        result = 00000000
    }

    position = 1:
    {
        bit = (value >> position) & 1;
        value = 10110110

        10110110 >> 1 = 01011011
            01011011
          & 00000001
           __________
            00000001

        result |= bit << (7 - position)
        bit = 0000_0001

        result = result |  bit <<  (7 - 1)
        result = 0000_0000 | 0000_0001 << 6
        result = 0000_0000 | 0100_0000 
        result = 0100_0000
    }

continue until the loop is complete

}

now return the result.