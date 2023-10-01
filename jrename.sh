#! /bin/bash
function help
{
	echo "Uso: " 
	echo ""
	echo "Abre uma lista de nomes dos arquivos da pasta atual no vim para renomeamento em massa."
	echo ""
	echo "Salve com \`:wq\` em vim para renomear"
	echo "Arquivos são excluídos se suas respectivas linhas deixadas em branco."
	echo "Saia de vim com \`:q!\` para cancelar"
	echo ""
	echo "ATENÇÃO: NÃO REORDENE OS NOMES DE ARQUIVO!"
	echo "Para evitar problemas, o recurso \`dd\` foi remapeado para só limpar a linha."
	echo "Caso o número de linhas seja alterado, o programa aborta."
}
ORIG_NAMES='/tmp/orig_names'
TEMP_NAMES='/tmp/temp_names'
NEW_NAMES='/tmp/new_names'
COMMANDS='/tmp/rename_commands'
touch "$COMMANDS"
chmod 700 "$COMMANDS"	# só de segurança

if [ "$1" == "--help" ] || [ "$1" == '-h' ]; then	help && exit; fi

function limpeza
{
		rm -f "$COMMANDS" "$ORIG_NAMES" "$NEW_NAMES" "$TEMP_NAMES"
}

function renomeadora
{
		cp "$ORIG_NAMES" "$NEW_NAMES"
		vim +1 -c "nnoremap dd ^D" "$NEW_NAMES"
		# checar se não mudou no. de linhas
		if [ "$(wc -l < $ORIG_NAMES)" != "$(wc -l < "$NEW_NAMES")" ] 
			then
				echo "Número de linhas alterado, os arquivos podem se perder definitivamente!"
				echo "Saindo..."
				exit
		fi
		# checar se nada mudou
		if [ "$(< "$ORIG_NAMES")" == "$(< "$NEW_NAMES")" ] 
			then
				echo "Arquivos inalterados."
				limpeza
				echo "Saindo..."
				exit
		fi
		# colocar aspas nos nomes
		sed -i 's/^/"/' "$ORIG_NAMES"
		sed -i 's/$/"/' "$ORIG_NAMES"
		sed -i 's/^/"/' "$NEW_NAMES"
		sed -i 's/$/"/' "$NEW_NAMES"

		# faz um nome temporario 
		# intermediario para compatibilidade 
		# em sistemas FAT32 + aspas iniciais
		cp "$NEW_NAMES" "$TEMP_NAMES"
		sed -i 's/^"/"tmp\-/' $TEMP_NAMES
#		TEMP_NAMES=$("$NEW_NAMES" | sed 's/^"/^"temp/')

		# paste -> sed para incluir mv -> grep removendo mover o arquivo para si mesmo -> sed final q torna linhas vazias como remoção do arquivo
		{ paste -d' ' "$ORIG_NAMES" "$TEMP_NAMES" | sed 's/^/mv /' | grep -E -v 'mv (".*") \1' | sed 's/mv \("[^"]*"\) "\s*"*$/rm -rf \1/' > "$COMMANDS"
		bash -x "$COMMANDS"
		} 
		
		# move do nome temporario para o novo
		{ paste -d' ' "$TEMP_NAMES" "$NEW_NAMES" | sed 's/^/mv /' | grep -E -v 'mv (".*") \1' > "$COMMANDS"
		bash -x "$COMMANDS"
		}
	
		limpeza
}

if [ $# -eq 0 ]; then { # em todos arquivos do diretório
	\ls > "$ORIG_NAMES" 
} else { # arquivos indicados
	IFS=$'\n'
	for i in $@; do
			echo $i >> "$ORIG_NAMES"
	done
}; fi
renomeadora
