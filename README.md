# JNAME 1.0.1  - rename for the better

Renomeador estupidamente rápido simples para Linux.

## Uso
``` 
jname -args
``` 

args:
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

$ jname -lsde 'arquivo----çom(nome)EStuPiDo!.pdf'

$ ls 
arquivo-com-nome-estupido.pdf
```

## Racional
Pessoas nomeiam arquivos _estupidamente_. 

Ajusta arquivos em batch.

Extensível e modular.

## Versões

1.0.0 - Funcional.
1.0.1 - trim_end() da função finaliza
Major.Minor.Patch

## Licença
> Copyright (c) 2023 - Jefferson T. 
> This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more details. You should have received a copy of the GNU General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>
