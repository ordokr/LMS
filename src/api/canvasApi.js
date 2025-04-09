module.exports = {
    getUserNotifications: async (canvasUserId) => {
        // ...stub implementation...
        return [
            {
                id: 'canvas1',
                createdAt: new Date().toISOString(),
                read: false,
                notificationType: 'assignment',
                // ...other fields...
            }
        ];
    },
    markNotificationAsRead: async (notificationId) => {
        // ...stub implementation...
        return { id: notificationId, read: true };
    },
    createNotification: async (notificationData) => {
        // ...stub implementation...
        return { id: 'canvas_created_id', ...notificationData };
    }
};