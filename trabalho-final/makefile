all:
	mkdir -p inputs
	mkdir -p machines
	mkdir -p output
	mkdir -p output/expression_trees
	python3 scripts/generate_lexer.py
	cargo --quiet build --release
	clear
	@echo "Compilação concluída com sucesso!"
	@echo "Para executar o analisador léxico, use o comando 'make run'"
	@echo "Ou, para rodar diretamente, use './target/release/compiler <caminho_para_o_arquivo>'"


run:
	@clear
	@INPUT="start"; \
	while [ "$$INPUT" != "" ]; do \
		echo "===================================="; \
		ls inputs/; \
		echo "===================================="; \
		read -p "Digite apenas o nome (sem ".txt") do arquivo de input (ou ENTER para encerrar): " INPUT; \
		clear; \
		if [ "$$INPUT" != "" ]; then \
			echo "Executando o analisador léxico no arquivo: inputs/$$INPUT.txt"; \
			time ./target/release/compiler "inputs/$$INPUT.txt"; \
		fi; \
	done

