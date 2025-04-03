// Add these imports at the top
use crate::components::forum::topics::bookmark_button::BookmarkButton;
use crate::components::forum::topics::subscription_button::SubscriptionButton;

// Within your topic actions section in the topic_detail component
<div class="topic-actions d-flex justify-content-between">
    <div class="d-flex gap-2">
        <button class="btn btn-sm btn-primary">
            <i class="bi bi-reply me-1"></i>"Reply"
        </button>
        <BookmarkButton topic_id=topic.id />
        <SubscriptionButton topic_id=topic.id />
    </div>
    // Other actions...
</div>