/*
  Do the fibonacci sequence.
  The end result goes into the ram address $0001.
*/

;Init variables
ldx 0
stx $0001   ; prev1 = 0
ldx 1
stx $0002   ; prev2 = 1
ldx 9
stx $0000   ; n = 9

loop:
  ldy 3
  sub
  ldy 128
  add
  jic stop  ; if n < 3 { stop(); }

  ldx $0002
  ldy $0001
  add
  sty $0002 ; prev2 = prev1
  stx $0001 ; prev1 = prev2 + prev1

  ldx $0000
  ldy 1
  sub
  stx $0000 ; n -= 1

  jmp loop
stop:
  hlt
