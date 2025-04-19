import React, { useState } from 'react';
import {
  Typography,
  Box,
  Paper,
  Grid,
  Chip,
  Button,
  TextField,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogContentText,
  DialogActions,
  Divider,
  Card,
  CardContent,
  CardHeader
} from '@mui/material';
import {
  Refresh as RefreshIcon,
  Check as CheckIcon,
  Error as ErrorIcon,
  Warning as WarningIcon,
  Info as InfoOutlinedIcon
} from '@mui/icons-material';
import { ErrorRecord, ErrorSeverity } from '../../types/error';
import { resolveError, retryError } from '../../api/errorApi';

interface ErrorDetailsPanelProps {
  error: ErrorRecord;
  onErrorResolved: () => void;
  onErrorRetried: () => void;
}

const ErrorDetailsPanel: React.FC<ErrorDetailsPanelProps> = ({ 
  error, 
  onErrorResolved,
  onErrorRetried
}) => {
  const [resolveDialogOpen, setResolveDialogOpen] = useState(false);
  const [resolution, setResolution] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleOpenResolveDialog = () => {
    setResolveDialogOpen(true);
  };

  const handleCloseResolveDialog = () => {
    setResolveDialogOpen(false);
    setResolution('');
  };

  const handleResolveError = async () => {
    try {
      setIsSubmitting(true);
      await resolveError(error.id, resolution);
      handleCloseResolveDialog();
      onErrorResolved();
    } catch (err) {
      console.error('Failed to resolve error:', err);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleRetryError = async () => {
    try {
      await retryError(error.id);
      onErrorRetried();
    } catch (err) {
      console.error('Failed to retry error:', err);
    }
  };

  const getSeverityIcon = (severity: string) => {
    switch (severity) {
      case ErrorSeverity.Critical:
        return <ErrorIcon color="error" fontSize="large" />;
      case ErrorSeverity.Error:
        return <ErrorIcon color="error" fontSize="large" />;
      case ErrorSeverity.Warning:
        return <WarningIcon color="warning" fontSize="large" />;
      case ErrorSeverity.Info:
        return <InfoOutlinedIcon color="info" fontSize="large" />;
      default:
        return <InfoOutlinedIcon fontSize="large" />;
    }
  };

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case ErrorSeverity.Critical:
        return 'error';
      case ErrorSeverity.Error:
        return 'error';
      case ErrorSeverity.Warning:
        return 'warning';
      case ErrorSeverity.Info:
        return 'info';
      default:
        return 'default';
    }
  };

  return (
    <>
      <Paper variant="outlined" sx={{ p: 3 }}>
        <Box sx={{ display: 'flex', alignItems: 'center', mb: 3 }}>
          {getSeverityIcon(error.severity)}
          <Box sx={{ ml: 2 }}>
            <Typography variant="h6">{error.message}</Typography>
            <Typography variant="body2" color="text.secondary">
              {new Date(error.timestamp).toLocaleString()}
            </Typography>
          </Box>
        </Box>

        <Grid container spacing={3}>
          <Grid item xs={12} md={6}>
            <Card variant="outlined">
              <CardHeader title="Error Details" />
              <Divider />
              <CardContent>
                <Grid container spacing={2}>
                  <Grid item xs={4}>
                    <Typography variant="body2" color="text.secondary">
                      Severity
                    </Typography>
                  </Grid>
                  <Grid item xs={8}>
                    <Chip
                      label={error.severity}
                      color={getSeverityColor(error.severity) as any}
                      size="small"
                    />
                  </Grid>

                  <Grid item xs={4}>
                    <Typography variant="body2" color="text.secondary">
                      Category
                    </Typography>
                  </Grid>
                  <Grid item xs={8}>
                    <Typography variant="body2">{error.category}</Typography>
                  </Grid>

                  <Grid item xs={4}>
                    <Typography variant="body2" color="text.secondary">
                      Source
                    </Typography>
                  </Grid>
                  <Grid item xs={8}>
                    <Typography variant="body2">{error.source}</Typography>
                  </Grid>

                  <Grid item xs={4}>
                    <Typography variant="body2" color="text.secondary">
                      Status
                    </Typography>
                  </Grid>
                  <Grid item xs={8}>
                    <Chip
                      label={error.resolved ? 'Resolved' : 'Unresolved'}
                      color={error.resolved ? 'success' : 'error'}
                      size="small"
                    />
                  </Grid>

                  {error.entity_type && (
                    <>
                      <Grid item xs={4}>
                        <Typography variant="body2" color="text.secondary">
                          Entity Type
                        </Typography>
                      </Grid>
                      <Grid item xs={8}>
                        <Typography variant="body2">{error.entity_type}</Typography>
                      </Grid>
                    </>
                  )}

                  {error.entity_id && (
                    <>
                      <Grid item xs={4}>
                        <Typography variant="body2" color="text.secondary">
                          Entity ID
                        </Typography>
                      </Grid>
                      <Grid item xs={8}>
                        <Typography variant="body2">{error.entity_id}</Typography>
                      </Grid>
                    </>
                  )}
                </Grid>
              </CardContent>
            </Card>
          </Grid>

          <Grid item xs={12} md={6}>
            <Card variant="outlined">
              <CardHeader title="Recovery Information" />
              <Divider />
              <CardContent>
                <Grid container spacing={2}>
                  <Grid item xs={4}>
                    <Typography variant="body2" color="text.secondary">
                      Retriable
                    </Typography>
                  </Grid>
                  <Grid item xs={8}>
                    <Chip
                      label={error.retriable ? 'Yes' : 'No'}
                      color={error.retriable ? 'success' : 'default'}
                      size="small"
                    />
                  </Grid>

                  <Grid item xs={4}>
                    <Typography variant="body2" color="text.secondary">
                      Retry Count
                    </Typography>
                  </Grid>
                  <Grid item xs={8}>
                    <Typography variant="body2">
                      {error.retry_count} / {error.max_retries}
                    </Typography>
                  </Grid>

                  {error.next_retry && (
                    <>
                      <Grid item xs={4}>
                        <Typography variant="body2" color="text.secondary">
                          Next Retry
                        </Typography>
                      </Grid>
                      <Grid item xs={8}>
                        <Typography variant="body2">
                          {new Date(error.next_retry).toLocaleString()}
                        </Typography>
                      </Grid>
                    </>
                  )}

                  {error.resolved && (
                    <>
                      <Grid item xs={4}>
                        <Typography variant="body2" color="text.secondary">
                          Resolved At
                        </Typography>
                      </Grid>
                      <Grid item xs={8}>
                        <Typography variant="body2">
                          {error.resolved_at ? new Date(error.resolved_at).toLocaleString() : 'N/A'}
                        </Typography>
                      </Grid>

                      <Grid item xs={4}>
                        <Typography variant="body2" color="text.secondary">
                          Resolution
                        </Typography>
                      </Grid>
                      <Grid item xs={8}>
                        <Typography variant="body2">
                          {error.resolution || 'N/A'}
                        </Typography>
                      </Grid>
                    </>
                  )}
                </Grid>
              </CardContent>
            </Card>
          </Grid>

          {error.details && (
            <Grid item xs={12}>
              <Card variant="outlined">
                <CardHeader title="Error Details" />
                <Divider />
                <CardContent>
                  <Typography variant="body2" component="pre" sx={{ whiteSpace: 'pre-wrap' }}>
                    {error.details}
                  </Typography>
                </CardContent>
              </Card>
            </Grid>
          )}
        </Grid>

        {!error.resolved && (
          <Box sx={{ mt: 3, display: 'flex', justifyContent: 'flex-end', gap: 2 }}>
            {error.retriable && error.retry_count < error.max_retries && (
              <Button
                variant="outlined"
                startIcon={<RefreshIcon />}
                onClick={handleRetryError}
              >
                Retry
              </Button>
            )}
            <Button
              variant="contained"
              color="primary"
              startIcon={<CheckIcon />}
              onClick={handleOpenResolveDialog}
            >
              Resolve
            </Button>
          </Box>
        )}
      </Paper>

      <Dialog open={resolveDialogOpen} onClose={handleCloseResolveDialog}>
        <DialogTitle>Resolve Error</DialogTitle>
        <DialogContent>
          <DialogContentText>
            Please provide a resolution for this error:
          </DialogContentText>
          <TextField
            autoFocus
            margin="dense"
            id="resolution"
            label="Resolution"
            type="text"
            fullWidth
            multiline
            rows={4}
            value={resolution}
            onChange={(e) => setResolution(e.target.value)}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={handleCloseResolveDialog}>Cancel</Button>
          <Button 
            onClick={handleResolveError} 
            disabled={!resolution || isSubmitting}
            variant="contained"
          >
            {isSubmitting ? 'Resolving...' : 'Resolve'}
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
};

export default ErrorDetailsPanel;
