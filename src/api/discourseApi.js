module.exports = {
    getUserNotifications: async (discourseUserId) => {
        // ...stub implementation...
        return [
            {
                id: 'discourse1',
                createdAt: new Date().toISOString(),
                read: false,
                notificationType: 'discussion',
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
        return { id: 'discourse_created_id', ...notificationData };
    }
};