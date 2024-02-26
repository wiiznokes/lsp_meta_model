set windows-powershell := true

# call before pull request
pull: fmt prettier fix test


test:
	cargo test --workspace --all-features

fix:
	cargo clippy --workspace --all-features --fix --allow-dirty --allow-staged

fmt:
	cargo fmt --all

prettier:
	# install on Debian: sudo snap install node --classic
	# npx is the command to run npm package, node is the runtime
	npx prettier -w .


git-cache:
	git rm -rf --cached .
	git add .