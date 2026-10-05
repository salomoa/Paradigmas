1.
```python
def adicionar(item, lista=[]):
    lista.append(item)
    return lista
    
print(adicionar(1))
    
print(adicionar(2))
```

R:
Saída: 
[1]
[1,2]

O conceito apresentado é o de Closure

2.
```java
static void zera(int[] v, int n) {
  v[0] = 0;

  n = 0;
}
```

R:
Saída: 0 5

O conceito apresentado é o de Passagem de parâmetros por valor/referência.

3.
```python
fs = [lambda: i for i in range(3)]

print([f() for f in fs])
```

R: 
Saída: [2,2,2]
O conceito apresentado é o de chamada indireta

4. 
```C
int contador(void) {

static int n = 0;

return ++n;

}

// em main:

contador(); contador();

printf("%d\n", contador());
```

R:
Saída: 3

O conceito é de referenciamento local

5.
```rust
fn dobra(v: &[i32]) -> Vec<i32> {
    v.iter().map(|x| x * 2).collect()
}

fn main() {
    let v = vec![1, 2, 3];
    let d = dobra(&v); 
    println!("{:?} {:?}", v, d); 
}
```
R:
Saída: [1,2,3] [2,4,6]

O conceito é o de referência 

6.
```python
total = 0

def adiciona(x):
    global total
    total = total + x
    return total

print(adiciona(5))
```

R: 
Saída: 5

O conceito é o da regra de atribuição de local
