DIGIT: [0-9];
INT: DIGIT+;
FLOAT: DIGIT+ '.' DIGIT+ EXPONENT?;
EXPONENT: [eE][+-]? DIGIT+;

TRUE: 'true';
FALSE: 'false';
IDENT: [a-zA-Z_]+ [a-zA-Z_0-9]*;

stmt: assignment | exprStmt;
exprStmt: expr ';' ;
assignment: 'let' IDENT '=' exprStmt;

expr: equality;
equality: comparison (('==' | '!=') comparison)*;
comparison: term (('>' | '>=' | '<' | '<=') term)*;
term: factor ([+-] factor)*;
factor: unary ([*/%] unary)*;
unary: [+-] unary | primary;
primary: INT | FLOAT | IDENT | TRUE | FALSE | '(' expr ')';
