// or wherever your forum navigation component is located

// Add this navigation item to your forum navigation
<li class="nav-item">
    <a 
        class="nav-link" 
        href="/forum/tags/followed"
        class:active=move || current_path().starts_with("/forum/tags/followed")
    >
        <i class="bi bi-bookmark me-1"></i>
        "My Tags"
    </a>
</li>