Alunos: 
Rayan de Andrade e Andrade RA: 24211898-2
Salomão Antonio Braga RA: 24047645-2
Vinicius Colombo RA: 24160033-2

---

Gramática escolhida: Java
Fonte: https://docs.oracle.com/en/java/javase/26/docs/specs/jls/jls-2.html
Notação: Java é uma linguagem CFG, porém é adaptada com convenções EBNF para facilitar leitura humana.

Regras de Gramática:
For: 
BasicForStatement:
for ( [ForInit] ; [Expression] ; [ForUpdate] ) Statement



Código a ser derivado:
```
public class Principal {
    public static void main(String[] args) {
        for (int i = 0; i <= 10; i++) {
            System.out.println(i);
        }
    }
}
```

Regras para serem utilizadas:

``` 
<programa> ::= "public class" <identificador> "{" <metodo> "}"

<metodo> ::= "public static void main(String[] args)" "{" <comando> "}"

<comando> ::= <for>

<for> ::= "for" "(" <inicializacao> ";" <condicao> ";" <incremento> ")" "{"
          <impressao> "}"

<inicializacao> ::= "int" <identificador> "=" <numero>

<condicao> ::= <identificador> "<=" <numero>

<incremento> ::= <identificador> "++"

<impressao> ::= "System.out.println" "(" <identificador> ")" ";"

<identificador> ::= "i" | "Principal"

<numero> ::= "0" | "10"
```

Derivação:

Programa

1.

```Programa ⇒ Modificador Classe```

2.

```⇒ public Classe```

3.

```⇒ public class Identificador { Método }```

4.

```⇒ public class Principal { Método }```

5.

```⇒ public class Principal { public static void main ( String [ ] args ) { Comando } }```

6. 

```⇒ public class Principal { public static void main ( String [ ] args ) { for ( Inicialização ; Condição ; Incremento ) { impressao } } }```

7. 

```⇒ public class Principal { public static void main ( String [ ] args ) { for ( int i = 0 ; Condição ; Incremento ) { impressao } } }```

8. 

```⇒ public class Principal { public static void main ( String [ ] args ) { for ( int i = 0 ; i <= 10 ; Incremento ) { impressao } } }```

9. 

```⇒ public class Principal { public static void main ( String [ ] args ) { for ( int i = 0 ; i <= 10 ; i ++ ) { impressao } } }```

10. 

```⇒ public class Principal { public static void main ( String [ ] args ) { for ( int i = 0 ; i <= 10 ; i ++ ) { System.out.println ( i ) ; } } }```
