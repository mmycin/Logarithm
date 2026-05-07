# GitHub Pages Setup Guide

## Enable GitHub Pages

Follow these steps to publish your landing page:

### 1. Go to Repository Settings

1. Navigate to your repository: `https://github.com/mmycin/Logarithm`
2. Click on **Settings** tab
3. Scroll down to **Pages** section in the left sidebar

### 2. Configure Source

1. Under **Source**, select **Deploy from a branch**
2. Under **Branch**, select:
   - Branch: `main`
   - Folder: `/docs`
3. Click **Save**

### 3. Wait for Deployment

- GitHub will automatically build and deploy your site
- This usually takes 1-2 minutes
- You'll see a green checkmark when it's ready

### 4. Access Your Site

Your landing page will be available at:
```
https://mmycin.github.io/Logarithm/
```

## Custom Domain (Optional)

If you want to use a custom domain:

1. In the **Pages** settings, enter your custom domain
2. Add a `CNAME` file to the `docs/` folder with your domain
3. Configure DNS records with your domain provider:
   - Add a `CNAME` record pointing to `mmycin.github.io`

## Landing Page Features

✅ **Responsive Design** - Works on all devices
✅ **Dark/Light Theme** - Automatic theme switching with toggle
✅ **Dynamic Images** - Images change based on theme
✅ **Smooth Animations** - Fade-in effects and hover states
✅ **SEO Optimized** - Meta tags and Open Graph support
✅ **Fast Loading** - Optimized assets and minimal dependencies

## Page Sections

1. **Hero Section**
   - Project title and description
   - Call-to-action buttons
   - Hero image (changes with theme)
   - Technology badges

2. **Features Section**
   - 9 feature cards highlighting key capabilities
   - Icons and descriptions
   - Hover effects

3. **Screenshots Section**
   - Main screenshot showcase
   - Grid of additional screenshots
   - Theme-aware image switching

4. **Download Section**
   - Platform-specific download cards (Windows, macOS, Linux)
   - Direct links to latest releases
   - Build from source option

5. **Footer**
   - Links to documentation and resources
   - Credits and acknowledgments

## Updating the Landing Page

To update the landing page:

1. Edit `docs/index.html`
2. Update images in `docs/` folder
3. Commit and push changes:
   ```bash
   git add docs/
   git commit -m "Update landing page"
   git push origin main
   ```
4. GitHub Pages will automatically redeploy (1-2 minutes)

## Theme Toggle

The landing page includes a theme toggle that:
- Saves preference to localStorage
- Switches between dark and light mode
- Updates all images to match the theme
- Provides smooth transitions

## Image Assets

The following images are used:
- `LoganIcon.png` - Logo in header
- `hero.png` - Hero image (dark theme)
- `hero_light.png` - Hero image (light theme)
- `ss_dark.jpg` - Main screenshot (dark theme)
- `ss_light.png` - Main screenshot (light theme)
- `ss2_dark.jpg` - Secondary screenshot (dark theme)
- `ss2_light.jpg` - Secondary screenshot (light theme)
- `banner.jpg` - Social media preview image

## Troubleshooting

### Page Not Loading
- Check that GitHub Pages is enabled in Settings
- Verify the branch is set to `main` and folder to `/docs`
- Wait a few minutes for deployment to complete

### Images Not Showing
- Ensure all images are in the `docs/` folder
- Check that image paths in HTML are relative (`./image.png`)
- Clear browser cache and refresh

### Theme Not Switching
- Check browser console for JavaScript errors
- Ensure localStorage is enabled in browser
- Try in incognito/private mode

## Next Steps

After enabling GitHub Pages:

1. ✅ Share the link on social media
2. ✅ Add the link to your repository description
3. ✅ Update README.md with the landing page link
4. ✅ Consider adding Google Analytics (optional)
5. ✅ Set up custom domain (optional)

## Support

If you encounter issues:
- Check [GitHub Pages documentation](https://docs.github.com/en/pages)
- Open an issue in the repository
- Contact the maintainer

---

**Status**: ✅ Landing page created and ready to deploy!

**URL**: https://mmycin.github.io/Logarithm/ (after enabling GitHub Pages)
