// Charming.js - A lightweight charting library for Rust/WASM applications
// This is a minimal implementation that provides the core functionality needed
// for the Quiz Module's visualization components.

(function(global) {
  'use strict';

  // Check if Charming is already defined
  if (global.Charming) {
    return;
  }

  // Define the Charming object
  const Charming = {
    // Version
    version: '1.0.0',

    // Create a new chart
    create: function(canvas, config) {
      if (!canvas || !(canvas instanceof HTMLCanvasElement)) {
        console.error('Charming: canvas element is required');
        return null;
      }

      if (!config || !config.type) {
        console.error('Charming: config with type is required');
        return null;
      }

      // Create a chart instance
      const chart = new CharmingChart(canvas, config);
      chart.render();
      return chart;
    },

    // Register a new chart type
    registerChartType: function(type, implementation) {
      ChartTypes[type] = implementation;
    }
  };

  // Chart types implementations
  const ChartTypes = {
    // Bar chart implementation
    bar: function(ctx, data, options) {
      const { width, height } = ctx.canvas;
      const { labels, datasets } = data;
      const barWidth = width / (labels.length * datasets.length + 1);
      const maxValue = Math.max(...datasets.flatMap(d => d.data));
      const scaleFactor = (height - 60) / (maxValue || 1);

      // Draw background
      ctx.fillStyle = options?.backgroundColor || '#f8f9fa';
      ctx.fillRect(0, 0, width, height);

      // Draw title
      if (options?.title?.text) {
        ctx.fillStyle = options.title.color || '#333';
        ctx.font = options.title.font || 'bold 16px Arial';
        ctx.textAlign = 'center';
        ctx.fillText(options.title.text, width / 2, 30);
      }

      // Draw bars
      datasets.forEach((dataset, datasetIndex) => {
        const barColor = dataset.backgroundColor || 
          (Array.isArray(dataset.backgroundColor) ? dataset.backgroundColor[0] : '#4bc0c0');
        
        ctx.fillStyle = barColor;
        ctx.strokeStyle = dataset.borderColor || '#333';
        
        dataset.data.forEach((value, index) => {
          const x = (index * datasets.length + datasetIndex) * barWidth + barWidth;
          const barHeight = value * scaleFactor;
          const y = height - barHeight - 30;
          
          ctx.fillRect(x, y, barWidth * 0.8, barHeight);
          ctx.strokeRect(x, y, barWidth * 0.8, barHeight);
          
          // Draw value on top of bar
          ctx.fillStyle = '#333';
          ctx.font = '12px Arial';
          ctx.textAlign = 'center';
          ctx.fillText(value.toString(), x + barWidth * 0.4, y - 5);
        });
      });

      // Draw x-axis labels
      ctx.fillStyle = '#333';
      ctx.font = '12px Arial';
      ctx.textAlign = 'center';
      labels.forEach((label, index) => {
        const x = (index * datasets.length + datasets.length / 2) * barWidth + barWidth;
        ctx.fillText(label, x, height - 10);
      });

      // Draw legend
      if (datasets.length > 1) {
        const legendY = 50;
        datasets.forEach((dataset, index) => {
          const legendX = width - 150;
          
          ctx.fillStyle = dataset.backgroundColor || '#4bc0c0';
          ctx.fillRect(legendX, legendY + index * 20, 15, 15);
          ctx.strokeStyle = '#333';
          ctx.strokeRect(legendX, legendY + index * 20, 15, 15);
          
          ctx.fillStyle = '#333';
          ctx.font = '12px Arial';
          ctx.textAlign = 'left';
          ctx.fillText(dataset.label || `Dataset ${index + 1}`, legendX + 20, legendY + index * 20 + 12);
        });
      }
    },

    // Line chart implementation
    line: function(ctx, data, options) {
      const { width, height } = ctx.canvas;
      const { labels, datasets } = data;
      const maxValue = Math.max(...datasets.flatMap(d => d.data));
      const scaleFactor = (height - 60) / (maxValue || 1);
      const xStep = (width - 60) / (labels.length - 1 || 1);

      // Draw background
      ctx.fillStyle = options?.backgroundColor || '#f8f9fa';
      ctx.fillRect(0, 0, width, height);

      // Draw title
      if (options?.title?.text) {
        ctx.fillStyle = options.title.color || '#333';
        ctx.font = options.title.font || 'bold 16px Arial';
        ctx.textAlign = 'center';
        ctx.fillText(options.title.text, width / 2, 30);
      }

      // Draw grid lines
      ctx.strokeStyle = '#ddd';
      ctx.lineWidth = 0.5;
      for (let i = 0; i <= 5; i++) {
        const y = height - 30 - (i * (height - 60) / 5);
        ctx.beginPath();
        ctx.moveTo(30, y);
        ctx.lineTo(width - 30, y);
        ctx.stroke();
        
        // Draw y-axis labels
        ctx.fillStyle = '#666';
        ctx.font = '10px Arial';
        ctx.textAlign = 'right';
        ctx.fillText((maxValue * i / 5).toFixed(1), 25, y + 4);
      }

      // Draw datasets
      datasets.forEach((dataset, datasetIndex) => {
        const lineColor = dataset.borderColor || '#4bc0c0';
        const fillColor = dataset.backgroundColor || 'rgba(75, 192, 192, 0.2)';
        
        ctx.strokeStyle = lineColor;
        ctx.lineWidth = 2;
        ctx.beginPath();
        
        dataset.data.forEach((value, index) => {
          const x = 30 + index * xStep;
          const y = height - 30 - (value * scaleFactor);
          
          if (index === 0) {
            ctx.moveTo(x, y);
          } else {
            ctx.lineTo(x, y);
          }
          
          // Draw points
          ctx.fillStyle = lineColor;
          ctx.beginPath();
          ctx.arc(x, y, 4, 0, Math.PI * 2);
          ctx.fill();
        });
        
        ctx.stroke();
        
        // Fill area if specified
        if (dataset.fill) {
          ctx.fillStyle = fillColor;
          ctx.beginPath();
          
          dataset.data.forEach((value, index) => {
            const x = 30 + index * xStep;
            const y = height - 30 - (value * scaleFactor);
            
            if (index === 0) {
              ctx.moveTo(x, y);
            } else {
              ctx.lineTo(x, y);
            }
          });
          
          ctx.lineTo(30 + (dataset.data.length - 1) * xStep, height - 30);
          ctx.lineTo(30, height - 30);
          ctx.closePath();
          ctx.fill();
        }
      });

      // Draw x-axis labels
      ctx.fillStyle = '#333';
      ctx.font = '12px Arial';
      ctx.textAlign = 'center';
      labels.forEach((label, index) => {
        const x = 30 + index * xStep;
        ctx.fillText(label, x, height - 10);
      });

      // Draw legend
      if (datasets.length > 1) {
        const legendY = 50;
        datasets.forEach((dataset, index) => {
          const legendX = width - 150;
          
          ctx.strokeStyle = dataset.borderColor || '#4bc0c0';
          ctx.lineWidth = 2;
          ctx.beginPath();
          ctx.moveTo(legendX, legendY + index * 20 + 7);
          ctx.lineTo(legendX + 15, legendY + index * 20 + 7);
          ctx.stroke();
          
          ctx.fillStyle = dataset.borderColor || '#4bc0c0';
          ctx.beginPath();
          ctx.arc(legendX + 7, legendY + index * 20 + 7, 3, 0, Math.PI * 2);
          ctx.fill();
          
          ctx.fillStyle = '#333';
          ctx.font = '12px Arial';
          ctx.textAlign = 'left';
          ctx.fillText(dataset.label || `Dataset ${index + 1}`, legendX + 20, legendY + index * 20 + 12);
        });
      }
    },

    // Pie chart implementation
    pie: function(ctx, data, options) {
      const { width, height } = ctx.canvas;
      const { labels, datasets } = data;
      const dataset = datasets[0]; // Pie charts typically have one dataset
      const total = dataset.data.reduce((sum, value) => sum + value, 0);
      const radius = Math.min(width, height) / 2 - 50;
      const centerX = width / 2;
      const centerY = height / 2;

      // Draw background
      ctx.fillStyle = options?.backgroundColor || '#f8f9fa';
      ctx.fillRect(0, 0, width, height);

      // Draw title
      if (options?.title?.text) {
        ctx.fillStyle = options.title.color || '#333';
        ctx.font = options.title.font || 'bold 16px Arial';
        ctx.textAlign = 'center';
        ctx.fillText(options.title.text, width / 2, 30);
      }

      // Generate colors if not provided
      const colors = dataset.backgroundColor || 
        Array(dataset.data.length).fill().map((_, i) => {
          const hue = (i * 137) % 360;
          return `hsl(${hue}, 70%, 60%)`;
        });

      // Draw pie slices
      let startAngle = 0;
      dataset.data.forEach((value, index) => {
        const sliceAngle = (value / total) * Math.PI * 2;
        const endAngle = startAngle + sliceAngle;
        
        ctx.fillStyle = Array.isArray(colors) ? colors[index % colors.length] : colors;
        ctx.beginPath();
        ctx.moveTo(centerX, centerY);
        ctx.arc(centerX, centerY, radius, startAngle, endAngle);
        ctx.closePath();
        ctx.fill();
        
        ctx.strokeStyle = '#fff';
        ctx.lineWidth = 2;
        ctx.stroke();
        
        // Draw percentage in the middle of the slice
        const midAngle = startAngle + sliceAngle / 2;
        const labelRadius = radius * 0.7;
        const labelX = centerX + Math.cos(midAngle) * labelRadius;
        const labelY = centerY + Math.sin(midAngle) * labelRadius;
        
        const percentage = ((value / total) * 100).toFixed(1) + '%';
        
        ctx.fillStyle = '#fff';
        ctx.font = 'bold 12px Arial';
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';
        ctx.fillText(percentage, labelX, labelY);
        
        startAngle = endAngle;
      });

      // Draw legend
      const legendY = height - 20 - (labels.length * 20);
      labels.forEach((label, index) => {
        const legendX = width - 150;
        
        ctx.fillStyle = Array.isArray(colors) ? colors[index % colors.length] : colors;
        ctx.fillRect(legendX, legendY + index * 20, 15, 15);
        ctx.strokeStyle = '#333';
        ctx.strokeRect(legendX, legendY + index * 20, 15, 15);
        
        ctx.fillStyle = '#333';
        ctx.font = '12px Arial';
        ctx.textAlign = 'left';
        ctx.fillText(label, legendX + 20, legendY + index * 20 + 12);
      });
    },

    // Radar chart implementation
    radar: function(ctx, data, options) {
      const { width, height } = ctx.canvas;
      const { labels, datasets } = data;
      const centerX = width / 2;
      const centerY = height / 2;
      const radius = Math.min(width, height) / 2 - 50;
      const sides = labels.length;
      const angleStep = (Math.PI * 2) / sides;

      // Draw background
      ctx.fillStyle = options?.backgroundColor || '#f8f9fa';
      ctx.fillRect(0, 0, width, height);

      // Draw title
      if (options?.title?.text) {
        ctx.fillStyle = options.title.color || '#333';
        ctx.font = options.title.font || 'bold 16px Arial';
        ctx.textAlign = 'center';
        ctx.fillText(options.title.text, width / 2, 30);
      }

      // Draw radar grid
      ctx.strokeStyle = '#ddd';
      ctx.lineWidth = 1;
      
      // Draw concentric circles
      for (let r = 0.2; r <= 1; r += 0.2) {
        ctx.beginPath();
        for (let i = 0; i < sides; i++) {
          const angle = i * angleStep - Math.PI / 2;
          const x = centerX + Math.cos(angle) * radius * r;
          const y = centerY + Math.sin(angle) * radius * r;
          
          if (i === 0) {
            ctx.moveTo(x, y);
          } else {
            ctx.lineTo(x, y);
          }
        }
        ctx.closePath();
        ctx.stroke();
      }
      
      // Draw radial lines
      for (let i = 0; i < sides; i++) {
        const angle = i * angleStep - Math.PI / 2;
        ctx.beginPath();
        ctx.moveTo(centerX, centerY);
        ctx.lineTo(
          centerX + Math.cos(angle) * radius,
          centerY + Math.sin(angle) * radius
        );
        ctx.stroke();
        
        // Draw labels
        const labelX = centerX + Math.cos(angle) * (radius + 20);
        const labelY = centerY + Math.sin(angle) * (radius + 20);
        
        ctx.fillStyle = '#333';
        ctx.font = '12px Arial';
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';
        ctx.fillText(labels[i], labelX, labelY);
      }

      // Draw datasets
      datasets.forEach((dataset, datasetIndex) => {
        const lineColor = dataset.borderColor || '#4bc0c0';
        const fillColor = dataset.backgroundColor || 'rgba(75, 192, 192, 0.2)';
        
        // Normalize data to 0-1 range
        const maxValue = Math.max(...dataset.data);
        const normalizedData = dataset.data.map(value => value / maxValue);
        
        // Draw filled area
        ctx.fillStyle = fillColor;
        ctx.beginPath();
        
        normalizedData.forEach((value, index) => {
          const angle = index * angleStep - Math.PI / 2;
          const x = centerX + Math.cos(angle) * radius * value;
          const y = centerY + Math.sin(angle) * radius * value;
          
          if (index === 0) {
            ctx.moveTo(x, y);
          } else {
            ctx.lineTo(x, y);
          }
        });
        
        ctx.closePath();
        ctx.fill();
        
        // Draw outline
        ctx.strokeStyle = lineColor;
        ctx.lineWidth = 2;
        ctx.beginPath();
        
        normalizedData.forEach((value, index) => {
          const angle = index * angleStep - Math.PI / 2;
          const x = centerX + Math.cos(angle) * radius * value;
          const y = centerY + Math.sin(angle) * radius * value;
          
          if (index === 0) {
            ctx.moveTo(x, y);
          } else {
            ctx.lineTo(x, y);
          }
        });
        
        ctx.closePath();
        ctx.stroke();
        
        // Draw points
        normalizedData.forEach((value, index) => {
          const angle = index * angleStep - Math.PI / 2;
          const x = centerX + Math.cos(angle) * radius * value;
          const y = centerY + Math.sin(angle) * radius * value;
          
          ctx.fillStyle = lineColor;
          ctx.beginPath();
          ctx.arc(x, y, 4, 0, Math.PI * 2);
          ctx.fill();
        });
      });

      // Draw legend
      if (datasets.length > 1) {
        const legendY = 50;
        datasets.forEach((dataset, index) => {
          const legendX = width - 150;
          
          ctx.fillStyle = dataset.backgroundColor || 'rgba(75, 192, 192, 0.2)';
          ctx.fillRect(legendX, legendY + index * 20, 15, 15);
          ctx.strokeStyle = dataset.borderColor || '#4bc0c0';
          ctx.strokeRect(legendX, legendY + index * 20, 15, 15);
          
          ctx.fillStyle = '#333';
          ctx.font = '12px Arial';
          ctx.textAlign = 'left';
          ctx.fillText(dataset.label || `Dataset ${index + 1}`, legendX + 20, legendY + index * 20 + 12);
        });
      }
    },

    // Scatter chart implementation
    scatter: function(ctx, data, options) {
      const { width, height } = ctx.canvas;
      const { datasets } = data;
      
      // Find min/max values for scaling
      let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
      
      datasets.forEach(dataset => {
        dataset.data.forEach(point => {
          minX = Math.min(minX, point.x);
          maxX = Math.max(maxX, point.x);
          minY = Math.min(minY, point.y);
          maxY = Math.max(maxY, point.y);
        });
      });
      
      // Add some padding
      const rangeX = maxX - minX || 1;
      const rangeY = maxY - minY || 1;
      
      minX -= rangeX * 0.1;
      maxX += rangeX * 0.1;
      minY -= rangeY * 0.1;
      maxY += rangeY * 0.1;
      
      const scaleX = (width - 80) / (maxX - minX);
      const scaleY = (height - 80) / (maxY - minY);

      // Draw background
      ctx.fillStyle = options?.backgroundColor || '#f8f9fa';
      ctx.fillRect(0, 0, width, height);

      // Draw title
      if (options?.title?.text) {
        ctx.fillStyle = options.title.color || '#333';
        ctx.font = options.title.font || 'bold 16px Arial';
        ctx.textAlign = 'center';
        ctx.fillText(options.title.text, width / 2, 30);
      }

      // Draw axes
      ctx.strokeStyle = '#333';
      ctx.lineWidth = 1;
      
      // X-axis
      ctx.beginPath();
      ctx.moveTo(40, height - 40);
      ctx.lineTo(width - 40, height - 40);
      ctx.stroke();
      
      // Y-axis
      ctx.beginPath();
      ctx.moveTo(40, 40);
      ctx.lineTo(40, height - 40);
      ctx.stroke();
      
      // Draw grid
      ctx.strokeStyle = '#ddd';
      ctx.lineWidth = 0.5;
      
      // X-grid
      for (let i = 0; i <= 10; i++) {
        const x = 40 + i * (width - 80) / 10;
        ctx.beginPath();
        ctx.moveTo(x, 40);
        ctx.lineTo(x, height - 40);
        ctx.stroke();
        
        // X-axis labels
        const value = minX + (i / 10) * (maxX - minX);
        ctx.fillStyle = '#666';
        ctx.font = '10px Arial';
        ctx.textAlign = 'center';
        ctx.fillText(value.toFixed(1), x, height - 25);
      }
      
      // Y-grid
      for (let i = 0; i <= 10; i++) {
        const y = height - 40 - i * (height - 80) / 10;
        ctx.beginPath();
        ctx.moveTo(40, y);
        ctx.lineTo(width - 40, y);
        ctx.stroke();
        
        // Y-axis labels
        const value = minY + (i / 10) * (maxY - minY);
        ctx.fillStyle = '#666';
        ctx.font = '10px Arial';
        ctx.textAlign = 'right';
        ctx.fillText(value.toFixed(1), 35, y + 4);
      }

      // Draw datasets
      datasets.forEach((dataset, datasetIndex) => {
        const pointColor = dataset.backgroundColor || '#4bc0c0';
        const pointRadius = dataset.pointRadius || 5;
        
        dataset.data.forEach(point => {
          const x = 40 + (point.x - minX) * scaleX;
          const y = height - 40 - (point.y - minY) * scaleY;
          
          ctx.fillStyle = pointColor;
          ctx.beginPath();
          ctx.arc(x, y, pointRadius, 0, Math.PI * 2);
          ctx.fill();
          
          ctx.strokeStyle = '#fff';
          ctx.lineWidth = 1;
          ctx.stroke();
        });
      });

      // Draw legend
      if (datasets.length > 1) {
        const legendY = 50;
        datasets.forEach((dataset, index) => {
          const legendX = width - 150;
          
          ctx.fillStyle = dataset.backgroundColor || '#4bc0c0';
          ctx.beginPath();
          ctx.arc(legendX + 7, legendY + index * 20 + 7, 5, 0, Math.PI * 2);
          ctx.fill();
          
          ctx.strokeStyle = '#fff';
          ctx.lineWidth = 1;
          ctx.stroke();
          
          ctx.fillStyle = '#333';
          ctx.font = '12px Arial';
          ctx.textAlign = 'left';
          ctx.fillText(dataset.label || `Dataset ${index + 1}`, legendX + 20, legendY + index * 20 + 12);
        });
      }
    }
  };

  // CharmingChart class
  class CharmingChart {
    constructor(canvas, config) {
      this.canvas = canvas;
      this.ctx = canvas.getContext('2d');
      this.config = config;
      this.type = config.type;
      this.data = config.data;
      this.options = config.options || {};
    }

    // Render the chart
    render() {
      // Clear canvas
      this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

      // Check if chart type is supported
      if (!ChartTypes[this.type]) {
        console.error(`Charming: chart type "${this.type}" is not supported`);
        this.renderError();
        return;
      }

      // Render the chart
      try {
        ChartTypes[this.type](this.ctx, this.data, this.options);
      } catch (error) {
        console.error('Charming: error rendering chart', error);
        this.renderError();
      }
    }

    // Render an error message
    renderError() {
      const { width, height } = this.canvas;

      this.ctx.fillStyle = '#f8d7da';
      this.ctx.fillRect(0, 0, width, height);

      this.ctx.fillStyle = '#721c24';
      this.ctx.font = 'bold 16px Arial';
      this.ctx.textAlign = 'center';
      this.ctx.fillText('Error rendering chart', width / 2, height / 2);
    }

    // Update chart data
    update(data) {
      this.data = data;
      this.render();
    }

    // Destroy the chart
    destroy() {
      this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    }
  }

  // Expose Charming to the global object
  global.Charming = Charming;

})(typeof window !== 'undefined' ? window : this);
