# API Integration Guide

## Date/Time Handling

All date/time values are stored internally as `DateTime<Utc>` objects from the `chrono` crate. This provides several advantages:

- Type safety for date/time operations
- Efficient database storage and querying
- Consistent formatting for API responses
- Timezone awareness

### Incoming API Data

When consuming external API data (e.g., from Canvas or Discourse), use the date conversion utilities:

```rust
use crate::utils::date_utils::parse_date_string;

// Example parsing from Canvas API
let due_date = parse_date_string(canvas_assignment["due_at"].as_str());

This utility handles multiple date formats that might be encountered in external APIs.

Outgoing API Data
When sending data through our API, dates are automatically serialized to RFC3339 format using serde's default DateTime serialization.

If you need custom formatting, use:

use crate::utils::date_utils::format_date;

// Format a date for custom output
let formatted_due_date = format_date(&assignment.due_date);

Database Storage
In database schemas, use the appropriate timestamp type for your database:

SQLite: TEXT (ISO8601 format)

Date/Time Best Practices
Always use UTC internally: Store all dates in UTC to avoid timezone confusion
Convert at boundaries: Convert to local time only at the presentation layer
Validate date constraints: Ensure logical date relationships (e.g., end dates after start dates)
Handle optional dates: Use Option<DateTime<Utc>> for optional dates
