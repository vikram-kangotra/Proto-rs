DIGIT: [0-9];
INT: DIGIT+;
FLOAT: DIGIT+ '.' DIGIT+ EXPONENT?;
EXPONENT: [eE][+-]? DIGIT+;

TRUE: 'true';
FALSE: 'false';
IDENT: [a-zA-Z_]+ [a-zA-Z_0-9]*;
LET: 'let';
IF: 'if';
ELSE: 'else';
WHILE: 'while';
BREAK: 'break';

COMMENT: '//' .* '\n' | '/*' .* '*/'

stmt: initialize | exprStmt | block | if | while;
exprStmt: expr ';' ;
breakStmt: BREAK ';' ;
initialize: LET IDENT '=' exprStmt;
block: '{' stmt* '}';
if: IF expr stmt (ELSE if)* (ELSE stmt)?;
while: WHILE expr stmt;

expr: equality;
assignment: IDENT ('=' expr)?;

equality: comparison (('==' | '!=') comparison)*;
comparison: term (('>' | '>=' | '<' | '<=') term)*;
term: factor ([+-] factor)*;
factor: unary ([*/%] unary)*;
unary: [+-] unary | primary;
primary: INT | FLOAT | TRUE | FALSE | '(' expr ')' | assignment;
