#!/bin/bash

pct=10

amt=`convert -ping $1 -format "%[fx:$pct*min(w,h)/100]" info:`

img_width=`identify -format '%w' $1`
wmk_width=`expr $img_width / 27`
wmk_margin_w=`expr $img_width / $((1 + $RANDOM % 10))`
wmk_margin_h=`expr $img_width / 17`
dissolve=0

if [ $img_width -le 512 ];
then
  dissolve=70
fi

#convert $1 \( -background none watermark.svg -resize $wmk_width -write \
#  MPR:wm -channel A -evaluate multiply 0.1 +channel \) \
#  -gravity SouthEast -geometry +${wmk_margin}+${wmk_margin} -composite $2

convert -comment "Copyright Gardenzilla Ltd. Hungary." -background none watermark.svg -resize $wmk_width -write mpr:watermark +delete \
  $1 \
  mpr:watermark -gravity southeast -geometry +${wmk_margin_h}+${wmk_margin_h} -compose over -composite \
  $2
