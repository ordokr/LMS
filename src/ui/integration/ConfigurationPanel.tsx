import React, { useState } from 'react';
import {
  Paper,
  Typography,
  Box,
  TextField,
  Button,
  Grid,
  Divider,
  FormControlLabel,
  Switch,
  InputAdornment,
  IconButton,
  Alert,
  CircularProgress
} from '@mui/material';
import {
  Visibility as VisibilityIcon,
  VisibilityOff as VisibilityOffIcon,
  Save as SaveIcon,
  Check as CheckIcon
} from '@mui/icons-material';
import { ApiConfig } from '../../types/config';
import { testCanvasConnection, testDiscourseConnection } from '../../api/configApi';

interface ConfigurationPanelProps {
  config: ApiConfig;
  onSave: (config: ApiConfig) => Promise<void>;
  disabled: boolean;
}

const ConfigurationPanel: React.FC<ConfigurationPanelProps> = ({ config, onSave, disabled }) => {
  const [formData, setFormData] = useState<ApiConfig>(config);
  const [showCanvasToken, setShowCanvasToken] = useState(false);
  const [showDiscourseKey, setShowDiscourseKey] = useState(false);
  const [isTestingCanvas, setIsTestingCanvas] = useState(false);
  const [isTestingDiscourse, setIsTestingDiscourse] = useState(false);
  const [canvasTestResult, setCanvasTestResult] = useState<{ success: boolean; message: string } | null>(null);
  const [discourseTestResult, setDiscourseTestResult] = useState<{ success: boolean; message: string } | null>(null);
  const [isSaving, setIsSaving] = useState(false);

  const handleInputChange = (section: 'canvas' | 'discourse', field: string, value: any) => {
    setFormData({
      ...formData,
      [section]: {
        ...formData[section],
        [field]: value
      }
    });
    
    // Clear test results when configuration changes
    if (section === 'canvas') {
      setCanvasTestResult(null);
    } else {
      setDiscourseTestResult(null);
    }
  };

  const handleTestCanvasConnection = async () => {
    try {
      setIsTestingCanvas(true);
      const result = await testCanvasConnection(formData.canvas);
      setCanvasTestResult({
        success: result.success,
        message: result.success ? 'Connection successful!' : `Connection failed: ${result.message}`
      });
    } catch (error) {
      setCanvasTestResult({
        success: false,
        message: 'Connection test failed due to an error'
      });
      console.error('Canvas connection test error:', error);
    } finally {
      setIsTestingCanvas(false);
    }
  };

  const handleTestDiscourseConnection = async () => {
    try {
      setIsTestingDiscourse(true);
      const result = await testDiscourseConnection(formData.discourse);
      setDiscourseTestResult({
        success: result.success,
        message: result.success ? 'Connection successful!' : `Connection failed: ${result.message}`
      });
    } catch (error) {
      setDiscourseTestResult({
        success: false,
        message: 'Connection test failed due to an error'
      });
      console.error('Discourse connection test error:', error);
    } finally {
      setIsTestingDiscourse(false);
    }
  };

  const handleSave = async () => {
    try {
      setIsSaving(true);
      await onSave(formData);
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <Paper sx={{ p: 3 }}>
      <Typography variant="h6" gutterBottom>
        API Configuration
      </Typography>
      
      <Box sx={{ mt: 3 }}>
        <Typography variant="subtitle1" gutterBottom>
          Canvas LMS Configuration
        </Typography>
        
        <Grid container spacing={3}>
          <Grid item xs={12}>
            <TextField
              label="Canvas API Base URL"
              fullWidth
              value={formData.canvas.base_url}
              onChange={(e) => handleInputChange('canvas', 'base_url', e.target.value)}
              disabled={disabled}
              helperText="Example: https://canvas.example.com/api/v1"
            />
          </Grid>
          
          <Grid item xs={12}>
            <TextField
              label="Canvas API Token"
              fullWidth
              type={showCanvasToken ? 'text' : 'password'}
              value={formData.canvas.api_token}
              onChange={(e) => handleInputChange('canvas', 'api_token', e.target.value)}
              disabled={disabled}
              InputProps={{
                endAdornment: (
                  <InputAdornment position="end">
                    <IconButton
                      onClick={() => setShowCanvasToken(!showCanvasToken)}
                      edge="end"
                    >
                      {showCanvasToken ? <VisibilityOffIcon /> : <VisibilityIcon />}
                    </IconButton>
                  </InputAdornment>
                )
              }}
            />
          </Grid>
          
          <Grid item xs={12} md={6}>
            <TextField
              label="Timeout (seconds)"
              type="number"
              fullWidth
              value={formData.canvas.timeout_seconds || 30}
              onChange={(e) => handleInputChange('canvas', 'timeout_seconds', parseInt(e.target.value))}
              disabled={disabled}
              inputProps={{ min: 1, max: 300 }}
            />
          </Grid>
          
          <Grid item xs={12} md={6}>
            <Button
              variant="outlined"
              onClick={handleTestCanvasConnection}
              disabled={disabled || isTestingCanvas || !formData.canvas.base_url || !formData.canvas.api_token}
              startIcon={isTestingCanvas ? <CircularProgress size={20} /> : <CheckIcon />}
              fullWidth
              sx={{ height: '56px' }}
            >
              {isTestingCanvas ? 'Testing...' : 'Test Connection'}
            </Button>
          </Grid>
          
          {canvasTestResult && (
            <Grid item xs={12}>
              <Alert severity={canvasTestResult.success ? 'success' : 'error'}>
                {canvasTestResult.message}
              </Alert>
            </Grid>
          )}
        </Grid>
      </Box>
      
      <Divider sx={{ my: 4 }} />
      
      <Box>
        <Typography variant="subtitle1" gutterBottom>
          Discourse Configuration
        </Typography>
        
        <Grid container spacing={3}>
          <Grid item xs={12}>
            <TextField
              label="Discourse Base URL"
              fullWidth
              value={formData.discourse.base_url}
              onChange={(e) => handleInputChange('discourse', 'base_url', e.target.value)}
              disabled={disabled}
              helperText="Example: https://discourse.example.com"
            />
          </Grid>
          
          <Grid item xs={12}>
            <TextField
              label="Discourse API Key"
              fullWidth
              type={showDiscourseKey ? 'text' : 'password'}
              value={formData.discourse.api_key}
              onChange={(e) => handleInputChange('discourse', 'api_key', e.target.value)}
              disabled={disabled}
              InputProps={{
                endAdornment: (
                  <InputAdornment position="end">
                    <IconButton
                      onClick={() => setShowDiscourseKey(!showDiscourseKey)}
                      edge="end"
                    >
                      {showDiscourseKey ? <VisibilityOffIcon /> : <VisibilityIcon />}
                    </IconButton>
                  </InputAdornment>
                )
              }}
            />
          </Grid>
          
          <Grid item xs={12} md={6}>
            <TextField
              label="API Username"
              fullWidth
              value={formData.discourse.api_username}
              onChange={(e) => handleInputChange('discourse', 'api_username', e.target.value)}
              disabled={disabled}
              helperText="Usually 'system' or an admin username"
            />
          </Grid>
          
          <Grid item xs={12} md={6}>
            <TextField
              label="Timeout (seconds)"
              type="number"
              fullWidth
              value={formData.discourse.timeout_seconds || 30}
              onChange={(e) => handleInputChange('discourse', 'timeout_seconds', parseInt(e.target.value))}
              disabled={disabled}
              inputProps={{ min: 1, max: 300 }}
            />
          </Grid>
          
          <Grid item xs={12} md={6}>
            <Button
              variant="outlined"
              onClick={handleTestDiscourseConnection}
              disabled={disabled || isTestingDiscourse || !formData.discourse.base_url || !formData.discourse.api_key || !formData.discourse.api_username}
              startIcon={isTestingDiscourse ? <CircularProgress size={20} /> : <CheckIcon />}
              fullWidth
              sx={{ height: '56px' }}
            >
              {isTestingDiscourse ? 'Testing...' : 'Test Connection'}
            </Button>
          </Grid>
          
          {discourseTestResult && (
            <Grid item xs={12}>
              <Alert severity={discourseTestResult.success ? 'success' : 'error'}>
                {discourseTestResult.message}
              </Alert>
            </Grid>
          )}
        </Grid>
      </Box>
      
      <Box sx={{ mt: 4, display: 'flex', justifyContent: 'flex-end' }}>
        <Button
          variant="contained"
          color="primary"
          startIcon={isSaving ? <CircularProgress size={20} color="inherit" /> : <SaveIcon />}
          onClick={handleSave}
          disabled={disabled || isSaving}
        >
          {isSaving ? 'Saving...' : 'Save Configuration'}
        </Button>
      </Box>
    </Paper>
  );
};

export default ConfigurationPanel;
