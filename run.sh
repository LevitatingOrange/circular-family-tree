cargo run baum.svg 9 -d A2 --font-size 4 --font-size-modifier 0.005 --x-offset 1 --y-offset 1 --y-offset-modifier 0.001 --line-width=0.1 --font-family="EBGaramond"
svg-convert -f pdf -o baum.pdf baum.svg
pdfposter baum.pdf baum_a3.pdf -m a3 -p 2x1a3