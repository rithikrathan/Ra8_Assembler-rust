$START: LDI 0x04

MOV B A
NOPE
$LOOP: DCR B ;loop that does n*(n-1)
JZ $END ;this is a forward reference
MOV C A
MOV D B
XOR C
$LOOP1: ADD C ;loop that multiplies two numbers
DCR D 

JNZ $LOOP1
JZ $LOOP
$END: STA 0x00 ;End if completed
HLT
