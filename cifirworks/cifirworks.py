import sys
import re
from PIL import Image
from collections import defaultdict

def png_to_cif(png_path, cif_path):
    try:
        img = Image.open(png_path).convert("RGBA")
        width, height = img.size
        pixels = img.load()
        
        color_map = {}
        next_color_id = 0
        optimized_pixels = []
        
        for y in range(height):
            row = []
            for x in range(width):
                r, g, b, a = pixels[x, y]
                if a < 255:
                    r, g, b = 0, 0, 0
                color_key = f"{r:02x}{g:02x}{b:02x}"
                
                if color_key not in color_map:
                    color_map[color_key] = next_color_id
                    next_color_id += 1
                
                row.append(color_map[color_key])
            optimized_pixels.append(row)
        
        with open(cif_path, 'w') as f:
            f.write(f"<{width}x{height}>\n")
            
            color_table = {v: k for k, v in color_map.items()}
            for y in range(height):
                line = []
                for x in range(width):
                    color_id = optimized_pixels[y][x]
                    line.append(f"#{color_table[color_id]}")
                f.write(' '.join(line) + '\n')
                
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

def cif_to_png(cif_path, png_path):
    try:
        with open(cif_path, 'r') as f:
            lines = f.readlines()
        
        resolution = re.match(r'<(\d+)x(\d+)>', lines[0].strip())
        width, height = int(resolution[1]), int(resolution[2])
        pixels = []
        
        color_cache = {}
        for line in lines[1:]:
            cleaned = re.sub(r'<--.*?-->', '', line).replace(' ', '')
            colors = [c for c in cleaned.split('#') if c]
            for color in colors:
                color = color.strip().ljust(6, '0')[:6]
                
                if color not in color_cache:
                    r = int(color[0:2], 16)
                    g = int(color[2:4], 16)
                    b = int(color[4:6], 16)
                    color_cache[color] = (r, g, b)
                
                pixels.append(color_cache[color])
        
        img = Image.new('RGB', (width, height))
        img.putdata(pixels)
        
        img.save(png_path, optimize=True, quality=95)
        
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: cifirworks.py <input> <output>")
        sys.exit(1)
    
    input_file, output_file = sys.argv[1], sys.argv[2]
    
    if input_file.endswith('.png') and output_file.endswith('.cif'):
        png_to_cif(input_file, output_file)
    elif input_file.endswith('.cif') and output_file.endswith('.png'):
        cif_to_png(input_file, output_file)
    else:
        print("Invalid file extensions")
        sys.exit(1)