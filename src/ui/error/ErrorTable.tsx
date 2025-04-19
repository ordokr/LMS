import React, { useState } from 'react';
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  TablePagination,
  Paper,
  Chip,
  IconButton,
  Tooltip,
  Typography,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogContentText,
  DialogActions,
  Button,
  TextField
} from '@mui/material';
import {
  Refresh as RefreshIcon,
  Check as CheckIcon,
  Info as InfoIcon,
  Error as ErrorIcon,
  Warning as WarningIcon,
  Info as InfoOutlinedIcon
} from '@mui/icons-material';
import { ErrorRecord, ErrorSeverity } from '../../types/error';
import { resolveError, retryError } from '../../api/errorApi';

interface ErrorTableProps {
  errors: ErrorRecord[];
  onErrorSelected: (error: ErrorRecord) => void;
  onErrorResolved: () => void;
  onErrorRetried: () => void;
}

const ErrorTable: React.FC<ErrorTableProps> = ({ 
  errors, 
  onErrorSelected, 
  onErrorResolved,
  onErrorRetried
}) => {
  const [page, setPage] = useState(0);
  const [rowsPerPage, setRowsPerPage] = useState(10);
  const [resolveDialogOpen, setResolveDialogOpen] = useState(false);
  const [selectedError, setSelectedError] = useState<ErrorRecord | null>(null);
  const [resolution, setResolution] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleChangePage = (event: unknown, newPage: number) => {
    setPage(newPage);
  };

  const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
    setRowsPerPage(parseInt(event.target.value, 10));
    setPage(0);
  };

  const handleOpenResolveDialog = (error: ErrorRecord) => {
    setSelectedError(error);
    setResolveDialogOpen(true);
  };

  const handleCloseResolveDialog = () => {
    setResolveDialogOpen(false);
    setSelectedError(null);
    setResolution('');
  };

  const handleResolveError = async () => {
    if (!selectedError) return;
    
    try {
      setIsSubmitting(true);
      await resolveError(selectedError.id, resolution);
      handleCloseResolveDialog();
      onErrorResolved();
    } catch (err) {
      console.error('Failed to resolve error:', err);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleRetryError = async (error: ErrorRecord) => {
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
        return <ErrorIcon color="error" />;
      case ErrorSeverity.Error:
        return <ErrorIcon color="error" />;
      case ErrorSeverity.Warning:
        return <WarningIcon color="warning" />;
      case ErrorSeverity.Info:
        return <InfoOutlinedIcon color="info" />;
      default:
        return <InfoIcon />;
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
      <TableContainer component={Paper} variant="outlined">
        <Table aria-label="error table">
          <TableHead>
            <TableRow>
              <TableCell>Severity</TableCell>
              <TableCell>Message</TableCell>
              <TableCell>Category</TableCell>
              <TableCell>Source</TableCell>
              <TableCell>Timestamp</TableCell>
              <TableCell>Status</TableCell>
              <TableCell>Actions</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {errors.length === 0 ? (
              <TableRow>
                <TableCell colSpan={7} align="center">
                  <Typography variant="body1" color="text.secondary" sx={{ py: 2 }}>
                    No errors found
                  </Typography>
                </TableCell>
              </TableRow>
            ) : (
              errors
                .slice(page * rowsPerPage, page * rowsPerPage + rowsPerPage)
                .map((error) => (
                  <TableRow key={error.id} hover>
                    <TableCell>
                      <Tooltip title={error.severity}>
                        {getSeverityIcon(error.severity)}
                      </Tooltip>
                    </TableCell>
                    <TableCell>
                      <Typography
                        variant="body2"
                        sx={{
                          maxWidth: 300,
                          overflow: 'hidden',
                          textOverflow: 'ellipsis',
                          whiteSpace: 'nowrap',
                        }}
                      >
                        {error.message}
                      </Typography>
                    </TableCell>
                    <TableCell>{error.category}</TableCell>
                    <TableCell>{error.source}</TableCell>
                    <TableCell>
                      {new Date(error.timestamp).toLocaleString()}
                    </TableCell>
                    <TableCell>
                      <Chip
                        label={error.resolved ? 'Resolved' : 'Unresolved'}
                        color={error.resolved ? 'success' : 'error'}
                        size="small"
                      />
                    </TableCell>
                    <TableCell>
                      <Tooltip title="View Details">
                        <IconButton
                          size="small"
                          onClick={() => onErrorSelected(error)}
                        >
                          <InfoIcon />
                        </IconButton>
                      </Tooltip>
                      {!error.resolved && (
                        <>
                          <Tooltip title="Resolve">
                            <IconButton
                              size="small"
                              onClick={() => handleOpenResolveDialog(error)}
                            >
                              <CheckIcon />
                            </IconButton>
                          </Tooltip>
                          {error.retriable && error.retry_count < error.max_retries && (
                            <Tooltip title="Retry">
                              <IconButton
                                size="small"
                                onClick={() => handleRetryError(error)}
                              >
                                <RefreshIcon />
                              </IconButton>
                            </Tooltip>
                          )}
                        </>
                      )}
                    </TableCell>
                  </TableRow>
                ))
            )}
          </TableBody>
        </Table>
      </TableContainer>
      <TablePagination
        rowsPerPageOptions={[5, 10, 25, 50]}
        component="div"
        count={errors.length}
        rowsPerPage={rowsPerPage}
        page={page}
        onPageChange={handleChangePage}
        onRowsPerPageChange={handleChangeRowsPerPage}
      />

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

export default ErrorTable;
