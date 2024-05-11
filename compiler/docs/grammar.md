# CFG

 A simple documentation of the current state of the C grammar currently supported.
 Uses extended BNF to express the grammar

## Expression Grammar

*For simplicity, when referencing expressions in e.g. the statement grammar
it will only be referred to as '&lt;expr&gt;'*

**&lt;or-expr&gt; ::= &lt;and-expr&gt; { || &lt;and-expr&gt; }**  
**&lt;and-expr&gt; ::= &lt;eq-expr&gt; { && &lt;eq-expr&gt; }**  
**&lt;eq-expr&gt; ::= &lt;rel-expr&gt; { != or == &lt;rel-expr&gt; }**  
**&lt;eq-expr&gt; ::= &lt;rel-expr&gt; { ( != or == ) &lt;rel-expr&gt; }**  
**&lt;rel-expr&gt; ::= &lt;add-expr&gt; { ( < or > or <= or >= ) &lt;add-expr&gt; }**  
**&lt;add-expr&gt; ::= &lt;term-expr&gt; { ( + or - ) &lt;term-expr&gt; }**  
**&lt;term-expr&gt; ::= &lt;factor-expr&gt; { != or == &lt;factor-expr&gt; }**  
**&lt;factor&gt; ::= "(" &lt;or-exp&gt; ")"**  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**| &lt;int&gt;**  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**| &lt;unary-expr&gt; &lt;factor&gt;**  

## Statement Grammar

**&lt;return&gt; ::= "return" &lt;expr&gt; Semi**  
**&lt;declaration&gt; ::= Type Ident [ Assign &lt;expr&gt; ] Semi**  

## AST Nodes

### Expressions

**exp = BinOp(operator, exp, ext)**  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**| UnOp(op, exp)**  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**| Int**  

### Statements

**stmnt = Return(exp)**  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**| Assign(Type, Ident, Assign, exp)**  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**| Decalre(Type, Ident)**  

### Functions

**func = ReturnValue Name Vec&lt;stmnt&gt;**  

### Programs

**prog = Vec&lt;func&gt;**  
