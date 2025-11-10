# Deployment Guide

This guide covers deploying ACES HIGH: ENDLESS SKIES to various hosting platforms.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Building for Production](#building-for-production)
3. [Deployment Options](#deployment-options)
4. [Platform-Specific Guides](#platform-specific-guides)
5. [Performance Optimization](#performance-optimization)
6. [Monitoring and Analytics](#monitoring-and-analytics)

## Prerequisites

### Required Tools

- Node.js 18+ and npm 9+
- Rust 1.70+ with wasm32-unknown-unknown target
- wasm-pack 0.12+
- (Optional) wasm-opt from binaryen for additional optimization

### Required HTTP Headers

For SharedArrayBuffer support (future multiplayer features), your hosting must support these headers:

```
Cross-Origin-Embedder-Policy: require-corp
Cross-Origin-Opener-Policy: same-origin
```

## Building for Production

### Standard Build

```bash
# Run the production build script
./build.sh

# Or manually:
npm run build:prod
```

This will:
1. Build optimized WASM module
2. Bundle JavaScript with webpack
3. Compress assets with gzip
4. Output to `./dist` directory

### Build Output

```
dist/
├── index.html                  # Entry point
├── *.js                        # Bundled JavaScript
├── *.js.gz                     # Gzipped JavaScript
├── *.wasm                      # WebAssembly module
├── *.wasm.gz                   # Gzipped WASM
├── assets/                     # Game assets
│   ├── sprites/
│   ├── audio/
│   └── data/
└── manifest.json               # Asset manifest
```

### Build Size Targets

| Asset Type | Target Size | Notes |
|------------|-------------|-------|
| WASM (gzipped) | < 300KB | Core game logic |
| JS (gzipped) | < 100KB | Glue code + bootstrap |
| Total initial load | < 500KB | Before asset streaming |
| Full assets | < 50MB | All sprites, audio, data |

## Deployment Options

### Option 1: Static Hosting (Recommended)

Best for: Simple deployments, cost-effective hosting

**Pros:**
- Simple setup
- Low cost (often free)
- Global CDN distribution
- HTTPS by default

**Cons:**
- No server-side logic
- Limited configuration

**Recommended Providers:**
- GitHub Pages
- Netlify
- Vercel
- AWS S3 + CloudFront
- Firebase Hosting
- Cloudflare Pages

### Option 2: Traditional Web Server

Best for: Custom infrastructure, specific requirements

**Pros:**
- Full control over configuration
- Custom server logic possible
- Integrate with existing infrastructure

**Cons:**
- More maintenance
- Need to configure HTTPS, compression, etc.

**Compatible Servers:**
- Nginx
- Apache
- Node.js (Express, Fastify)
- Any static file server

### Option 3: Container Deployment

Best for: Microservices architecture, Kubernetes

**Pros:**
- Reproducible deployments
- Easy scaling
- Portable across environments

**Cons:**
- More complex setup
- Higher resource usage

## Platform-Specific Guides

### GitHub Pages

**Setup:**

1. Build the project:
   ```bash
   npm run build:prod
   ```

2. Add GitHub Actions workflow (`.github/workflows/deploy.yml`):
   ```yaml
   name: Deploy to GitHub Pages
   
   on:
     push:
       branches: [ main ]
   
   jobs:
     deploy:
       runs-on: ubuntu-latest
       
       steps:
       - uses: actions/checkout@v3
       
       - name: Setup Rust
         uses: actions-rs/toolchain@v1
         with:
           toolchain: stable
           target: wasm32-unknown-unknown
       
       - name: Install wasm-pack
         run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
       
       - name: Setup Node.js
         uses: actions/setup-node@v3
         with:
           node-version: '18'
       
       - name: Install dependencies
         run: npm ci
       
       - name: Build
         run: npm run build:prod
       
       - name: Deploy to GitHub Pages
         uses: peaceiris/actions-gh-pages@v3
         with:
           github_token: ${{ secrets.GITHUB_TOKEN }}
           publish_dir: ./dist
   ```

3. Enable GitHub Pages in repository settings
4. Set source to `gh-pages` branch

**Note:** GitHub Pages doesn't support custom headers. SharedArrayBuffer features will not work.

### Netlify

**Setup:**

1. Create `netlify.toml` in project root:
   ```toml
   [build]
     command = "npm run build:prod"
     publish = "dist"
   
   [[headers]]
     for = "/*"
     [headers.values]
       Cross-Origin-Embedder-Policy = "require-corp"
       Cross-Origin-Opener-Policy = "same-origin"
       X-Frame-Options = "DENY"
       X-Content-Type-Options = "nosniff"
       Referrer-Policy = "no-referrer"
   
   [[headers]]
     for = "*.wasm"
     [headers.values]
       Content-Type = "application/wasm"
       Cache-Control = "public, max-age=31536000, immutable"
   
   [[headers]]
     for = "*.js"
     [headers.values]
       Cache-Control = "public, max-age=31536000, immutable"
   
   [[redirects]]
     from = "/*"
     to = "/index.html"
     status = 200
   ```

2. Connect repository to Netlify
3. Deploy automatically on push

### Vercel

**Setup:**

1. Create `vercel.json` in project root:
   ```json
   {
     "buildCommand": "npm run build:prod",
     "outputDirectory": "dist",
     "headers": [
       {
         "source": "/(.*)",
         "headers": [
           {
             "key": "Cross-Origin-Embedder-Policy",
             "value": "require-corp"
           },
           {
             "key": "Cross-Origin-Opener-Policy",
             "value": "same-origin"
           }
         ]
       },
       {
         "source": "/(.*)\\.wasm",
         "headers": [
           {
             "key": "Content-Type",
             "value": "application/wasm"
           },
           {
             "key": "Cache-Control",
             "value": "public, max-age=31536000, immutable"
           }
         ]
       }
     ]
   }
   ```

2. Install Vercel CLI: `npm i -g vercel`
3. Deploy: `vercel --prod`

### AWS S3 + CloudFront

**Setup:**

1. Create S3 bucket for static hosting
2. Upload `dist/` contents to bucket
3. Create CloudFront distribution
4. Configure custom error responses
5. Add Lambda@Edge for custom headers:

```javascript
exports.handler = async (event) => {
    const response = event.Records[0].cf.response;
    const headers = response.headers;
    
    headers['cross-origin-embedder-policy'] = [{
        key: 'Cross-Origin-Embedder-Policy',
        value: 'require-corp'
    }];
    
    headers['cross-origin-opener-policy'] = [{
        key: 'Cross-Origin-Opener-Policy',
        value: 'same-origin'
    }];
    
    return response;
};
```

### Nginx

**Configuration:**

```nginx
server {
    listen 80;
    server_name aceshigh.example.com;
    root /var/www/aces-high/dist;
    index index.html;
    
    # Enable gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 256;
    gzip_types
        application/wasm
        application/javascript
        text/css
        application/json;
    
    # CORS headers for SharedArrayBuffer
    add_header Cross-Origin-Embedder-Policy "require-corp" always;
    add_header Cross-Origin-Opener-Policy "same-origin" always;
    
    # Security headers
    add_header X-Frame-Options "DENY" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header Referrer-Policy "no-referrer" always;
    
    # Serve pre-compressed files
    gzip_static on;
    
    # Cache static assets
    location ~* \.(wasm|js|css|png|jpg|jpeg|gif|ico|svg|ogg|mp3)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
    
    # Serve WASM files with correct MIME type
    location ~* \.wasm$ {
        types { application/wasm wasm; }
    }
    
    # Handle client-side routing
    location / {
        try_files $uri $uri/ /index.html;
    }
}
```

### Docker

**Dockerfile:**

```dockerfile
# Build stage
FROM rust:1.70 as builder

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
RUN apt-get install -y nodejs

WORKDIR /app

# Copy source
COPY . .

# Install dependencies and build
RUN npm ci
RUN npm run build:prod

# Production stage
FROM nginx:alpine

# Copy build output
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
```

**Build and run:**

```bash
# Build image
docker build -t aces-high .

# Run container
docker run -p 8080:80 aces-high
```

## Performance Optimization

### CDN Configuration

1. **Enable HTTP/2** - Multiplexing, server push
2. **Enable Brotli compression** - Better than gzip for text
3. **Set appropriate cache headers** - Long-term caching for immutable assets
4. **Use edge locations** - Reduce latency globally

### Asset Optimization

```bash
# Optimize images
pngcrush -rem alla -reduce -brute sprites/*.png

# Optimize audio
ffmpeg -i input.wav -c:a libvorbis -q:a 4 output.ogg

# Generate multiple resolutions for responsive images
convert sprite.png -resize 50% sprite@0.5x.png
```

### Progressive Loading

Implement progressive asset loading to reduce initial load time:

1. **Critical path**: Load essential assets first
   - Core WASM module
   - UI sprites
   - Player aircraft sprite
   - Essential audio

2. **Secondary assets**: Load in background
   - Enemy sprites
   - Background music
   - Sound effects
   - Additional zones

3. **On-demand**: Load as needed
   - Boss sprites
   - Achievement icons
   - Leaderboard data

### Performance Monitoring

Add analytics to track:
- Load times
- Frame rates
- Error rates
- User engagement

```javascript
// Example with Google Analytics
gtag('event', 'timing_complete', {
    name: 'wasm_load',
    value: loadTime,
    event_category: 'performance'
});
```

## Monitoring and Analytics

### Error Tracking

Integrate error tracking service:

```javascript
window.addEventListener('error', (event) => {
    // Send to error tracking service
    trackError({
        message: event.message,
        stack: event.error?.stack,
        browser: navigator.userAgent,
        url: window.location.href
    });
});
```

### Performance Monitoring

```javascript
// Measure key metrics
const observer = new PerformanceObserver((list) => {
    for (const entry of list.getEntries()) {
        if (entry.entryType === 'measure') {
            trackPerformance({
                name: entry.name,
                duration: entry.duration
            });
        }
    }
});

observer.observe({ entryTypes: ['measure'] });

// Measure WASM load time
performance.mark('wasm-start');
await init();
performance.mark('wasm-end');
performance.measure('wasm-load', 'wasm-start', 'wasm-end');
```

## Troubleshooting

### Common Issues

**WASM fails to load:**
- Check MIME type is `application/wasm`
- Verify file exists and path is correct
- Check browser console for CORS errors

**Performance issues:**
- Enable compression (gzip/brotli)
- Use CDN for asset delivery
- Check network waterfall in DevTools
- Profile with Chrome Performance tab

**Audio not playing:**
- User interaction required to start AudioContext
- Check audio format support (OGG, MP3)
- Verify file paths are correct

**Save data not persisting:**
- Check IndexedDB is available
- Verify storage quota
- Test in incognito mode

## Security Considerations

1. **Content Security Policy:**
   ```html
   <meta http-equiv="Content-Security-Policy" 
         content="default-src 'self'; 
                  script-src 'self' 'wasm-unsafe-eval';
                  connect-src 'self' https://api.example.com;">
   ```

2. **Subresource Integrity:** Generate SRI hashes for assets
3. **HTTPS Only:** Force HTTPS in production
4. **Rate Limiting:** Implement if you have API endpoints

## Rollback Procedure

If issues occur after deployment:

1. **Immediate rollback:**
   ```bash
   # Revert to previous version
   git revert HEAD
   git push origin main
   ```

2. **Verify rollback:**
   - Check site loads correctly
   - Test core functionality
   - Monitor error rates

3. **Post-mortem:**
   - Identify root cause
   - Add tests to prevent regression
   - Document incident

## Checklist

Before deploying to production:

- [ ] Run full test suite (`npm test`)
- [ ] Build production bundle (`npm run build:prod`)
- [ ] Test production build locally
- [ ] Verify WASM loads correctly
- [ ] Check all assets load
- [ ] Test on multiple browsers
- [ ] Test on mobile devices
- [ ] Verify analytics tracking
- [ ] Check error tracking
- [ ] Review security headers
- [ ] Test performance (60 FPS)
- [ ] Verify save/load works
- [ ] Update CHANGELOG.md
- [ ] Tag release in git
- [ ] Deploy to staging first
- [ ] Smoke test on staging
- [ ] Deploy to production
- [ ] Monitor for errors

---

For questions or issues, please open an issue on GitHub.
