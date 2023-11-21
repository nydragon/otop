##
## EPITECH PROJECT, 2023
## otop
## File description:
## Makefile
##

OUTPUT_DIR = ./build

FRONT_FOLDER = ./packages/frontend
BACK_FOLDER = ./packages/backend

BACK_BIN_NAME = otop
NODE_PM = pnpm

init:
	@echo "Please use 'make build' to build the project"
	sudo bash scripts/setup.sh

build: clean front back
# 1. Check if NODE_PM is available
front:
	mkdir -p $(OUTPUT_DIR)/www;
	$(NODE_PM) install --prefix $(FRONT_FOLDER);
	$(NODE_PM) run --prefix $(FRONT_FOLDER) build;
	cp -r $(FRONT_FOLDER)/dist/* $(OUTPUT_DIR)/www;
	rm -rf $(FRONT_FOLDER)/dist

back:
	cd $(BACK_FOLDER) && cargo build;

clean:
	rm -rf $(OUTPUT_DIR);