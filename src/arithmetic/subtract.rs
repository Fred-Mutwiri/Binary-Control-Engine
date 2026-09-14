use super::add::add;

// pub fn subtract( a: u8, b: u8 ) -> (u8, u8) {
//     let inverted = !b;

//     let ( negative_b, carry1 ) = add(inverted, 1);
//     let ( result, carry2 ) = add(a, negative_b);

//     let borrow =  if carry1 == 1 && carry2 == 1  {
//         0
//     } else {
//         1
//     };

//     ( result, borrow )
// }


pub fn subtract(a: u8, b: u8) -> (u8, bool) {
    let inverted = !b;

    let (negative_b, _) = add(inverted, 1);

    let (result, carry) = add(a, negative_b);

    let borrow = carry == 0;

    (result, borrow)
}