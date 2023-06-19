# inventory in inventory.yml
# use WSL; ansible requires it

all: front back media deploy

# shortcuts
f: front deploy
b: back deploy

# build frontend docker image and upload to dockerhub
front:
	@ansible-playbook playbooks/front.yml

# build backend image on host
back:
	@ansible-playbook playbooks/back.yml

# update media folder on server with current media
media:
	@ansible-playbook playbooks/media.yml

# deploy front and back image to host
deploy:
	@ansible-playbook playbooks/deploy.yml

## development stuff

# run development rocket instance
back-dev:
	cd roles/back/files && cargo run
