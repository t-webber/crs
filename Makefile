#### VARIABLES TO SET ####

NAME := homeserver
SERVER := localhost

##########################

CONFIG := $(NAME).yaml
SYNAPSE := python -m synapse.app.homeserver

ifndef VIRTUAL_ENV
$(error Not in virtualenv, please create one and source the venv/bin/activate)
endif

.DEFAULT_GOAL := start
.PHONY: requirements install clean start createuser

requirements:
	pip install --upgrade pip
	pip install --upgrade setuptools
	pip install matrix-synapse
	pip list --format=freeze >requirements.txt

install:
	pip install -r requirements.txt
	pip list --format=freeze >requirements.txt
	$(SYNAPSE) --server-name $(SERVER) --config-path $(CONFIG) --generate-config --report-stats=no
	echo -e "\nsuppress_key_server_warning: true" >> $(CONFIG)
	synapse_port_db --sqlite-database $(NAME).db --postgres-config $(CONFIG)

start: install
	$(SYNAPSE) --config-path $(CONFIG)

createuser:
	register_new_matrix_user -c $(CONFIG)

clean:
	rm -rf venv $(NAME).* *.log.config *.signing.key media_store
