//program 1, multiplicative inverse
//getting the values out of memory
mov #8      //move memloc 8 into acc
wrt r7      //write that value into r8
ldr [r7]    //load memloc[r8] into acc
wrt r0      //write MSB to r0
mov #9      //move memloc 9 into acc
wrt r7      //write that value into r8
ldr [r7]    //load memloc[r8] into acc
wrt r1      //write LSB to r1

//check for zero
rdr r0      //load r0 value
cmp $zero   //check if r0 is zero
beq #3     //skip the LSB check

mov #14    //this is a hack
br          //because of 4 bit signed limit

rdr r1      //load r1 value
cmp $zero   //check if r1 is zero
beq #3     //skip the finish

mov #9     //this is a hack
br          //becasue of 4 bit signed limit

//stop if 0 is passed in
mov #0xff   //load max value into acc
str [r7]    //load max value into memloc #9
mov #8      //load memloc 8 into acc
wrt r7      //load memloc 8 into r8
mov #0xff   //load max value into acc
str [r7]    //load max value into memloc #8

mov #60   //offset from here to end (num(inst) - 1)
br

//initialize values
mov #0     //MSB (0x0	0)
wrt r2      //r2 is MSB
mov #1     //LSB (0x01)
wrt r3      //r3 is LSB

mov #0x80  //r4 is going to be used to set Q[i] to 1
wrt r4      //save to r4

mov #0x00  //r5 is going to count the number of times r4 reaches zero
wrt r5      //will stop the loop when r > 1

mov #0     //set r7 | r8 to be zero
wrt r7
wrt r8

//INPUT(R)   : r0 | r1
//DIVISOR(N) : r2 | r3
//RESULT(Q)  : r7 | r8
//Q_SETTER   : r4
//COUNTER    : r5

//FOR_LOOP:
//if R <= N then ...
rdr r0      //MSB{R}
cmp r2      //MSB{N}
bne #4     //skip the lsb check

rdr r1      //LSB{R}
cmp r3      //LSB{N}
ble #8     //skip the else step
mov #4
br

rdr r0      //MSB{R}
cmp r2      //MSB{N}
ble #3      //skip the else step

//else body, won't do anything cuz r7 | r8 are zero by default
mov #29    //skip to shift
br          //byeeeeee

rdr r4
cmp $ZERO   //is r4 zero? gotta reset it if it is
bne #6

mov #1     //increment
add r5      //r4
wrt r5
mov #0x80  //reset r4
wrt r4

mov #0     //compare r5 against 0
cmp r5      //if !=, then we work on LSB
bne #7     //skip over MSB setting

rdr r4
orr r7
wrt r7
mov #4
br         //skip over LSB setting

rdr r4
orr r8
wrt r8

//done with MSB/LSB setting
rdr r4
lsr #1
wrt r4

//N := N - R
//N[0] := N[0] - R[0]
rdr r3
sub r1
wrt r3

//N[1] := N[1] - R[1]
rdr r2
sbc r0
wrt r2

//N << 1
rdr r2      //r2 << 1
lsl #1     //r2 << 1
wrt r2      //r2 << 1

mov #0x7f  // r3 & #0x7f
and r3      // r3 & #0x7f
lsr #7     // lsr $acc #7
and r2      // and $acc r2 (complete the shift)
wrt r2      // write to r2
rdr r3      // r3 << 1
lsl #1     // r3 << 1
wrt r3      // r3 << 1

mov #10
wrt r0
rdr r7
str [r0]

mov #11
wrt r0
rdr r8
str [r0]

halt
halt
