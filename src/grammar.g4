DIGIT: [0-9];
INT: DIGIT+;
FLOAT: DIGIT+ '.' DIGIT+ EXPONENT?;
EXPONENT: [eE][+-]? DIGIT+;

TRUE: 'true';
FALSE: 'false';
IDENT: [a-zA-Z_]+ [a-zA-Z_0-9]*;

COMMENT: '//' .* '\n' | '/*' .* '*/'

stmt: assignment | exprStmt | block | if;
exprStmt: expr ';' ;
assignment: 'let' IDENT '=' exprStmt;
block: '{' stmt* '}';
if: 'if' expr stmt ('else' if)* ('else' stmt)?;

expr: equality;
equality: comparison (('==' | '!=') comparison)*;
comparison: term (('>' | '>=' | '<' | '<=') term)*;
term: factor ([+-] factor)*;
factor: unary ([*/%] unary)*;
unary: [+-] unary | primary;
primary: INT | FLOAT | IDENT | TRUE | FALSE | '(' expr ')';
