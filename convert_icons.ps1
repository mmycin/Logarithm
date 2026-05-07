# PowerShell script to convert icons to RGBA format
Add-Type -AssemblyName System.Drawing

$sourcePath = "public\StoreLogo.png"
$outputDir = "src-tauri\icons"

# Load source image
Write-Host "Loading source image: $sourcePath"
$sourceImage = [System.Drawing.Image]::FromFile((Resolve-Path $sourcePath))

# Icon sizes to generate
$sizes = @(32, 64, 128, 256)

foreach ($size in $sizes) {
    $outputPath = Join-Path $outputDir "${size}x${size}.png"
    Write-Host "Generating $outputPath..."
    
    # Create new bitmap with RGBA format
    $bitmap = New-Object System.Drawing.Bitmap($size, $size, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
    $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $graphics.DrawImage($sourceImage, 0, 0, $size, $size)
    $graphics.Dispose()
    
    # Save as PNG
    $bitmap.Save($outputPath, [System.Drawing.Imaging.ImageFormat]::Png)
    $bitmap.Dispose()
    
    Write-Host "  Saved ${size}x${size}.png"
}

# Generate 128@2x (256x256)
$outputPath = Join-Path $outputDir "128x128@2x.png"
Write-Host "Generating $outputPath..."
$bitmap = New-Object System.Drawing.Bitmap(256, 256, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)
$graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
$graphics.DrawImage($sourceImage, 0, 0, 256, 256)
$graphics.Dispose()
$bitmap.Save($outputPath, [System.Drawing.Imaging.ImageFormat]::Png)
$bitmap.Dispose()
Write-Host "  Saved 128x128@2x.png"

# Copy source as icon.png
$outputPath = Join-Path $outputDir "icon.png"
Write-Host "Copying source to $outputPath..."
Copy-Item $sourcePath $outputPath -Force
Write-Host "  Saved icon.png"

$sourceImage.Dispose()

Write-Host ""
Write-Host "All icons converted to RGBA format!"
