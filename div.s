// division
// mem[0] | mem[1]  = dividend
// mem[2]           = divisor

// mem[4] | mem[5]  = quotient
// mem[6]           = quotient fraction

//loading arguments
mov #0
wrt r7
ldr [r7]
wrt r0      //loaded mem[0] to r0

mov #1
wrt r7
ldr [r7]
wrt r1      //loaded mem[1] to r1

mov #2
wrt r7
ldr [r7]
wrt r2      //loaded mem[2] to r2

// r0 | r1 = R = N
// r2 | r3 = D << n

// zero divisor check
mov #0
cmp r2
bne not_zero_divisor

br zero_divisor_intermed1

//check zero dividend. #0 still in acc
not_zero_divisor:
cmp r0
bne not_zero_dividend

cmp r1
bne not_zero_dividend

br zero_dividend_intermed1

not_zero_dividend:
mov #1                  //lets check if divisor is 1
cmp r2
bne check_equal

br divisor_is_one_intermed1

//check if dividend == divisor
check_equal:
mov #0
cmp r0
bne init

rdr r1      //check if r1 == r2
cmp r2      //if so, return one
bne init    //else, go to init

br return_one_intermed1

init:
mov #0x80     //for setting q[i] = 0/1
wrt r3

//////////////////////////////////////////////////
// r0 | r1  = MSB | LSB of dividend             //
// r2 | 0   = MSB | LSB of divisor (D << 8)     //
// r3       = setter for q[i]                   //
// r4       = 0 or 1. flag for MSB/LSB          //
// r5 | r6  = quotient                          //
// r7       = temp reg                          //
//////////////////////////////////////////////////

loop:       //for i = n-1 .. 0
//R := 2 * R. common to both if/else
mov #0x80
and r1      //extract the MSB
wrt r7      //save it to r7
rdr r0      //read MSB
lsl #1      //left shift  1
orr r7      //orr with msb of LSB
wrt r0      //write to r0

rdr r1      //read LSB
lsl #1      //left shift 1
wrt r1      //write back to r1

br r_check

//hack cuz 8 bit limit
divisor_is_one_intermed1:
br divisor_is_one_intermed2

zero_dividend_intermed1:
br zero_dividend_intermed2

zero_divisor_intermed1:
br zero_divisor_intermed2

return_one_intermed1:
br return_one_intermed2

//if R >= 0 then
r_check:
mov #0
cmp r0
ble r_le_zero

br if_body

//R <= 0
r_le_zero:
cmp r0
beq LSB_check

br else // R < 0

LSB_check:
cmp r1
bgt if_body //R > 0

cmp r1
beq if_body // R >= 0

br else     // R < 0

if_body:
mov #0
cmp r4
beq msb_setting

br lsb_setting

msb_setting:
rdr r5      //get MSB of q
orr r3      //orr with setter
wrt r5      //set MSB of q
br sub      //skip over lsb_setting

lsb_setting:
rdr r6      //get LSB of q
orr r3      //orr with setter
wrt r6      //set LSB of q

sub:        //R -= D (R is already 2 * R)
rdr r0      //read MSB of R, to be subbed from D
sub r2      //subtract D
wrt r0      //write the result to MSB of R
br skip_intermed

divisor_is_one_intermed2:
br divisor_is_one

zero_dividend_intermed2:
br zero_dividend

zero_divisor_intermed2:
br zero_divisor

return_one_intermed2:
br return_one

skip_intermed:
br cleanup

else:       //R += D
rdr r0      //read MSB of R
add r2      //add D
wrt r0      //write to R

cleanup:    //shift the values, set flags

mov #0      //compare shifter against 0
cmp r3
beq reset_setter

rdr r3      //right shift setter
lsr #1      //right shift setter
wrt r3      //right shift setter
br loop     //restart loop

reset_setter:
cmp r4      //compare against flag
bne finish_loop //if flag is already 1, finish loop

mov #1
wrt r4      //set flag to 1
mov #0x80
wrt r3      //reset setter
br loop

finish_loop:    //do finishing touches here
//Q := Q - bit.bnot(Q)
mvn r5      //write ~Q_MSB to r3
wrt r3      //store in r3
rdr r5      //write Q_MSB to $acc
sub r3      //Q_MSB - ~Q_MSB
wrt r5      //write to Q_MSB

//LSB
mvn r6      //write ~Q_LSB to r4
wrt r4      //store in r4
rdr r6      //write Q_LSB to $acc
sub r4      //Q_LSB - ~Q_LSB
wrt r6      //write to Q_LSB

//STORE MSB | LSB of quotient to memory so i have more free registers
mov #4
wrt r7
rdr r5
str [r7]

mov #5
wrt r7
rdr r6
str [r7]

//r3 - r7 are free registers now
mov #0x7f   //store largest positive number
wrt r3      //into r3

rdr r0      //check MSB of R
cmp r3      //against r3
ble remainder   //if greater than, no need to worry about it

mov #6  //store 0 into remainder
wrt r7
mov #0
str [r7]
br done //get out of here

remainder:
//Q := Q - 1; LSB
mov #1
wrt r7
rdr r6
sub r7

//Q := Q - 1; MSB
mov #0
wrt r7
rdr r5
sbc r7

//update in memory
mov #4
wrt r7
rdr r5
str [r7]

mov #5
wrt r7
rdr r6
str [r7]

//R := R + D
rdr r0  //read MSB of R
add r2  //add MSB of D
wrt r0

mov #6
wrt r7
rdr r0
str [r7]
br done

zero_divisor:
mov #4
wrt r7
mov #0xFF
str [r7]

mov #5
wrt r7
mov #0xFF
str [r7]

mov #6
wrt r7
mov #0xFF
str [r7]

br done

zero_dividend:
mov #4
wrt r7
mov #0
str [r7]

mov #5
wrt r7
mov #0
str [r7]

mov #6
wrt r7
mov #0
str [r7]

br done

divisor_is_one:
mov #4
wrt r7
rdr r0
str [r7]

mov #5
wrt r7
rdr r1
str [r7]

mov #6
wrt r7
mov #0
str [r7]

br done

return_one:
mov #4
wrt r7
mov #0
str [r7]

mov #5
wrt r7
mov #0x1
str [r7]

mov #6
wrt r7
mov #0
str [r7]

done:
halt
halt
