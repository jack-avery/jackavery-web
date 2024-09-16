all: back deploy

back:
	@ansible-playbook playbooks/back.yml

deploy:
	@ansible-playbook playbooks/deploy.yml

dev:
	cd roles/back/files && cargo run
