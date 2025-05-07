all: back deploy

back:
	@ansible-playbook playbooks/back.yml

deploy:
	@ansible-playbook playbooks/deploy.yml

dev:
	cd roles/deploy/files/hugo && hugo server --disableFastRender
