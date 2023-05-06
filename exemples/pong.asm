;Init the variables
ldx 1
stx $0000   ;int velX = 1;
stx $0001   ;int velY = 1;
ldx 45
stx $0002   ;uint x = 45;
ldx 32
stx $0003   ;uint y = 32;

;Loop start here.
loop:
  
  ldy $0001
  add
  stx $0003 ;y += velY;

  ldx $0002
  ldy $0000
  add
  stx $0002 ;x += velX;

  ldy 0
  or
  jiz negate_vel_x  ;if x == 0 { negate_vel_x(); }
  
  ldy 90
  sub
  ldy 128
  add
  jic negate_vel_x  ;if x >= 90 { negate_vel_x(); }

continue:
  ldx $0003
  ldy 0
  or
  jiz negate_vel_y  ;if y == 0 { negate_vel_y(); }

  ldy 64
  sub
  ldy 128
  add
  jic negate_vel_y  ;if y >= 64 { negate_vel_y(); }

  jmp loop

negate_vel_x:
  ldx $0000
  not
  ldy 1
  add
  stx $0000
  jmp continue

negate_vel_y:
  ldx $0001
  not
  ldy 1
  stx $0001
  jmp loop
