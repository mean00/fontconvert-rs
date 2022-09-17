# flatconvert-rs : Tweakable embedded font conversion
This is the rust version of https://github.com/mean00/fontconvert.git, which is a fork of https://github.com/charles-haynes/fontconvert, which is a fork of adafruit font convert tool.

This is a truetype-to-adafruit font file tool. 

This adds compression, chars selection and 1/2/4 bit per pixel support (for smoother fonts).
```
USAGE:
    flatconvert-rs [OPTIONS] --font <FONT> --output-file <OUTPUT_FILE> --size <SIZE>

OPTIONS:
    -b, --begin <BEGIN>                Ascii value of the first char to render [default: 32]
    -c, --compression                  compression
    -e, --end <END>                    Ascii value of the last char to render [default: 127]
    -f, --font <FONT>                  Path to the font to use
    -h, --help                         Print help information
    -k, --pick <PICK>                  String to pick individual chars i.e. -k "abcd" will only
                                       render a,b,c,d [default: ]
    -m, --bitmap-file <BITMAP_FILE>    bitmap file [default: ]
    -o, --output-file <OUTPUT_FILE>    output file (C header)
    -p, --bpp <BPP>                    bpp  (1= B&W, 2=4 levels of grey, 4 = 16 levels of grey)
                                       [default: 1]
    -s, --size <SIZE>                  Size of the font to render
    -V, --version                      Print version information
```

NB: -k and -b/-e should not be used together




