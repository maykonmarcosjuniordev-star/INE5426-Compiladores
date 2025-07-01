all:
	mkdir -p machines
	# TODO: Buscar versão python e usar a apropriada
	python3.10 scripts/generate_lexer.py
	cargo --quiet build --release
	clear
	@echo "Compilação concluída com sucesso. Binário salvo em './target/release/compiler'"
	@echo "Para compilador um arquivo contendo código fonte, use './target/release/compiler <caminho_para_o_arquivo>'"
	@echo "Ou, para executar o compilador de forma interativa, use o comando 'make run'"


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

