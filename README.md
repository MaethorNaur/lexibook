Grammar
====

```
start:
  header (NL)
comment: COMMENT? ;
WS : [ \t]+ -> skip ;
NL : ('\r'? '\n')+ ;
COMMENT : '%' (~[\n])* ;
```
