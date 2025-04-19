import React, { useState, useEffect } from 'react';
import {
  Container,
  Typography,
  Box,
  Paper,
  Grid,
  Button,
  CircularProgress,
  LinearProgress,
  Card,
  CardContent,
  CardHeader,
  Tabs,
  Tab,
  Divider,
  Alert,
  Snackbar,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  SelectChangeEvent
} from '@mui/material';
import {
  Sync as SyncIcon,
  Settings as SettingsIcon,
  History as HistoryIcon,
  Cancel as CancelIcon,
  Check as CheckIcon,
  Error as ErrorIcon
} from '@mui/icons-material';
import { SyncState, SyncDirection, SyncResult } from '../../types/sync';
import { ApiConfig } from '../../types/config';
import { fetchSyncState, startSync, cancelSync, syncEntity, getAvailableStrategies } from '../../api/syncApi';
import { fetchApiConfig, updateApiConfig } from '../../api/configApi';
import ConfigurationPanel from './ConfigurationPanel';
import SyncHistoryPanel from './SyncHistoryPanel';
import EntitySyncPanel from './EntitySyncPanel';
import StrategyConfigPanel from './StrategyConfigPanel';

interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`integration-tabpanel-${index}`}
      aria-labelledby={`integration-tab-${index}`}
      {...other}
    >
      {value === index && (
        <Box sx={{ p: 3 }}>
          {children}
        </Box>
      )}
    </div>
  );
}

const IntegrationDashboard: React.FC = () => {
  const [tabValue, setTabValue] = useState(0);
  const [syncState, setSyncState] = useState<SyncState | null>(null);
  const [apiConfig, setApiConfig] = useState<ApiConfig | null>(null);
  const [syncHistory, setSyncHistory] = useState<SyncResult[]>([]);
  const [syncDirection, setSyncDirection] = useState<SyncDirection>(SyncDirection.Bidirectional);
  const [availableStrategies, setAvailableStrategies] = useState<string[]>([]);
  const [selectedStrategy, setSelectedStrategy] = useState<string>('basic');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [pollingInterval, setPollingInterval] = useState<NodeJS.Timeout | null>(null);

  // Fetch initial data
  useEffect(() => {
    const fetchData = async () => {
      try {
        setIsLoading(true);
        const [stateResponse, configResponse, strategiesResponse] = await Promise.all([
          fetchSyncState(),
          fetchApiConfig(),
          getAvailableStrategies()
        ]);
        setSyncState(stateResponse);
        setApiConfig(configResponse);
        setAvailableStrategies(strategiesResponse);

        // Set default strategy if available
        if (strategiesResponse.length > 0 && !strategiesResponse.includes(selectedStrategy)) {
          setSelectedStrategy(strategiesResponse[0]);
        }
      } catch (err) {
        setError('Failed to load integration data');
        console.error(err);
      } finally {
        setIsLoading(false);
      }
    };

    fetchData();
  }, []);

  // Set up polling for sync state when syncing is active
  useEffect(() => {
    if (syncState?.is_syncing && !pollingInterval) {
      const interval = setInterval(async () => {
        try {
          const stateResponse = await fetchSyncState();
          setSyncState(stateResponse);

          // If sync has completed, stop polling
          if (!stateResponse.is_syncing) {
            setSuccess('Synchronization completed successfully');
            if (pollingInterval) {
              clearInterval(pollingInterval);
              setPollingInterval(null);
            }
          }
        } catch (err) {
          console.error('Failed to poll sync state:', err);
        }
      }, 2000); // Poll every 2 seconds

      setPollingInterval(interval);
    } else if (!syncState?.is_syncing && pollingInterval) {
      clearInterval(pollingInterval);
      setPollingInterval(null);
    }

    // Cleanup on unmount
    return () => {
      if (pollingInterval) {
        clearInterval(pollingInterval);
      }
    };
  }, [syncState?.is_syncing, pollingInterval]);

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setTabValue(newValue);
  };

  const handleSyncDirectionChange = (event: SelectChangeEvent) => {
    setSyncDirection(event.target.value as SyncDirection);
  };

  const handleStrategyChange = (event: SelectChangeEvent) => {
    setSelectedStrategy(event.target.value);
  };

  const handleStrategyConfigChange = (config: any) => {
    console.log('Strategy configuration changed:', config);
    setSuccess('Strategy configuration updated successfully');

    // In a real implementation, we would save the configuration to the server
    // For now, we just update the selected strategy
    if (config.strategy !== selectedStrategy) {
      setSelectedStrategy(config.strategy);
    }
  };

  const handleStartSync = async () => {
    try {
      setIsLoading(true);
      const response = await startSync(syncDirection, selectedStrategy);
      setSyncState(response);
      setSuccess('Synchronization started successfully');
    } catch (err) {
      setError('Failed to start synchronization');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCancelSync = async () => {
    try {
      setIsLoading(true);
      const response = await cancelSync();
      setSyncState(response);
      setSuccess('Synchronization cancelled successfully');
    } catch (err) {
      setError('Failed to cancel synchronization');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleConfigUpdate = async (newConfig: ApiConfig) => {
    try {
      setIsLoading(true);
      await updateApiConfig(newConfig);
      setApiConfig(newConfig);
      setSuccess('Configuration updated successfully');
    } catch (err) {
      setError('Failed to update configuration');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSyncEntity = async (entityType: string, entityId: string, direction: SyncDirection, strategy?: string) => {
    try {
      setIsLoading(true);
      const result = await syncEntity(entityType, entityId, direction, strategy || selectedStrategy);
      setSyncHistory([result, ...syncHistory]);
      setSuccess(`Entity ${entityType}:${entityId} synchronized successfully`);
    } catch (err) {
      setError(`Failed to synchronize entity ${entityType}:${entityId}`);
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCloseSnackbar = () => {
    setError(null);
    setSuccess(null);
  };

  return (
    <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
      <Typography variant="h4" component="h1" gutterBottom>
        Canvas-Discourse Integration
      </Typography>

      <Box sx={{ borderBottom: 1, borderColor: 'divider', mb: 3 }}>
        <Tabs value={tabValue} onChange={handleTabChange} aria-label="integration tabs">
          <Tab icon={<SyncIcon />} label="Synchronization" id="integration-tab-0" />
          <Tab icon={<SettingsIcon />} label="Configuration" id="integration-tab-1" />
          <Tab icon={<HistoryIcon />} label="History" id="integration-tab-2" />
        </Tabs>
      </Box>

      <TabPanel value={tabValue} index={0}>
        <Grid container spacing={3}>
          <Grid item xs={12}>
            <Paper sx={{ p: 3 }}>
              <Typography variant="h6" gutterBottom>
                Synchronization Status
              </Typography>

              {isLoading && !syncState ? (
                <Box sx={{ display: 'flex', justifyContent: 'center', my: 4 }}>
                  <CircularProgress />
                </Box>
              ) : syncState ? (
                <>
                  <Box sx={{ mb: 3 }}>
                    <Grid container spacing={2} alignItems="center">
                      <Grid item xs={12} md={6}>
                        <Typography variant="body1">
                          Status: <strong>{syncState.is_syncing ? 'Syncing' : 'Idle'}</strong>
                        </Typography>
                        {syncState.current_sync_stage && (
                          <Typography variant="body2" color="text.secondary">
                            Current Stage: {syncState.current_sync_stage}
                          </Typography>
                        )}
                        {syncState.last_sync && (
                          <Typography variant="body2" color="text.secondary">
                            Last Sync: {new Date(syncState.last_sync).toLocaleString()}
                          </Typography>
                        )}
                      </Grid>
                      <Grid item xs={12} md={6}>
                        {syncState.is_syncing && (
                          <Box sx={{ width: '100%' }}>
                            <LinearProgress
                              variant="determinate"
                              value={syncState.current_sync_progress * 100}
                              sx={{ mb: 1 }}
                            />
                            <Typography variant="body2" color="text.secondary" align="right">
                              {Math.round(syncState.current_sync_progress * 100)}%
                            </Typography>
                          </Box>
                        )}
                      </Grid>
                    </Grid>
                  </Box>

                  <Divider sx={{ my: 2 }} />

                  <Box sx={{ mt: 3 }}>
                    <Typography variant="subtitle1" gutterBottom>
                      Start New Synchronization
                    </Typography>

                    <Grid container spacing={2} alignItems="center">
                      <Grid item xs={12} md={4}>
                        <FormControl fullWidth>
                          <InputLabel id="sync-direction-label">Sync Direction</InputLabel>
                          <Select
                            labelId="sync-direction-label"
                            id="sync-direction"
                            value={syncDirection}
                            label="Sync Direction"
                            onChange={handleSyncDirectionChange}
                            disabled={syncState.is_syncing || isLoading}
                          >
                            <MenuItem value={SyncDirection.CanvasToDiscourse}>Canvas to Discourse</MenuItem>
                            <MenuItem value={SyncDirection.DiscourseToCanvas}>Discourse to Canvas</MenuItem>
                            <MenuItem value={SyncDirection.Bidirectional}>Bidirectional</MenuItem>
                          </Select>
                        </FormControl>
                      </Grid>
                      <Grid item xs={12} md={4}>
                        <FormControl fullWidth>
                          <InputLabel id="sync-strategy-label">Sync Strategy</InputLabel>
                          <Select
                            labelId="sync-strategy-label"
                            id="sync-strategy"
                            value={selectedStrategy}
                            label="Sync Strategy"
                            onChange={handleStrategyChange}
                            disabled={syncState.is_syncing || isLoading}
                          >
                            {availableStrategies.map((strategy) => (
                              <MenuItem key={strategy} value={strategy}>
                                {strategy.charAt(0).toUpperCase() + strategy.slice(1)}
                              </MenuItem>
                            ))}
                          </Select>
                        </FormControl>
                      </Grid>
                      <Grid item xs={12} md={4}>
                        {syncState.is_syncing ? (
                          <Button
                            variant="contained"
                            color="secondary"
                            startIcon={<CancelIcon />}
                            onClick={handleCancelSync}
                            disabled={isLoading}
                            fullWidth
                          >
                            Cancel Synchronization
                          </Button>
                        ) : (
                          <Button
                            variant="contained"
                            color="primary"
                            startIcon={<SyncIcon />}
                            onClick={handleStartSync}
                            disabled={isLoading}
                            fullWidth
                          >
                            Start Synchronization
                          </Button>
                        )}
                      </Grid>
                    </Grid>
                  </Box>

                  <Box sx={{ mt: 4 }}>
                    <StrategyConfigPanel
                      availableStrategies={availableStrategies}
                      selectedStrategy={selectedStrategy}
                      onStrategyChange={setSelectedStrategy}
                      onStrategyConfigChange={handleStrategyConfigChange}
                      disabled={syncState.is_syncing || isLoading}
                    />
                  </Box>

                  <Box sx={{ mt: 4 }}>
                    <EntitySyncPanel
                      onSyncEntity={handleSyncEntity}
                      disabled={syncState.is_syncing || isLoading}
                      availableStrategies={availableStrategies}
                      selectedStrategy={selectedStrategy}
                    />
                  </Box>
                </>
              ) : (
                <Alert severity="error">Failed to load synchronization state</Alert>
              )}
            </Paper>
          </Grid>

          {syncState?.is_syncing && syncState.current_sync_results && syncState.current_sync_results.length > 0 && (
            <Grid item xs={12}>
              <Paper sx={{ p: 3 }}>
                <Typography variant="h6" gutterBottom>
                  Current Sync Results
                </Typography>

                <Grid container spacing={2}>
                  {syncState.current_sync_results.map((result, index) => (
                    <Grid item xs={12} md={6} key={index}>
                      <Card variant="outlined">
                        <CardHeader
                          title={result.entity_type}
                          subheader={`${result.canvas_updates} Canvas updates, ${result.discourse_updates} Discourse updates`}
                          avatar={
                            result.status === 'Synced' ? (
                              <CheckIcon color="success" />
                            ) : (
                              <ErrorIcon color="error" />
                            )
                          }
                        />
                        {result.errors.length > 0 && (
                          <CardContent>
                            <Typography variant="body2" color="error">
                              {result.errors.length} errors occurred
                            </Typography>
                          </CardContent>
                        )}
                      </Card>
                    </Grid>
                  ))}
                </Grid>
              </Paper>
            </Grid>
          )}
        </Grid>
      </TabPanel>

      <TabPanel value={tabValue} index={1}>
        {isLoading && !apiConfig ? (
          <Box sx={{ display: 'flex', justifyContent: 'center', my: 4 }}>
            <CircularProgress />
          </Box>
        ) : apiConfig ? (
          <ConfigurationPanel
            config={apiConfig}
            onSave={handleConfigUpdate}
            disabled={isLoading || (syncState?.is_syncing || false)}
          />
        ) : (
          <Alert severity="error">Failed to load configuration</Alert>
        )}
      </TabPanel>

      <TabPanel value={tabValue} index={2}>
        <SyncHistoryPanel history={syncHistory} />
      </TabPanel>

      <Snackbar open={!!error} autoHideDuration={6000} onClose={handleCloseSnackbar}>
        <Alert onClose={handleCloseSnackbar} severity="error" sx={{ width: '100%' }}>
          {error}
        </Alert>
      </Snackbar>

      <Snackbar open={!!success} autoHideDuration={6000} onClose={handleCloseSnackbar}>
        <Alert onClose={handleCloseSnackbar} severity="success" sx={{ width: '100%' }}>
          {success}
        </Alert>
      </Snackbar>
    </Container>
  );
};

export default IntegrationDashboard;
