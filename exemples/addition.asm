/*
  This simple program does 2 + 2 and save the result on the ram adreess $0000
*/

ldx 2
ldy 2
add
stx $0000
hlt
