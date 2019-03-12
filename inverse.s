//program 1, multiplicative inverse
//getting the values out of memory
mov #8      //move memloc 8 into acc
wrt r7      //write that value into r6
ldr [r7]    //load memloc[r6] into acc
wrt r0      //write MSB to r0
mov #9      //move memloc 9 into acc
wrt r7      //write that value into r6
ldr [r7]    //load memloc[r6] into acc
wrt r1      //write LSB to r1

//check for zero
rdr r0      //load r0 value
cmp $zero   //check if r0 is zero
beq #3     //skip the LSB check

mov #14    //this is a hack
br         //because of 4 bit signed limit

rdr r1      //load r1 value
cmp $zero   //check if r1 is zero
beq #3     //skip the finish

br init 

//stop if 0 is passed in
mov #0xff   //load max value into acc
str [r7]    //load max value into memloc #9
mov #8      //load memloc 8 into acc
wrt r7      //load memloc 8 into r6
mov #0xff   //load max value into acc
str [r7]    //load max value into memloc #8

br END

//initialize values
INIT: 
mov #0     //MSB (0x0	0)
wrt r2      //r2 is MSB
mov #1     //LSB (0x01)
wrt r3      //r3 is LSB

mov #0x80  //r4 is going to be used to set Q[i] to 1
wrt r4      //save to r4

mov #0x00  //r5 is going to count the number of times r4 reaches zero
wrt r5      //will stop the loop when r > 1

mov #0     //set r7 | r6 to be zero
wrt r7
wrt r6

//INPUT(R)   : r0 | r1
//DIVISOR(N) : r2 | r3
//RESULT(Q)  : r7 | r6
//Q_SETTER   : r4
//COUNTER    : r5

//FOR_LOOP:
//if R <= N then ...
LOOP:
rdr r0      //MSB{R}
cmp r2      //MSB{N}
ble #3      //skip to else body 

br ELSE

// R_MSB <= N_MSB 
rdr r2      //MSB{N}
cmp r0      //MSB{R}
beq #3     //check LSB. since R[0] <= N[0]; if R == N, we must check LSB

mov #6
br

// R_MSB == N_MSB
rdr r1      //LSB{R}
cmp r3      //LSB{N}
ble #3      //if LSB{R} <= LSB{N}, then if step

//else body, won't do anything cuz r7 | r6 are zero by default
ELSE:
br SHIFT   //byeeeeee

mov #0     //compare r5 against 0
cmp r5      //if !=, then we work on LSB
bne #6     //skip over MSB setting

rdr r4
orr r7
wrt r7
mov #4
br         //skip over LSB setting

rdr r4
orr r6
wrt r6

//done with MSB/LSB setting

//N := N - R
//N[0] := N[0] - R[0]
rdr r3
sub r1
wrt r3

//N[1] := N[1] - R[1]
rdr r2
sbc r0
wrt r2

// SHIFTING HERE, END IF
//N << 1
SHIFT:
rdr r2      //r2 << 1
lsl #1     //r2 << 1
wrt r2      //r2 << 1

mov #0x80  // r3 & #0x7f
and r3      // r3 & #0x7f
lsr #7     // lsr $acc #7
orr r2      // and $acc r2 (complete the shift)
wrt r2      // write to r2
rdr r3      // r3 << 1
lsl #1     // r3 << 1
wrt r3      // r3 << 1

// q_ setter>> 1 to be used in next loop
rdr r4
lsr #1
wrt r4

rdr r4
cmp $ZERO   //is r4 zero? gotta reset it if it is
bne #6

mov #1     //increment
add r5      //r4
wrt r5
mov #0x80  //reset r4
wrt r4

// if counter > 1 (or 1 < counter, or 2 <= counter) exit loop
mov #2
cmp r5
ble #3 // exit loop

br LOOP

// loop end
// write to memory the results
END:
mov #10
wrt r0
rdr r7
str [r0]

mov #11
wrt r0
rdr r6
str [r0]

halt
halt

