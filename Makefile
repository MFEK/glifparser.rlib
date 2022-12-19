SHELL := /bin/bash

.PHONY:
clean:
	find . -mindepth 1 -maxdepth 1 -not -name 'target' -and -not -name '.git' -and -not -name '.gitignore' -and -not -name 'Makefile' | xargs rm -r

.PHONY:
move-target-doc:
	find target/doc -mindepth 1 -maxdepth 1 -not -name '.lock' | parallel --bar mv {} .
	find . -maxdepth 1 -mindepth 1 -not -name 'Makefile' -and -not -name '.git' -and -not -name '.lock' -and -not -name 'target' | xargs git add
