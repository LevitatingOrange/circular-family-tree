target/release/circular-binary-tree baum_a2.svg 9 -d A2 --font-size 4 --font-size-modifier 0.005 --x-offset 1 --y-offset 1 --y-offset-modifier 0.001 --line-width=0.1 --font-family="EBGaramond" --margin=5
target/release/circular-binary-tree baum_a3.svg 9 -d A1 --font-size 8 --font-size-modifier 0.01 --x-offset 1 --y-offset 1 --y-offset-modifier 0.001 --line-width=0.1 --font-family="EBGaramond" --margin=10
rsvg-convert -f pdf -o baum_a2.pdf baum_a2.svg
rsvg-convert -f pdf -o baum_a3.pdf baum_a3.svg