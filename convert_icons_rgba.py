#!/usr/bin/env python3
"""Convert PNG icons to RGBA format for Tauri."""

from PIL import Image
import os

# Icon sizes to generate
sizes = [32, 64, 128, 256]

# Source image
source = "public/StoreLogo.png"

# Output directory
output_dir = "src-tauri/icons"

# Open source image
print(f"Opening source image: {source}")
img = Image.open(source)

# Convert to RGBA if not already
if img.mode != 'RGBA':
    print(f"Converting from {img.mode} to RGBA")
    img = img.convert('RGBA')
else:
    print("Source is already RGBA")

# Generate icons
for size in sizes:
    output_path = os.path.join(output_dir, f"{size}x{size}.png")
    print(f"Generating {output_path}...")
    
    # Resize with high-quality resampling
    resized = img.resize((size, size), Image.Resampling.LANCZOS)
    
    # Save as RGBA PNG
    resized.save(output_path, 'PNG', optimize=False)
    
    # Verify
    test = Image.open(output_path)
    print(f"  ✓ Saved as {test.mode} ({test.size[0]}x{test.size[1]})")

# Also save 128@2x
output_path = os.path.join(output_dir, "128x128@2x.png")
print(f"Generating {output_path}...")
resized = img.resize((256, 256), Image.Resampling.LANCZOS)
resized.save(output_path, 'PNG', optimize=False)
test = Image.open(output_path)
print(f"  ✓ Saved as {test.mode} ({test.size[0]}x{test.size[1]})")

# Save main icon.png
output_path = os.path.join(output_dir, "icon.png")
print(f"Generating {output_path}...")
img.save(output_path, 'PNG', optimize=False)
test = Image.open(output_path)
print(f"  ✓ Saved as {test.mode} ({test.size[0]}x{test.size[1]})")

print("\n✅ All icons converted to RGBA format!")
