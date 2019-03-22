//////////////////////////////////////////
// mem[0] | mem[1]  = dividend          //
// mem[2]           = divisor           //
// mem[4] | mem[5]  = quotient          //
// mem[6]           = fractional        //
//////////////////////////////////////////

//read dividend
mov #0
wrt r7
ldr [r7]
wrt r8 // MSB of dividend

mov #1
wrt r7
ldr [r7]
wrt r9 //LSB of dividend

//read divisor
mov #2
wrt r7
ldr [r7]
wrt r0  //save divisor to r0

cmp $zero
bne init

br div_by_zero

//////////////////////////////////////
// r0           = divisor           //
// r1           = quotient          //
// r2 | r3      = R                 //
// r4           = flag              //
// r5           = setter            //
// r6, r7       = temp              //
// r8 | r9      = dividend (moved)  //
//////////////////////////////////////
init:
mov #0
wrt r1
wrt r2
wrt r3
wrt r4

mov #0x80
wrt r5
br loop

div_by_zero:
mov #4
wrt r7
mov #0xff
str [r7]

mov #5
wrt r7
mov #0xff
str [r7]

mov #6
wrt r7
mov #0xff
str [r7]

halt

loop:
// read R_LSB[7], if it is 1, then carry it over to R_MSB
left_shift_R:
rdr r2      //MSB of R
lsl #1      //left shift 1
wrt r2      //re-write to R_MSB

rdr r3      //read LSB
lsr #7      //$acc will either be 0 or 1
orr r2      //or with MSB of R
wrt r2      //write back to R_MSB

rdr r3      //read LSB
lsl #1      //left shift 1
wrt r3      //write back to LSB_R

//check flag, based on flag, MSB/LSB/fractional
//prep step for R(0) := N(i)
flag_check:
mov #0
cmp r3
bne not_MSB

br MSB

not_MSB:
mov #1
cmp r3
bne jump_to_REM

br LSB

jump_to_REM:
br REM:

//R(0) := N(i); Set the least-significant bit of R equal to bit i of the numerator
MSB:
rdr r8  //MSB of dividend
and r5  //setter. $acc will be 1 if we can set R(0) = 1
cmp $zero
beq no_set_MSB

mov #1  //lsb
orr r3  //write lsb to R_LSB
wrt r3  //write back to R_LSB

no_set_MSB:
br R_SET_DONE

LSB:
rdr r7  //LSB of dividend
and r5  //setter. $acc will be 1 if we can set R(0) =
cmp $zero
beq no_set_LSB

mov #1  //lsb
orr r3  //write lsb to R_LSB
wrt r3  //write back to R_LSB

no_set_LSB:
br R_SET_DONE

REM:
mov #0  //stand-in for remainder
and r5  //setter. $acc will be 1 if we can set R(0) =
cmp $zero
beq no_set_REM

mov #1  //lsb
orr r3  //write lsb to R_LSB
wrt r3  //write back to R_LSB

no_set_REM:
br R_SET_DONE

intermed_jump_back_to_loop:
br loop:

//if R >= D check happens here
R_SET_DONE:
rdr r2
cmp $zero
bgt if_body

//r2 is zero. so have to check r3 (R_LSB) and D
rdr r0  //get divisor
cmp r3  //cmp divisor against R_LSB
ble if_body

br finish_loop

if_body:
//R := R - D
rdr r3  //R_LSB
sub r0  //R_LSB - D
wrt r3  //result into R_LSB

rdr r2      //R_MSB
sbc $zero   //dont matter
wrt r2      //write back to R_MSB

//Q(i) := 1
rdr r1  //read reg that holds Q
orr r5  //orr with setter
wrt r1  //write to Q

finish_loop:    //clean up, shift of setter and change of flag if nec
//shifting setter
rdr r5
lsr #1
wrt r5

//checking if setter is 0, shifted setter is still in $acc
cmp $zero
beq reset_setter

br intermed_jump_back_to_loop

reset_setter:
mov #0x80   //init setter value
wrt r5      //write to r5

rdr r4      //read flag
cmp $zero   //if zero, write Q to MSB mem loc
bne not_MSB_Q

br MSB_Q

not_MSB_Q:
mov #1      //LSB flag value
cmp r4      //if 1, write Q to LSB mem loc
bne not_LSB_Q

br LSB_Q

not_LSB_Q:
br frac_Q

MSB_Q:
mov #4  //memloc for MSB
wrt r7  //write to r7
rdr r1  //read Q
wrt r12 //for debugging
str [r7]

mov #1  //increment flag
add r4  //cuz thats important
wrt r4  //who knew :shrug:
br intermed_jump_back_to_loop

LSB_Q:
mov #5  //memloc for LSB
wrt r7  //write to r7
rdr r1  //read Q
wrt r13 //for debugging
str [r7]

mov #1  //increment flag
add r4  //cuz thats important
wrt r4  //who knew :shrug:
br intermed_jump_back_to_loop

frac_Q:
mov #6  //memloc for frac
wrt r7  //write to r7
rdr r1  //read Q
wrt r14 //for debugging
str [r7]

rounding:
br done

done:
halt
