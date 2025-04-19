import React, { useState } from 'react';
import {
  Typography,
  Box,
  TextField,
  Button,
  Grid,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Paper,
  Collapse,
  IconButton,
  SelectChangeEvent
} from '@mui/material';
import {
  ExpandMore as ExpandMoreIcon,
  ExpandLess as ExpandLessIcon,
  Sync as SyncIcon
} from '@mui/icons-material';
import { SyncDirection } from '../../types/sync';

interface EntitySyncPanelProps {
  onSyncEntity: (entityType: string, entityId: string, direction: SyncDirection, strategy?: string) => Promise<void>;
  disabled: boolean;
  availableStrategies?: string[];
  selectedStrategy?: string;
}

const EntitySyncPanel: React.FC<EntitySyncPanelProps> = ({
  onSyncEntity,
  disabled,
  availableStrategies = [],
  selectedStrategy = 'basic'
}) => {
  const [expanded, setExpanded] = useState(false);
  const [entityType, setEntityType] = useState('');
  const [entityId, setEntityId] = useState('');
  const [syncDirection, setSyncDirection] = useState<SyncDirection>(SyncDirection.Bidirectional);
  const [syncStrategy, setSyncStrategy] = useState(selectedStrategy);
  const [isLoading, setIsLoading] = useState(false);

  const handleExpandClick = () => {
    setExpanded(!expanded);
  };

  const handleEntityTypeChange = (event: SelectChangeEvent) => {
    setEntityType(event.target.value);
  };

  const handleEntityIdChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setEntityId(event.target.value);
  };

  const handleSyncDirectionChange = (event: SelectChangeEvent) => {
    setSyncDirection(event.target.value as SyncDirection);
  };

  const handleSyncStrategyChange = (event: SelectChangeEvent) => {
    setSyncStrategy(event.target.value);
  };

  const handleSyncEntity = async () => {
    if (!entityType || !entityId) return;

    try {
      setIsLoading(true);
      await onSyncEntity(entityType, entityId, syncDirection, syncStrategy);
      // Reset form after successful sync
      setEntityId('');
    } catch (error) {
      console.error('Entity sync error:', error);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <Box>
      <Box sx={{ display: 'flex', alignItems: 'center', cursor: 'pointer' }} onClick={handleExpandClick}>
        <Typography variant="subtitle1">
          Synchronize Specific Entity
        </Typography>
        <IconButton size="small">
          {expanded ? <ExpandLessIcon /> : <ExpandMoreIcon />}
        </IconButton>
      </Box>

      <Collapse in={expanded}>
        <Paper variant="outlined" sx={{ p: 2, mt: 2 }}>
          <Grid container spacing={2}>
            <Grid item xs={12} md={3}>
              <FormControl fullWidth>
                <InputLabel id="entity-type-label">Entity Type</InputLabel>
                <Select
                  labelId="entity-type-label"
                  id="entity-type"
                  value={entityType}
                  label="Entity Type"
                  onChange={handleEntityTypeChange}
                  disabled={disabled || isLoading}
                >
                  <MenuItem value="user">User</MenuItem>
                  <MenuItem value="course">Course</MenuItem>
                  <MenuItem value="discussion">Discussion</MenuItem>
                  <MenuItem value="comment">Comment</MenuItem>
                  <MenuItem value="tag">Tag</MenuItem>
                  <MenuItem value="assignment">Assignment</MenuItem>
                  <MenuItem value="submission">Submission</MenuItem>
                  <MenuItem value="group">Group</MenuItem>
                  <MenuItem value="page">Page</MenuItem>
                  <MenuItem value="file">File</MenuItem>
                  <MenuItem value="announcement">Announcement</MenuItem>
                </Select>
              </FormControl>
            </Grid>

            <Grid item xs={12} md={3}>
              <TextField
                label="Entity ID"
                fullWidth
                value={entityId}
                onChange={handleEntityIdChange}
                disabled={disabled || isLoading || !entityType}
                placeholder={entityType ? `Enter ${entityType} ID` : 'Select entity type first'}
              />
            </Grid>

            <Grid item xs={12} md={3}>
              <FormControl fullWidth>
                <InputLabel id="entity-sync-direction-label">Sync Direction</InputLabel>
                <Select
                  labelId="entity-sync-direction-label"
                  id="entity-sync-direction"
                  value={syncDirection}
                  label="Sync Direction"
                  onChange={handleSyncDirectionChange}
                  disabled={disabled || isLoading}
                >
                  <MenuItem value={SyncDirection.CanvasToDiscourse}>Canvas to Discourse</MenuItem>
                  <MenuItem value={SyncDirection.DiscourseToCanvas}>Discourse to Canvas</MenuItem>
                  <MenuItem value={SyncDirection.Bidirectional}>Bidirectional</MenuItem>
                </Select>
              </FormControl>
            </Grid>

            <Grid item xs={12} md={3}>
              <FormControl fullWidth>
                <InputLabel id="entity-sync-strategy-label">Sync Strategy</InputLabel>
                <Select
                  labelId="entity-sync-strategy-label"
                  id="entity-sync-strategy"
                  value={syncStrategy}
                  label="Sync Strategy"
                  onChange={handleSyncStrategyChange}
                  disabled={disabled || isLoading}
                >
                  {availableStrategies.length > 0 ? (
                    availableStrategies.map((strategy) => (
                      <MenuItem key={strategy} value={strategy}>
                        {strategy.charAt(0).toUpperCase() + strategy.slice(1)}
                      </MenuItem>
                    ))
                  ) : (
                    <MenuItem value="basic">Basic</MenuItem>
                  )}
                </Select>
              </FormControl>
            </Grid>

            <Grid item xs={12}>
              <Button
                variant="contained"
                color="primary"
                startIcon={<SyncIcon />}
                onClick={handleSyncEntity}
                disabled={disabled || isLoading || !entityType || !entityId}
                fullWidth
              >
                Synchronize Entity
              </Button>
            </Grid>
          </Grid>
        </Paper>
      </Collapse>
    </Box>
  );
};

export default EntitySyncPanel;
