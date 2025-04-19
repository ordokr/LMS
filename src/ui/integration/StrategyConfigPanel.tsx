import React, { useState, useEffect } from 'react';
import {
  Typography,
  Box,
  Paper,
  Grid,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  TextField,
  Button,
  Divider,
  FormControlLabel,
  Switch,
  SelectChangeEvent,
  Alert,
  CircularProgress,
  Collapse,
  IconButton
} from '@mui/material';
import {
  ExpandMore as ExpandMoreIcon,
  ExpandLess as ExpandLessIcon,
  Save as SaveIcon
} from '@mui/icons-material';
import { SyncDirection } from '../../types/sync';

interface StrategyConfigPanelProps {
  availableStrategies: string[];
  selectedStrategy: string;
  onStrategyChange: (strategy: string) => void;
  onStrategyConfigChange?: (config: any) => void;
  disabled: boolean;
}

const StrategyConfigPanel: React.FC<StrategyConfigPanelProps> = ({
  availableStrategies,
  selectedStrategy,
  onStrategyChange,
  onStrategyConfigChange,
  disabled
}) => {
  const [expanded, setExpanded] = useState(false);
  const [conflictResolution, setConflictResolution] = useState('mostRecent');
  const [incrementalSyncInterval, setIncrementalSyncInterval] = useState(60);
  const [isLoading, setIsLoading] = useState(false);

  const handleExpandClick = () => {
    setExpanded(!expanded);
  };

  const handleStrategyChange = (event: SelectChangeEvent) => {
    onStrategyChange(event.target.value);
  };

  const handleConflictResolutionChange = (event: SelectChangeEvent) => {
    setConflictResolution(event.target.value);
    
    if (onStrategyConfigChange) {
      onStrategyConfigChange({
        strategy: selectedStrategy,
        conflictResolution: event.target.value,
        incrementalSyncInterval
      });
    }
  };

  const handleIncrementalSyncIntervalChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = parseInt(event.target.value);
    if (!isNaN(value) && value > 0) {
      setIncrementalSyncInterval(value);
      
      if (onStrategyConfigChange) {
        onStrategyConfigChange({
          strategy: selectedStrategy,
          conflictResolution,
          incrementalSyncInterval: value
        });
      }
    }
  };

  const handleSaveConfig = () => {
    if (onStrategyConfigChange) {
      setIsLoading(true);
      
      // Simulate API call
      setTimeout(() => {
        onStrategyConfigChange({
          strategy: selectedStrategy,
          conflictResolution,
          incrementalSyncInterval
        });
        setIsLoading(false);
      }, 500);
    }
  };

  return (
    <Box>
      <Box sx={{ display: 'flex', alignItems: 'center', cursor: 'pointer' }} onClick={handleExpandClick}>
        <Typography variant="subtitle1">
          Advanced Synchronization Settings
        </Typography>
        <IconButton size="small">
          {expanded ? <ExpandLessIcon /> : <ExpandMoreIcon />}
        </IconButton>
      </Box>
      
      <Collapse in={expanded}>
        <Paper variant="outlined" sx={{ p: 2, mt: 2 }}>
          <Grid container spacing={2}>
            <Grid item xs={12} md={6}>
              <FormControl fullWidth>
                <InputLabel id="strategy-select-label">Synchronization Strategy</InputLabel>
                <Select
                  labelId="strategy-select-label"
                  id="strategy-select"
                  value={selectedStrategy}
                  label="Synchronization Strategy"
                  onChange={handleStrategyChange}
                  disabled={disabled}
                >
                  {availableStrategies.map((strategy) => (
                    <MenuItem key={strategy} value={strategy}>
                      {strategy.charAt(0).toUpperCase() + strategy.slice(1)}
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>
            
            {selectedStrategy === 'bidirectional' && (
              <Grid item xs={12} md={6}>
                <FormControl fullWidth>
                  <InputLabel id="conflict-resolution-label">Conflict Resolution</InputLabel>
                  <Select
                    labelId="conflict-resolution-label"
                    id="conflict-resolution"
                    value={conflictResolution}
                    label="Conflict Resolution"
                    onChange={handleConflictResolutionChange}
                    disabled={disabled}
                  >
                    <MenuItem value="sourceWins">Source Wins (Canvas)</MenuItem>
                    <MenuItem value="targetWins">Target Wins (Discourse)</MenuItem>
                    <MenuItem value="mostRecent">Most Recent Wins</MenuItem>
                    <MenuItem value="merge">Merge Changes</MenuItem>
                    <MenuItem value="manual">Manual Resolution</MenuItem>
                  </Select>
                </FormControl>
              </Grid>
            )}
            
            {selectedStrategy === 'incremental' && (
              <Grid item xs={12} md={6}>
                <TextField
                  label="Sync Interval (minutes)"
                  type="number"
                  fullWidth
                  value={incrementalSyncInterval}
                  onChange={handleIncrementalSyncIntervalChange}
                  disabled={disabled}
                  inputProps={{ min: 1, max: 1440 }}
                  helperText="How often to check for changes"
                />
              </Grid>
            )}
            
            <Grid item xs={12}>
              <Divider sx={{ my: 2 }} />
              
              <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <Typography variant="body2" color="text.secondary">
                  {selectedStrategy === 'basic' && 'Basic strategy performs a simple one-way sync without tracking changes.'}
                  {selectedStrategy === 'incremental' && 'Incremental strategy only syncs changes since the last sync.'}
                  {selectedStrategy === 'bidirectional' && 'Bidirectional strategy syncs in both directions with conflict resolution.'}
                </Typography>
                
                <Button
                  variant="contained"
                  color="primary"
                  startIcon={isLoading ? <CircularProgress size={20} color="inherit" /> : <SaveIcon />}
                  onClick={handleSaveConfig}
                  disabled={disabled || isLoading}
                >
                  Save Configuration
                </Button>
              </Box>
            </Grid>
          </Grid>
        </Paper>
      </Collapse>
    </Box>
  );
};

export default StrategyConfigPanel;
