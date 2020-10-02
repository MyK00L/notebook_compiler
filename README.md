# notebook\_compiler

Compile code into latex for icpc notebooks

Reads from ./config.ron and ./layout.txt and outputs to out.tex

To then compile to pdf you can do
```sh
pdflatex --shell-escape out.tex
```

## Dependencies
- tex packages
    - geometry
    - inputenc
    - multicol
    - minted
    - fancyhdr
- [pygmentize](https://pygments.org/)

## Layout file format
- 0 tabs: section
- 1 tab:  subsection
- 2 tabs: source file

