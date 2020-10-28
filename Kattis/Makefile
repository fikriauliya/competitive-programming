run:
	RUST_BACKTRACE=1 cargo run -p $(project) --bin $(project) < $(project)/$(testcase).in >$(project)/$(testcase).out
	diff -y --left-column $(project)/$(testcase).ans $(project)/$(testcase).out
generate:
	project_name=$(project) cargo run -p tools --bin generator
commit:
	git add Cargo.toml
	git add $(project)
	git c -m "Add $(difficulty) $(project)"
	git push
