// TODO load from mem

// load from address 16 to r0
mov #13
wrt r7
ldr [r7]
wrt r0

// load from address 17 to r1
mov #14
wrt r7
ldr [r7]
wrt r1

//short isqrt(short num) {
//    short res = 0;
//    short bit = 1 << 14; // The second-to-top bit is set: 1 << 30 for 32 bits
// 
//    // "bit" starts at the highest power of four <= the argument. 
//  // FIRST WHILE
//    while (bit > num)
//        bit >>= 2;
//        
//    while (bit != 0) {
//        if (num >= res + bit) {
//            num -= res + bit;
//            res += bit << 1;
//        }
//        
//        res >>= 1;
//        bit >>= 2;
//    }
//    return res;
//}
//
//      MSB | LSB
// NUM:  r0 | r1
// RES:  r2 | r3
// BIT:  r4 | r5
// TEMP: r6 | r7
// FLAG: r8

// 0 out the registers used
mov #0
wrt r2
wrt r3
wrt r4
wrt r5
wrt r6
wrt r7
wrt r8

// set BIT to 01000...
mov #1
lsl #6
wrt r4

// FIRST LOOP,
// while (bit > num)
// bit >>= 2

// go to msb_loop_cond
// br msb_loop_cond
mov #4
br

msb_loop:
// MSB{bit} = MSB{BIT >> 2}
rdr r4
lsr #2
wrt r4

msb_loop_cond:
// if MSB{bit} > MSB{num}
rdr r4
cmp r0
bgt #-5 // go to msb_loop

// if MSB{bit} > 0
// done shifting if true
rdr r4
cmp $zero 
beq #3 // if MSB is 0, need to shift into LSB
br end_shift_loop

// set the bit in the LSB
mov #1
lsl #6
wrt r5
br lsb_shift_loop_cond

lsb_shift_loop:
rdr r5
lsr #2
wrt r5

lsb_shift_loop_cond:

// if LSB{bit} > LSB{num} 
rdr r5
cmp r1
bgt #-5 // go to lsb_shift_loop

end_shift_loop:

// SECOND LOOP

br bit_loop_cond

bit_loop:
// temp = res + bit
// LSB{temp} = LSB{res} + LSB{bit}
rdr r3
add r5
wrt r7

// MSB{temp} = MSB{res} + MSB{bit} + carry
rdr r2
adc r4
wrt r6

// compare num to res + bit
// if MSB{num} > MSB{temp} enter if
rdr r0
cmp r6
ble #3 // continue if msb is <=
br if_num_ge

// if MSB{num} != MSB{temp} num must be less
rdr r0
cmp r6
beq #3 // continue if numbers are same
br end_if_num_ge

// if LSB{num} >= LSB{temp}
rdr r7
cmp r1
ble #3 // go to if_num_ge

// go to end_if_num_ge
br end_if_num_ge

if_num_ge:

// num -= res + bit
// LSB{num} = LSB{temp} - LSB{num}
rdr r1
sub r7
wrt r1

// MSB{num} = MSB{temp} - MSB{num} - carry
rdr r0
sbc r6
wrt r0

// MSB{temp} = MSB{bit} << 1
rdr r4
lsl #1
wrt r6

// LSB {temp} = LSB{bit} << 1
rdr r5
lsl #1
wrt r7

// res += temp (bit << 1)
// LSB{res} += LSB{temp}
rdr r3
add r7
wrt r3

// MSB{res} += LSB{temp} + carry
rdr r2
adc r6
wrt r2

end_if_num_ge:

// res >>= 1
// get bottom bit of MSB{res} into r6
rdr r2
lsl #7
wrt r6

// MSB{res} >> 1
rdr r2
lsr #1
wrt r2

// LSB{res} >> 1, top bit set to bottom of msb
rdr r3
lsr #1
orr r6 // add in bottom bit of MSB{res}
wrt r3

// shift bit right 2
rdr r4
lsl #6
wrt r6

rdr r4
lsr #2
wrt r4

rdr r5
lsr #2
orr r6
wrt r5

bit_loop_cond:
rdr r4
cmp $zero
beq #3
br bit_loop // start of loop

rdr r5
cmp $zero
beq #3
br bit_loop // start of loop

// END OF LOOP

// ROUNDING part

// copy RES into r9
// it's max 1 register so we only need 1
rdr r3
wrt r9

// NUM << 2
//get top 2 bits of LSB{num}
rdr r1
lsr #6
wrt r6

// MSB{num} << 2
rdr r0
lsl #2
orr r6 // copy over bits from LSB reg
wrt r0

// LSB{num} << 2
rdr r1
lsl #2
wrt r1

// res << 2
//get top 2 bits of LSB{res}
rdr r3
lsr #6
wrt r6

// MSB{res} << 2
rdr r2
lsl #2
orr r6
wrt r2

// LSB{res} << 2
rdr r3
lsl #2
wrt r3

// add 1 to res and carry
mov #1
wrt r7

// LSB{res} += 1
rdr r3
add r7
wrt r3

// MSB{res} += carry
rdr r2
adc r4 // HACK (r4 should be 0, but not guaranteed really)
wrt r2

// if MSB{res} <= MSB{num} continue
rdr r2
cmp r0
ble #3 // continue
br done // we know res > num or num < res, exit

// we know res <= num, num >= res in msb
// if MSB{num} != MSB{res} same as MSB{res} 
rdr r0
cmp r2
beq #3 // MSB{num} == MSB{res} continue
br round_up

// check lsb
rdr r3
cmp r1
bgt jump_to_done // exit, res > num, or num < res

round_up:
rdr r9
wrt r6
mov #0xff
cmp r6 
jump_to_done: 
beq done
rdr r6 
add r7
wrt r9

done:

mov #15
wrt r7
rdr r9
str [r7]

halt









