# JNAME 0.1.1  - rename for the better

Renomeador estupidamente rápido simples para Linux.

## Instalação
Um binário de ajuste chamado `normalizador` é criado e pode ser usado em manipulação de quaisquer `strings`.

```
$ git clone 'https://github.com/EstalineCCCP/jname'

$ cd jname

$ cargo build --release

$ mv -t /usr/local/sbin/ ./target/release/normalizador jname.sh jrename.sh
```

## Uso
Chame quantos arquivos quiser para serem renomeados em `vim`:
```
jrename.sh "arquivo1+TODO_erRado.pdf" "n-esimo-arquivo---mal.nomeado.ext"
```
Após editar os nomes, salve com `:wq`.

Para detalhes:
```
jrename..sh --help
```

Ainda, diretamente na linha de comando, faça ajustes rápidos e em **batch**:
``` 
jname.sh -args "arquivo1.ext" "arquivo2.ext"
``` 

Argumentos possíveis:
```
l - lowercase
d - remove diacríticos
e - remove caracters estranhos à maioria dos terminais
s - remove espaços
p - transforma separações de pontos por hífen
r - remove caracters repetidos, exceto letras
```

```
$ ls
arquivo----çom(nome) EStuPiDo!.pdf

$ jname.sh -lsde 'arquivo----çom(nome)EStuPiDo!.pdf'

$ ls 
arquivo-com-nome-estupido.pdf
```

## Racional
Arquivos são nomeados de forma _estúpida_. 

Ajusta arquivos em batch.

Extensível e modular.

## Licença
> Copyright (c) 2023 - Jefferson T. 
> This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>
