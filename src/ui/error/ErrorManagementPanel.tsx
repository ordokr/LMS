import React, { useState, useEffect } from 'react';
import {
  Typography,
  Box,
  Paper,
  Tabs,
  Tab,
  Button,
  CircularProgress,
  Alert,
  Snackbar,
  Divider,
  Chip,
  IconButton,
  Tooltip,
  Grid
} from '@mui/material';
import {
  Refresh as RefreshIcon,
  Delete as DeleteIcon,
  FilterList as FilterIcon
} from '@mui/icons-material';
import { ErrorRecord, ErrorSeverity, ErrorCategory } from '../../types/error';
import { fetchAllErrors, fetchUnresolvedErrors, clearResolvedErrors } from '../../api/errorApi';
import ErrorTable from './ErrorTable';
import ErrorFilterPanel from './ErrorFilterPanel';
import ErrorDetailsPanel from './ErrorDetailsPanel';

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
      id={`error-tabpanel-${index}`}
      aria-labelledby={`error-tab-${index}`}
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

const ErrorManagementPanel: React.FC = () => {
  const [tabValue, setTabValue] = useState(0);
  const [errors, setErrors] = useState<ErrorRecord[]>([]);
  const [unresolvedErrors, setUnresolvedErrors] = useState<ErrorRecord[]>([]);
  const [selectedError, setSelectedError] = useState<ErrorRecord | null>(null);
  const [showFilterPanel, setShowFilterPanel] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  // Fetch errors on component mount
  useEffect(() => {
    fetchErrors();
  }, []);

  const fetchErrors = async () => {
    try {
      setIsLoading(true);
      const [allErrors, unresolved] = await Promise.all([
        fetchAllErrors(),
        fetchUnresolvedErrors()
      ]);
      setErrors(allErrors);
      setUnresolvedErrors(unresolved);
    } catch (err) {
      setError('Failed to fetch errors');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setTabValue(newValue);
  };

  const handleClearResolvedErrors = async () => {
    try {
      setIsLoading(true);
      const count = await clearResolvedErrors();
      setSuccess(`Cleared ${count} resolved errors`);
      fetchErrors();
    } catch (err) {
      setError('Failed to clear resolved errors');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleErrorSelected = (error: ErrorRecord) => {
    setSelectedError(error);
  };

  const handleErrorResolved = () => {
    fetchErrors();
    setSelectedError(null);
    setSuccess('Error resolved successfully');
  };

  const handleErrorRetried = () => {
    fetchErrors();
    setSuccess('Error retry initiated');
  };

  const handleCloseSnackbar = () => {
    setError(null);
    setSuccess(null);
  };

  const handleToggleFilterPanel = () => {
    setShowFilterPanel(!showFilterPanel);
  };

  const getErrorCounts = () => {
    const criticalCount = errors.filter(e => e.severity === ErrorSeverity.Critical && !e.resolved).length;
    const errorCount = errors.filter(e => e.severity === ErrorSeverity.Error && !e.resolved).length;
    const warningCount = errors.filter(e => e.severity === ErrorSeverity.Warning && !e.resolved).length;
    
    return { criticalCount, errorCount, warningCount };
  };

  const { criticalCount, errorCount, warningCount } = getErrorCounts();

  return (
    <Paper sx={{ p: 3 }}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h6">Error Management</Typography>
        <Box>
          <Tooltip title="Refresh">
            <IconButton onClick={fetchErrors} disabled={isLoading}>
              {isLoading ? <CircularProgress size={24} /> : <RefreshIcon />}
            </IconButton>
          </Tooltip>
          <Tooltip title="Filter">
            <IconButton onClick={handleToggleFilterPanel}>
              <FilterIcon />
            </IconButton>
          </Tooltip>
          <Tooltip title="Clear Resolved Errors">
            <IconButton onClick={handleClearResolvedErrors} disabled={isLoading}>
              <DeleteIcon />
            </IconButton>
          </Tooltip>
        </Box>
      </Box>

      <Grid container spacing={2} sx={{ mb: 3 }}>
        <Grid item xs={12} md={4}>
          <Paper variant="outlined" sx={{ p: 2, textAlign: 'center' }}>
            <Typography variant="subtitle2" color="error">Critical Errors</Typography>
            <Typography variant="h4" color="error">{criticalCount}</Typography>
          </Paper>
        </Grid>
        <Grid item xs={12} md={4}>
          <Paper variant="outlined" sx={{ p: 2, textAlign: 'center' }}>
            <Typography variant="subtitle2" color="warning.main">Errors</Typography>
            <Typography variant="h4" color="warning.main">{errorCount}</Typography>
          </Paper>
        </Grid>
        <Grid item xs={12} md={4}>
          <Paper variant="outlined" sx={{ p: 2, textAlign: 'center' }}>
            <Typography variant="subtitle2" color="info.main">Warnings</Typography>
            <Typography variant="h4" color="info.main">{warningCount}</Typography>
          </Paper>
        </Grid>
      </Grid>

      {showFilterPanel && (
        <Box sx={{ mb: 3 }}>
          <ErrorFilterPanel 
            onFilter={(filteredErrors) => setErrors(filteredErrors)} 
            onClose={() => setShowFilterPanel(false)}
          />
        </Box>
      )}

      <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
        <Tabs value={tabValue} onChange={handleTabChange}>
          <Tab label="All Errors" id="error-tab-0" />
          <Tab 
            label={
              <Box sx={{ display: 'flex', alignItems: 'center' }}>
                Unresolved
                <Chip 
                  label={unresolvedErrors.length} 
                  size="small" 
                  color="error" 
                  sx={{ ml: 1 }}
                />
              </Box>
            } 
            id="error-tab-1" 
          />
          {selectedError && <Tab label="Error Details" id="error-tab-2" />}
        </Tabs>
      </Box>

      <TabPanel value={tabValue} index={0}>
        <ErrorTable 
          errors={errors} 
          onErrorSelected={handleErrorSelected}
          onErrorResolved={handleErrorResolved}
          onErrorRetried={handleErrorRetried}
        />
      </TabPanel>

      <TabPanel value={tabValue} index={1}>
        <ErrorTable 
          errors={unresolvedErrors} 
          onErrorSelected={handleErrorSelected}
          onErrorResolved={handleErrorResolved}
          onErrorRetried={handleErrorRetried}
        />
      </TabPanel>

      {selectedError && (
        <TabPanel value={tabValue} index={2}>
          <ErrorDetailsPanel 
            error={selectedError} 
            onErrorResolved={handleErrorResolved}
            onErrorRetried={handleErrorRetried}
          />
        </TabPanel>
      )}

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
    </Paper>
  );
};

export default ErrorManagementPanel;
