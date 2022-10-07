/*
    w_i: current digit
    optype: op_i: 1st div in block: `div z <op>`, can be `1` or `26`
    correction: corr_i: add after 1st div: `add x <corr>`
    offset: off_i: 3rd to last instruction in block: `add y <off>`

    the operations lay out a stack data structure, we can pair up 'pushes' (op = 1)
    and 'pops (op = 26)

    in my case, this was the pairing:
    1 - 14
    2 - 13
    3 - 6
    4 - 5
    7 - 8
    9 - 10
    11 - 12

    A block does the following:
    if optype == 1: (push)
        if (z % 26) + corr_i != w_i
            z *= 26 // shift values to the left, down the stack
            z += (w_i + off_i) // add new value to the top
        end
    if optype == 26: (~pop)
        if (z % 26) + corr_i != w_i
            z /= 26 // shift values to the right, letting go of the top element
            z *= 26 // shift everything left again
            z += (w_i + off_i) // push this value to the top instead
        else
            z /= 26 // just pop the top element off without adding another

    This means that whatever we push in the push section, we can forget it
    if `(z % 26) + corr_i == w_i`.
    As `z` starts out at `0`, we know it's value after the first push will be `w_i + off_i`.
    Meaning that if we want to nullify this, we need the following condition to be true:
    `z = w_i + off_i` and `(z % 26) + corr_j == w_j` ->
    `(w_i + off_i % 26) + corr_j == w_j)`, but as the max offset value is `16` and the
    max value for `w` can be `9`, we can get rid of the modulo:
    `(w_i + off_i) + corr_j = w_j`.

    Re-writing the parens to help us organizing a bit more:
    w_i + (off_i + corr_j) = w_j where i and j are pairs in the machine code.

    This leads us to the following linear programming problem:
    w1 + 4 = w14
    w2 + 8 = w13
    w3 - 6 = w6 -> w6 + 6 = w3
    w4 + 6 = w5
    w7 - 2 = w8 -> w8 + 2 = w7
    w9 - 1 = w10 -> w10 + 1 = w9
    w11 + 0 = w12
    1 <= w_i <= 9, forall i in [1, 14]

    Which would be quite hard to calculate by hand, however, as each `w` value
    can only be a digit, it's quite easy to do in practice.
*/

/*
    By calculating the maximum value for each variable:
    w1 = 5
    w2 = 1
    w3 = 9
    w4 = 3
    w5 = 9
    w6 = 3
    w7 = 9
    w8 = 7
    w9 = 9
    w10= 8
    w11= 9
    w12= 9
    w13= 9
    w14= 9

    We get this number:
*/
fn find_largest_number_accepted_by_monad() -> u64 {
    51939397989999
}

/*
    By calculating the minimum value for each variable:
    w1 = 1
    w2 = 1
    w3 = 7
    w4 = 1
    w5 = 7
    w6 = 1
    w7 = 3
    w8 = 1
    w9 = 2
    w10= 1
    w11= 1
    w12= 1
    w13= 9
    w14= 5

    We get this number:
*/
fn find_smallest_number_accepted_by_monad() -> u64 {
    11717131211195
}

fn main() {
    println!(
        "The largest number accepted by MONAD is {}",
        find_largest_number_accepted_by_monad()
    );

    println!(
        "The smallest number accepted by MONAD is {}",
        find_smallest_number_accepted_by_monad()
    );
}
