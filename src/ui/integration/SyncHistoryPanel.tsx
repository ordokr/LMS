import React, { useState } from 'react';
import {
  Typography,
  Box,
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  TablePagination,
  Chip,
  IconButton,
  Collapse,
  List,
  ListItem,
  ListItemText,
  Divider
} from '@mui/material';
import {
  Check as CheckIcon,
  Error as ErrorIcon,
  KeyboardArrowDown as KeyboardArrowDownIcon,
  KeyboardArrowUp as KeyboardArrowUpIcon
} from '@mui/icons-material';
import { SyncResult } from '../../types/sync';

interface SyncHistoryPanelProps {
  history: SyncResult[];
}

interface ExpandableRowProps {
  result: SyncResult;
}

const ExpandableRow: React.FC<ExpandableRowProps> = ({ result }) => {
  const [open, setOpen] = useState(false);
  
  return (
    <>
      <TableRow sx={{ '& > *': { borderBottom: 'unset' } }}>
        <TableCell>
          <IconButton
            size="small"
            onClick={() => setOpen(!open)}
            disabled={result.errors.length === 0}
          >
            {open ? <KeyboardArrowUpIcon /> : <KeyboardArrowDownIcon />}
          </IconButton>
        </TableCell>
        <TableCell component="th" scope="row">
          {result.entity_type}
        </TableCell>
        <TableCell>
          {result.entity_id !== '00000000-0000-0000-0000-000000000000' 
            ? result.entity_id 
            : <Typography variant="body2" color="text.secondary">Bulk Sync</Typography>}
        </TableCell>
        <TableCell>
          {new Date(result.started_at).toLocaleString()}
        </TableCell>
        <TableCell>
          {Math.round((new Date(result.completed_at).getTime() - new Date(result.started_at).getTime()) / 1000)} seconds
        </TableCell>
        <TableCell>
          {result.canvas_updates}
        </TableCell>
        <TableCell>
          {result.discourse_updates}
        </TableCell>
        <TableCell>
          <Chip
            icon={result.status === 'Synced' ? <CheckIcon /> : <ErrorIcon />}
            label={result.status}
            color={result.status === 'Synced' ? 'success' : 'error'}
            size="small"
          />
        </TableCell>
      </TableRow>
      <TableRow>
        <TableCell style={{ paddingBottom: 0, paddingTop: 0 }} colSpan={8}>
          <Collapse in={open} timeout="auto" unmountOnExit>
            <Box sx={{ margin: 1 }}>
              <Typography variant="h6" gutterBottom component="div">
                Errors
              </Typography>
              <List dense>
                {result.errors.map((error, index) => (
                  <React.Fragment key={index}>
                    <ListItem>
                      <ListItemText primary={error} />
                    </ListItem>
                    {index < result.errors.length - 1 && <Divider />}
                  </React.Fragment>
                ))}
              </List>
            </Box>
          </Collapse>
        </TableCell>
      </TableRow>
    </>
  );
};

const SyncHistoryPanel: React.FC<SyncHistoryPanelProps> = ({ history }) => {
  const [page, setPage] = useState(0);
  const [rowsPerPage, setRowsPerPage] = useState(10);

  const handleChangePage = (event: unknown, newPage: number) => {
    setPage(newPage);
  };

  const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
    setRowsPerPage(parseInt(event.target.value, 10));
    setPage(0);
  };

  return (
    <Paper sx={{ p: 3 }}>
      <Typography variant="h6" gutterBottom>
        Synchronization History
      </Typography>
      
      {history.length === 0 ? (
        <Typography variant="body1" color="text.secondary" sx={{ mt: 2 }}>
          No synchronization history available.
        </Typography>
      ) : (
        <Box sx={{ mt: 2 }}>
          <TableContainer>
            <Table aria-label="sync history table">
              <TableHead>
                <TableRow>
                  <TableCell />
                  <TableCell>Entity Type</TableCell>
                  <TableCell>Entity ID</TableCell>
                  <TableCell>Started At</TableCell>
                  <TableCell>Duration</TableCell>
                  <TableCell>Canvas Updates</TableCell>
                  <TableCell>Discourse Updates</TableCell>
                  <TableCell>Status</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {history
                  .slice(page * rowsPerPage, page * rowsPerPage + rowsPerPage)
                  .map((result) => (
                    <ExpandableRow key={result.id} result={result} />
                  ))}
              </TableBody>
            </Table>
          </TableContainer>
          <TablePagination
            rowsPerPageOptions={[5, 10, 25]}
            component="div"
            count={history.length}
            rowsPerPage={rowsPerPage}
            page={page}
            onPageChange={handleChangePage}
            onRowsPerPageChange={handleChangeRowsPerPage}
          />
        </Box>
      )}
    </Paper>
  );
};

export default SyncHistoryPanel;
