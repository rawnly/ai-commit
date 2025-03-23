release:
	cargo build --release
	tar -czf ai-commit.tar.gz --directory=./target/release ai-commit
	SHASUM=$$(shasum -a 256 ai-commit.tar.gz | cut -d ' ' -f 1) ; \
	sd '\{\{shasum\}\}' "$$SHASUM" ai-commit.rb
