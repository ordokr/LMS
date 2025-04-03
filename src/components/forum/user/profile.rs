// Add this to your UserProfile component where appropriate

// Inside your user profile navigation tabs/actions
<A
    href=format!("/user/{}/preferences", user_id)
    class="btn btn-outline-secondary ms-2"
>
    <i class="bi bi-gear me-1"></i>
    "Preferences"
</A>

// Inside your user profile navigation tabs/actions
<A
    href=format!("/user/{}/topics", user_id)
    class="btn btn-outline-secondary ms-2"
>
    <i class="bi bi-bookmark me-1"></i>
    "My Topics"
</A>