// Create a preloader for critical assets
class AssetPreloader {
  private loadedAssets: Set<string> = new Set();
  private loadQueue: Map<string, Promise<any>> = new Map();
  private priorities: Map<string, number> = new Map();
  
  constructor() {
    // Initialize with default priorities
    this.setPriority('css/forum.css', 1);
    this.setPriority('js/forum-core.js', 1);
    this.setPriority('images/forum-icons.svg', 2);
    this.setPriority('fonts/forum-font.woff2', 2);
  }
  
  setPriority(assetPath: string, priority: number): void {
    this.priorities.set(assetPath, priority);
  }
  
  preload(assetPath: string): Promise<any> {
    if (this.loadedAssets.has(assetPath)) {
      return Promise.resolve();
    }
    
    if (this.loadQueue.has(assetPath)) {
      return this.loadQueue.get(assetPath)!;
    }
    
    const promise = this.loadAsset(assetPath);
    this.loadQueue.set(assetPath, promise);
    
    promise.then(() => {
      this.loadedAssets.add(assetPath);
      this.loadQueue.delete(assetPath);
    });
    
    return promise;
  }
  
  preloadAll(priority: number = 1): void {
    for (const [asset, assetPriority] of this.priorities.entries()) {
      if (assetPriority <= priority && !this.loadedAssets.has(asset)) {
        this.preload(asset);
      }
    }
  }
  
  preloadCritical(): void {
    this.preloadAll(1);
  }
  
  private loadAsset(assetPath: string): Promise<any> {
    if (assetPath.endsWith('.css')) {
      return this.loadStylesheet(assetPath);
    } else if (assetPath.endsWith('.js')) {
      return this.loadScript(assetPath);
    } else if (assetPath.endsWith('.woff2') || assetPath.endsWith('.woff') || assetPath.endsWith('.ttf')) {
      return this.loadFont(assetPath);
    } else if (assetPath.endsWith('.svg') || assetPath.endsWith('.png') || assetPath.endsWith('.jpg') || assetPath.endsWith('.webp')) {
      return this.loadImage(assetPath);
    } else {
      return this.loadGeneric(assetPath);
    }
  }
  
  private loadStylesheet(href: string): Promise<void> {
    return new Promise((resolve, reject) => {
      const link = document.createElement('link');
      link.rel = 'stylesheet';
      link.href = href;
      link.onload = () => resolve();
      link.onerror = () => reject(new Error(`Failed to load stylesheet: ${href}`));
      document.head.appendChild(link);
    });
  }
  
  private loadScript(src: string): Promise<void> {
    return new Promise((resolve, reject) => {
      const script = document.createElement('script');
      script.src = src;
      script.async = true;
      script.onload = () => resolve();
      script.onerror = () => reject(new Error(`Failed to load script: ${src}`));
      document.head.appendChild(script);
    });
  }
  
  private loadImage(src: string): Promise<void> {
    return new Promise((resolve, reject) => {
      const img = new Image();
      img.src = src;
      img.onload = () => resolve();
      img.onerror = () => reject(new Error(`Failed to load image: ${src}`));
    });
  }
  
  private loadFont(src: string): Promise<void> {
    return new Promise((resolve) => {
      const fontFace = new FontFace('Forum Font', `url(${src})`);
      fontFace.load().then((loaded) => {
        (document.fonts as any).add(loaded);
        resolve();
      }).catch(() => {
        // Font loading failures should not block the app
        resolve();
      });
    });
  }
  
  private loadGeneric(src: string): Promise<void> {
    return new Promise((resolve, reject) => {
      fetch(src).then(() => resolve()).catch(() => reject());
    });
  }
}

// Export singleton instance
export const assetPreloader = new AssetPreloader();

// Create a component for lazy loading images
export function createLazyImage(src: string, alt: string, className: string = ''): HTMLElement {
  const container = document.createElement('div');
  container.className = `lazy-image-container ${className}`;
  
  const img = document.createElement('img');
  img.alt = alt;
  img.loading = 'lazy';
  img.dataset.src = src;
  
  const placeholder = document.createElement('div');
  placeholder.className = 'lazy-placeholder';
  
  container.appendChild(placeholder);
  container.appendChild(img);
  
  // Set up intersection observer to load when visible
  const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        const target = entry.target as HTMLImageElement;
        if (target.dataset.src) {
          target.src = target.dataset.src;
          delete target.dataset.src;
        }
        observer.unobserve(target);
      }
    });
  }, {
    rootMargin: '100px 0px', // Start loading 100px before visible
    threshold: 0.01
  });
  
  observer.observe(img);
  
  return container;
}