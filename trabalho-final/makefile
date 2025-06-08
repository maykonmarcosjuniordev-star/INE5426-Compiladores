all:
	mkdir -p inputs
	mkdir -p machines
	if [[ ! -f machines/lexer.automata ]]; then \
		python3 scripts/generate_lexer.py; \
	fi
	cargo --quiet build --release
	clear
	@echo "Compilação concluída com sucesso!"
	@echo "Para executar o analisador léxico, use o comando 'make run'"


run:
	@echo "opções:"
	@echo ""
	@ls inputs/
	@echo ""
	@INPUT="start"; \
	while [ "$$INPUT" != "" ]; do \
		read -p "Digite o nome do arquivo de input (ou ENTER para encerrar): " INPUT; \
		if [ "$$INPUT" != "" ]; then \
			echo "Executando o analisador léxico no arquivo: inputs/$$INPUT.txt"; \
			./target/release/compiler "inputs/$$INPUT.txt"; \
		fi; \
	done

