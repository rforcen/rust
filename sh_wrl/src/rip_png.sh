# rip wrl's to png's

for w in ../wrl/*wrl
do
    png="${w%.wrl}.png"
    echo $w "->" $png
    
    view3dscene $w --screenshot 0 $png
done