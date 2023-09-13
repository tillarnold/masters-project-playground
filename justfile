_default:
  @just --list --unsorted

# initialize the git submodule
init:
	git submodule init
	git submodule update


# Build all LaTeX documents in the openDP repo
build_latex_proofs:
	#!/bin/bash
	cd opendp
	for TEXFILE in $(fd '\.tex$')
	do
		TEXDIR=$(dirname "$TEXFILE")
		FILENAME=$(basename "$TEXFILE")
		echo "Processing $FILENAME in $TEXDIR"
		cd "$TEXDIR"
		pdflatex -interaction=nonstopmode "$FILENAME"
		cd -
	done


list_proof_pdfs:
	fd '\.pdf$'
