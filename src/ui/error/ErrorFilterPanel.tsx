import React, { useState } from 'react';
import {
  Paper,
  Typography,
  Box,
  Grid,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  TextField,
  Button,
  FormControlLabel,
  Switch,
  SelectChangeEvent,
  IconButton
} from '@mui/material';
import { Close as CloseIcon } from '@mui/icons-material';
import { ErrorSeverity, ErrorCategory, ErrorFilter } from '../../types/error';
import { fetchErrorsByFilter } from '../../api/errorApi';

interface ErrorFilterPanelProps {
  onFilter: (errors: any[]) => void;
  onClose: () => void;
}

const ErrorFilterPanel: React.FC<ErrorFilterPanelProps> = ({ onFilter, onClose }) => {
  const [filter, setFilter] = useState<ErrorFilter>({});
  const [isLoading, setIsLoading] = useState(false);

  const handleSeverityChange = (event: SelectChangeEvent) => {
    const value = event.target.value;
    setFilter({
      ...filter,
      severity: value === 'all' ? undefined : value,
    });
  };

  const handleCategoryChange = (event: SelectChangeEvent) => {
    const value = event.target.value;
    setFilter({
      ...filter,
      category: value === 'all' ? undefined : value,
    });
  };

  const handleSourceChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;
    setFilter({
      ...filter,
      source: value || undefined,
    });
  };

  const handleEntityTypeChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;
    setFilter({
      ...filter,
      entity_type: value || undefined,
    });
  };

  const handleEntityIdChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;
    setFilter({
      ...filter,
      entity_id: value || undefined,
    });
  };

  const handleResolvedChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const checked = event.target.checked;
    setFilter({
      ...filter,
      resolved: checked,
    });
  };

  const handleRetriableChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const checked = event.target.checked;
    setFilter({
      ...filter,
      retriable: checked,
    });
  };

  const handleApplyFilter = async () => {
    try {
      setIsLoading(true);
      const errors = await fetchErrorsByFilter(filter);
      onFilter(errors);
    } catch (err) {
      console.error('Failed to apply filter:', err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleResetFilter = () => {
    setFilter({});
  };

  return (
    <Paper variant="outlined" sx={{ p: 2 }}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 2 }}>
        <Typography variant="subtitle1">Filter Errors</Typography>
        <IconButton size="small" onClick={onClose}>
          <CloseIcon />
        </IconButton>
      </Box>
      
      <Grid container spacing={2}>
        <Grid item xs={12} md={4}>
          <FormControl fullWidth size="small">
            <InputLabel id="severity-label">Severity</InputLabel>
            <Select
              labelId="severity-label"
              id="severity"
              value={filter.severity || 'all'}
              label="Severity"
              onChange={handleSeverityChange}
            >
              <MenuItem value="all">All Severities</MenuItem>
              <MenuItem value={ErrorSeverity.Critical}>Critical</MenuItem>
              <MenuItem value={ErrorSeverity.Error}>Error</MenuItem>
              <MenuItem value={ErrorSeverity.Warning}>Warning</MenuItem>
              <MenuItem value={ErrorSeverity.Info}>Info</MenuItem>
            </Select>
          </FormControl>
        </Grid>
        
        <Grid item xs={12} md={4}>
          <FormControl fullWidth size="small">
            <InputLabel id="category-label">Category</InputLabel>
            <Select
              labelId="category-label"
              id="category"
              value={filter.category || 'all'}
              label="Category"
              onChange={handleCategoryChange}
            >
              <MenuItem value="all">All Categories</MenuItem>
              <MenuItem value={ErrorCategory.ApiConnection}>API Connection</MenuItem>
              <MenuItem value={ErrorCategory.Authentication}>Authentication</MenuItem>
              <MenuItem value={ErrorCategory.Authorization}>Authorization</MenuItem>
              <MenuItem value={ErrorCategory.Validation}>Validation</MenuItem>
              <MenuItem value={ErrorCategory.Synchronization}>Synchronization</MenuItem>
              <MenuItem value={ErrorCategory.Database}>Database</MenuItem>
              <MenuItem value={ErrorCategory.Configuration}>Configuration</MenuItem>
              <MenuItem value={ErrorCategory.System}>System</MenuItem>
              <MenuItem value={ErrorCategory.Unknown}>Unknown</MenuItem>
            </Select>
          </FormControl>
        </Grid>
        
        <Grid item xs={12} md={4}>
          <TextField
            fullWidth
            size="small"
            label="Source"
            value={filter.source || ''}
            onChange={handleSourceChange}
          />
        </Grid>
        
        <Grid item xs={12} md={6}>
          <TextField
            fullWidth
            size="small"
            label="Entity Type"
            value={filter.entity_type || ''}
            onChange={handleEntityTypeChange}
          />
        </Grid>
        
        <Grid item xs={12} md={6}>
          <TextField
            fullWidth
            size="small"
            label="Entity ID"
            value={filter.entity_id || ''}
            onChange={handleEntityIdChange}
          />
        </Grid>
        
        <Grid item xs={12} md={6}>
          <FormControlLabel
            control={
              <Switch
                checked={filter.resolved === true}
                onChange={handleResolvedChange}
              />
            }
            label="Show Resolved Only"
          />
        </Grid>
        
        <Grid item xs={12} md={6}>
          <FormControlLabel
            control={
              <Switch
                checked={filter.retriable === true}
                onChange={handleRetriableChange}
              />
            }
            label="Show Retriable Only"
          />
        </Grid>
        
        <Grid item xs={12}>
          <Box sx={{ display: 'flex', justifyContent: 'flex-end', gap: 1 }}>
            <Button
              variant="outlined"
              onClick={handleResetFilter}
              disabled={isLoading}
            >
              Reset
            </Button>
            <Button
              variant="contained"
              onClick={handleApplyFilter}
              disabled={isLoading}
            >
              {isLoading ? 'Applying...' : 'Apply Filter'}
            </Button>
          </Box>
        </Grid>
      </Grid>
    </Paper>
  );
};

export default ErrorFilterPanel;
