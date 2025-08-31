#### VARIABLES TO SET ####

NAME := homeserver
SERVER := localhost

# possible values: amd64, arm, arm64, darwin-arm64
ARCH := amd64

##########################

CONFIG := $(NAME).yaml
SYNAPSE := python -m synapse.app.homeserver

define SETTINGS

suppress_key_server_warning: true
app_service_config_files:
  - mautrix-whatsapp-registration.yaml
media:
  enabled: true

endef
export SETTINGS

.DEFAULT_GOAL := start
.PHONY: requirements install clean start createuser whatsapp install-synapse
.SECONDARY:

requirements:
	pip install --upgrade pip
	pip install --upgrade setuptools
	pip install matrix-synapse
	pip list --format=freeze >requirements.txt

install-synapse:
	pip install -r requirements.txt
	pip list --format=freeze >requirements.txt
	$(SYNAPSE) --server-name $(SERVER) --config-path $(CONFIG) --generate-config --report-stats=no
	@echo "$$SETTINGS" >> $(CONFIG)

install: mautrix-whatsapp-registration.yaml install-synapse

start:
	$(SYNAPSE) --config-path $(CONFIG)

createuser:
	register_new_matrix_user -c $(CONFIG)

clean:
	rm -rf venv $(NAME).* *.log.config *.signing.key *.yaml logs mautrix-*
	sudo rm -rf media_store

### MAUTRIXES ###

WA_VERSION := 0.12.4

mautrix-whatsapp-$(ARCH):
	curl -LO https://github.com/mautrix/whatsapp/releases/download/v$(WA_VERSION)/$@
	chmod +x $@

mautrix-%-config.yaml.default: mautrix-%-$(ARCH)
	./$< -e -c $@

mautrix-%-config.yaml: mautrix-%-config.yaml.default
	sed 's/type: postgres/type: sqlite3-fk-wal/;s/uri: postgres:\/\/user:password@host\/database?sslmode=disable/uri: mautrix-whatsapp.sqlite3/;s/address: http:\/\/example./address: http:\/\//;s/example.com/$(SERVER)/' $< > $@

mautrix-%-registration.yaml: mautrix-%-config.yaml
	[ -f $@ ] || ./mautrix-$*-$(ARCH) -g -c $< -r $@

mautrix-%-run: mautrix-%-config.yaml mautrix-%-registration.yaml
	./mautrix-$*-$(ARCH) -c $< -r $(word 2,$^)

