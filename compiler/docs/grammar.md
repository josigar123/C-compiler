# CFG

 A simple documentation of the current state of the C grammar currently supported.
 Uses extended BNF to express the grammar

## Expression Grammar

*For simplicity, when referencing expressions in e.g. the statement grammar
it will only be referred to as '&lt;expr&gt;'*

**&lt;decl-assign_expr&gt; ::= Ident Assign &lt;decl-assign_expr&gt;**  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**| &lt;or-expr&gt;**  
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
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**| Ident**  

## Statement Grammar

**&lt;stmnt&gt; ::= "return" &lt;expr&gt; ";"  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;| Type Ident [ "=" &lt;expr&gt; ] ";"**  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**| &lt;decl-assign_expr&gt; ";"**  

## AST Nodes

*Can think of each sub-section representing an enum and its variants where the contents reference the cfg above*  

### Expressions
  
**expr = BinOp(op, expr, expr)**  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**| UnOp(op, expr)**  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**| Int**  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**| Ident**  

### Statements

**stmnt = Return(expr)**  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;**| Assign(Type, Ident, Assign, expr)**  

### Functions

**func = ReturnValue Name Vec&lt;stmnt&gt;**  

### Programs

**prog = Vec&lt;func&gt;**  
