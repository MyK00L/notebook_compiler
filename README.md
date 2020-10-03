# notebook\_compiler

Compile code into latex for icpc notebooks

Reads from ```./config.ron``` and ```./layout.txt``` and outputs to ```./out.tex```

To then compile to pdf you can do
```sh
pdflatex --shell-escape out.tex
```

## Dependencies
- tex packages (direct dependencies only)
	- amsart
	- geometry
	- fancyhdr
	- inputenc
	- multicol
	- minted
	- datetyme
	- bera
	- fontenc
- [pygmentize](https://pygments.org/)

## Layout file format
- 0 tabs: section
- 1 tab:  subsection
- 2 tabs: source file

## Notes
If the you get a pdf without a toc, just recompile it

