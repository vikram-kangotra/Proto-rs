DIGIT: [0-9];
INT: DIGIT+;
FLOAT: DIGIT+ '.' DIGIT+ EXPONENT?;
EXPONENT: [eE][+-]? DIGIT+;

expr: term;
term: factor (('+' | '-') factor)*;
factor: unary (('*' | '/') unary)*;
unary: ('+'|'-') unary | primary;
primary: INT | '(' expr ')';
