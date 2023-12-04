EXPR: TERM;
TERM: FACTOR (('+' | '-') FACTOR)*;
FACTOR: UNARY (('*' | '/') UNARY)*;
UNARY: ('+'|'-') UNARY | PRIMARY;
PRIMARY: INT | '(' EXPR ')';
