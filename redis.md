# Redis

## comandos
* Conectar: `redis-cli`


### Chave

 estrutura de dados mais comum da ferramenta

Adicionar chave: `set {chave} {valor}` exemplo: <br>
    
    $ set foo bar    
 
pegar chave: `get {chave}` exemplo: <br>
    
    $ get foo
      "bar"
    

listar chaves: `keys {chave} {valor}` exemplo: <br>
    
    $ keys foo
      1) "foo"


### Listas
 listas são estruturas altamente manipulaveis no redis, podendo escolher a 
 direção de inserção de itens, seja no começo ou final.

 adicionar item na HEAD (inicio da lista): `LPUSH {lista} {valor}` <br>
 adicionar item na TAIL (final da lista): `LPUSH {lista} {valor}`

    $ LPUSH foo a  # "a"
    $ LPUSH foo b  # "b","a"
    $ RPUSH foo c  # "b","a","c" 

 listar itens: `LRANGE {lista} {inicio} {fim}`

    $ LRANGE mylist 0 2
      1) "b"
      2) "a"
    
    $ LRANGE mylist 0 -1
      1) "b"
      2) "a"
      3) "c"


### Sets
 de começo, podem parecer com as listas, porém, sets não preservam a ordem 
 de inserção e não permitem duplicatas. Por contrapartida, possuem menor complexidade de busca/inserção/remoção O(1).

### Hashes
 sua estrutura se assemelha com um `json`
 inserção: `HSET {Hash} ...{key}:{value}`
 exmplo de Hash: `user:1000 nome:fulano, idade:40`