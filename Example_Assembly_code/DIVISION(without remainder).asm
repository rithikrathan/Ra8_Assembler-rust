; @ORI 12
; @DW b
LDI 0x02
MOV B A 
LDI 0x0B 
MVI D 0x00
$LOOP: SUB B ; defining a label i suck at commenting 
INC D 
JNZ $LOOP
MOV A D 
HLT

