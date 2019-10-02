# steganography
Basic stenography library written in rust

## Alpha
Embeds message in alpha channel of pixels. Decoding goes until alpha = 255 or end.

## RGBA
Embeds message in RGBA channels of pixels. Decoding goes until three values of 0 or end and strips trailing 0's.
