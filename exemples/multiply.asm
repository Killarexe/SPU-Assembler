/*
  This program multiply two 16-bit numbers
  and return the result in the X register.

  On this program it will be an exemple 5 * 2
*/

;Init values
ldx 0
stx $0001   ; result = 0
ldx 2
stx $0000   ; repeat = 2

mul_loop:
  ldx $0001
  ldy 5
  add       ; result += 5
  stx $0001
  ldx $0000
  ldy 1
  sub       ; repeat -= 1
  jiz end   ; if repeat == 0 { return result; }
  jmp mul_loop  ;else{ goto mul_loop; }
end:
  ldx $0001 ; return result;
  hlt
