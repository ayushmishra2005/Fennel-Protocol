 #!/bin/bash
sudo apt-get update
sudo apt-get install -y docker.io
gcloud auth print-access-token | docker login -u oauth2accesstoken --password-stdin us-east1-docker.pkg.dev
docker run -dit -p 6060:6060 --name subservice us-east1-docker.pkg.dev/whiteflag-0/fennel-docker-registry/polkadot:latest
docker exec -it polkadot /app/target/release/polkadot --alice --validator --base-path /tmp/relay-alice --chain /app/chainspec.json --port 30333 --ws-port 9944 
