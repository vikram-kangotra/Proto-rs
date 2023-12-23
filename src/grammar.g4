DIGIT: [0-9];
INT: DIGIT+;
FLOAT: DIGIT+ '.' DIGIT+ EXPONENT?;
EXPONENT: [eE][+-]? DIGIT+;

TRUE: 'true';
FALSE: 'false';
IDENT: [a-zA-Z_]+ [a-zA-Z_0-9]*;

COMMENT: '//' .* '\n' | '/*' .* '*/'

stmt: initialize | exprStmt | block | if;
exprStmt: expr ';' ;
initialize: 'let' IDENT '=' exprStmt;
block: '{' stmt* '}';
if: 'if' expr stmt ('else' if)* ('else' stmt)?;

expr: equality;
assignment: IDENT ('=' expr)?;

equality: comparison (('==' | '!=') comparison)*;
comparison: term (('>' | '>=' | '<' | '<=') term)*;
term: factor ([+-] factor)*;
factor: unary ([*/%] unary)*;
unary: [+-] unary | primary;
primary: INT | FLOAT | TRUE | FALSE | '(' expr ')' | assignment;
