# rust-shapesys
rust port of shape-sys.cpp

###grammar explanation
```
def: RULE := LHS '>' RHS
     LHS  := [A-Z][:alpha:]
     RHS  := "" | [:alpha:] | "." | RHS ',' RHS
ex: AbCdEf>ACE,bdf
     - AbCdEf>_ instructs the parser to match an ACE shaped plygon,
       introducing b,d,f points between it's vertices
     - _>aBc instucts the parser to form a new aBc polygon with using the
       vertices introduced in LHS
     - Old vertices must be uppercase, new ones lowercase.
     - The LHS definition wraps arownd, therfore in "ABCd", d is considered
       between A and C (*)
     - '.' introduces the center of the polygon
def: RULES := RULE | RULE ";" RULES
     - rules LHS must match unique polygons
       ( ex: "ABC>", "AdBC>" is not allowed )
```

###todo
possibly generate/load a geometry shader

###example

![alt text](https://github.com/ltlollo/rust-shapesys/raw/master/AaBnndCc>acd,Aad,aBc,dcC;ABCD>AB.,BC.,CD.,DA..png "preview")

to generate some pictures
```
shufw CabBcdAef | appline _ "echo _; ./target/release/shapesys \"_>dCe,bBc,fAa,bc.,de.,fa.;ABCDEF>AB.,BC.,CD.,DE.,EF.,FA.\" 6 \"231,800;693,800;924,400;693,0;231,0;0,400\""

```
