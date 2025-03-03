> [!IMPORTANT]
> I dont recommend use .cif in this moment,
> because now cifir it's slow and highweight

<div align="center">
  <picture>
        <img alt="The Cifir Image Format Logo"
         src="cifirworks/cifir.png"
         width="25%">
  </picture>

<h1>Cifir</h1>

| [Releases](https://github.com/SynthouS/cifir/releases) | [Issues](https://github.com/SynthouS/cifir/issues) | 
</div>

# About & Usage
The **CIF (Cifir Image Format)** is a simple, text-based image format designed for ease of use and readability. It supports RGB colors and stores images in a human-readable format. Below is a detailed explanation of how the format works.
A CIF file consists of the following components:

1. **Header**: The first line specifies the resolution of the image in the format `<widthxheight>`. For example, `<3x3>` represents an image with a width of 3 pixels and a height of 3 pixels.

2. **Pixel Data**: Each subsequent line represents a row of pixels in the image. Each pixel is defined by a hexadecimal RGB color code prefixed with a `#`. For example, `#ff0000` represents red, `#00ff00` represents green, and `#0000ff` represents blue.

3. **Comments**: Comments can be added anywhere in the file using the format `<-- comment -->`. These are ignored during parsing.

# Cifir Viewer
Just open ```.cif``` use cifir app. or in windows u can type in cmd
```cifir.exe filename.cif``` and in linux ```./cifir filename.cif```
Also use ```lmb``` and ```mwheel``` for navigation

# Cifirworks.py

it's a converter from png to cif and from cif to png.
Usage ```python cifirworks.py input.png output.cif``` ```python cifirworks.py input.cif output.png```

