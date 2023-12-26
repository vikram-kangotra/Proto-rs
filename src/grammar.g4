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
CONTINUE: 'continue';
FUNCTION: 'fn';

DATA_TYPE: 'i8' | 'i16' | 'i32' | 'i64';
RETURN_TYPE: DATA_TYPE | '()';

COMMENT: '//' .* '\n' | '/*' .* '*/'

stmt: initialize | exprStmt | block | if | while | function_dec | function_def;
exprStmt: expr ';' ;
breakStmt: BREAK ';' ;
continueStmt: CONTINUE ';' ;
initialize: LET IDENT (':' DATA_TYPE)? '=' exprStmt;
block: '{' stmt* '}';
if: IF expr stmt (ELSE if)* (ELSE stmt)?;
while: WHILE expr stmt;
function_dec: FUNCTION IDENT '(' (IDENT ':' DATA_TYPE)* ')' ( '->' RETURN_TYPE )? ';' ;
function_def: FUNCTION IDENT '(' (IDENT ':' DATA_TYPE)* ')' ( '->' RETURN_TYPE )? block;

expr: equality | function_call;
assignment: IDENT ('=' expr)?;
function_call: IDENT '(' expr* ')';

equality: comparison (('==' | '!=') comparison)*;
comparison: term (('>' | '>=' | '<' | '<=') term)*;
term: factor ([+-] factor)*;
factor: unary ([*/%] unary)*;
unary: [+-] unary | primary;
primary: INT | FLOAT | TRUE | FALSE | '(' expr ')' | assignment;
