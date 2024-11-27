1) basic mnemonic to bytecode assembly
2) assembler directives
3) symbol tables and forward references

BASIC ASSEMBLY
    > Take in the tokens check if it is available in the table or not, if it is then get its corresopnding machinecode
        and assemble

ASSEMBLER DIRECTIVES: 
    > ORI addr => sets the start address of the program to be in the specified address
    > DV value => defines a variable

SYMBOL TABLE AND FORWARD REFERENCES
    > A symbol table is a dictionay or a hash table that is used to store the key(lables) and their corresponding addresses
    > A forward reference will be stored in a temporary list and will be added to the symbol table if the its definition address is obtained

<!-- INSTRUCTION: -->

<!-- ------------------------------------------------------------------ -->
<!-- [$<label>/@<Assembler_directives>:|<opcode>|<operand>|;<comments>] -->
<!-- ------------------------------------------------------------------ -->

Tokenizing:
    >STAGE 1: In this stage the comments are removed from the input assembly code lines 
        <check for ";" symbol in the line and if found then remove everything following it>
    >STAGE 2: In this stage the label definitions are separated from the assembly code lines and added to the hash table with 



