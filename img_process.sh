#!/bin/bash

IMG=$1
OUTDIR=$(dirname $IMG)/../sized

allSizes=(1024 512 256 128 64)
allSizesNames=(xl l m s xs)

# Create output path if not exist
mkdir -p $OUTDIR

for i in ${!allSizes[@]}; do
  size=${allSizes[i]}
  filename=$(basename "$IMG")
  outfilepath=${OUTDIR}/${allSizesNames[i]}_${filename}
  
  # Convert image size
  convert $IMG -gravity center -resize ${size}x${size} \
    -extent ${size}x${size} $outfilepath
  
  # Add watermark to sized image
  img_add_watermark.sh $outfilepath $outfilepath
done


