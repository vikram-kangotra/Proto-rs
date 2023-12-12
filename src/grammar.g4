DIGIT: [0-9];
INT: DIGIT+;
FLOAT: DIGIT+ '.' DIGIT+ EXPONENT?;
EXPONENT: [eE][+-]? DIGIT+;

IDENT: [a-zA-Z_]+ [a-zA-Z_0-9]* | 'true' | 'false';

expr: comparison;
equality: comparison (('==' | '!=') comparison)*;
comparison: term (('>' | '>=' | '<' | '<=') term)*;
term: factor ([+-] factor)*;
factor: unary ([+/] unary)*;
unary: [+-] unary | primary;
primary: INT | FLOAT | IDENT | '(' expr ')';
