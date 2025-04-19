import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import {
  AppBar,
  Toolbar,
  Typography,
  Container,
  Box,
  Drawer,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  Divider
} from '@mui/material';
import {
  Dashboard as DashboardIcon,
  School as SchoolIcon,
  Forum as ForumIcon,
  Settings as SettingsIcon,
  Sync as SyncIcon,
  Error as ErrorIcon
} from '@mui/icons-material';
import IntegrationDashboard from './integration/IntegrationDashboard';
import ErrorManagementPanel from './error/ErrorManagementPanel';

// Create a theme
const theme = createTheme({
  palette: {
    primary: {
      main: '#1976d2',
    },
    secondary: {
      main: '#dc004e',
    },
  },
});

// Drawer width
const drawerWidth = 240;

const App: React.FC = () => {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Router>
        <Box sx={{ display: 'flex' }}>
          {/* App Bar */}
          <AppBar position="fixed" sx={{ zIndex: (theme) => theme.zIndex.drawer + 1 }}>
            <Toolbar>
              <Typography variant="h6" noWrap component="div">
                Canvas-Discourse Integration
              </Typography>
            </Toolbar>
          </AppBar>

          {/* Drawer */}
          <Drawer
            variant="permanent"
            sx={{
              width: drawerWidth,
              flexShrink: 0,
              [`& .MuiDrawer-paper`]: { width: drawerWidth, boxSizing: 'border-box' },
            }}
          >
            <Toolbar />
            <Box sx={{ overflow: 'auto' }}>
              <List>
                <ListItem button component="a" href="/">
                  <ListItemIcon>
                    <DashboardIcon />
                  </ListItemIcon>
                  <ListItemText primary="Dashboard" />
                </ListItem>
                <ListItem button component="a" href="/courses">
                  <ListItemIcon>
                    <SchoolIcon />
                  </ListItemIcon>
                  <ListItemText primary="Courses" />
                </ListItem>
                <ListItem button component="a" href="/discussions">
                  <ListItemIcon>
                    <ForumIcon />
                  </ListItemIcon>
                  <ListItemText primary="Discussions" />
                </ListItem>
              </List>
              <Divider />
              <List>
                <ListItem button component="a" href="/integration">
                  <ListItemIcon>
                    <SyncIcon />
                  </ListItemIcon>
                  <ListItemText primary="Integration" />
                </ListItem>
                <ListItem button component="a" href="/errors">
                  <ListItemIcon>
                    <ErrorIcon />
                  </ListItemIcon>
                  <ListItemText primary="Error Management" />
                </ListItem>
                <ListItem button component="a" href="/settings">
                  <ListItemIcon>
                    <SettingsIcon />
                  </ListItemIcon>
                  <ListItemText primary="Settings" />
                </ListItem>
              </List>
            </Box>
          </Drawer>

          {/* Main Content */}
          <Box component="main" sx={{ flexGrow: 1, p: 3 }}>
            <Toolbar />
            <Routes>
              <Route path="/" element={<Typography variant="h4">Dashboard</Typography>} />
              <Route path="/courses" element={<Typography variant="h4">Courses</Typography>} />
              <Route path="/discussions" element={<Typography variant="h4">Discussions</Typography>} />
              <Route path="/integration" element={<IntegrationDashboard />} />
              <Route path="/errors" element={<ErrorManagementPanel />} />
              <Route path="/settings" element={<Typography variant="h4">Settings</Typography>} />
            </Routes>
          </Box>
        </Box>
      </Router>
    </ThemeProvider>
  );
};

export default App;
