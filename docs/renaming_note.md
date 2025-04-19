# Renaming Note: educonnect → Ordo LMS and Quenti → Ordo Quiz

This document records the changes made to rename references from "educonnect" to "Ordo LMS" and from "Quenti" to "Ordo Quiz" throughout the codebase.

## Database References

- Changed database name from `educonnect.db` to `ordo.db` in multiple files
- Updated database connection strings from `sqlite:educonnect.db` to `sqlite:ordo.db`
- Updated default database paths

## Application Configuration

- Updated product name from "EduConnect Forum" to "Ordo LMS" in Tauri configuration
- Updated bundle identifier from "com.educonnect.forum" to "com.ordo.lms"
- Updated application descriptions to reflect Ordo LMS branding

## Launcher Components

- Created new `ordo_quiz_launcher.rs` module for launching the Ordo Quiz standalone app
- Updated the existing `quenti_launcher.rs` module to reference "Ordo Quiz" instead of "Quenti"
- Created new `ordo_quiz_commands.rs` module for Tauri commands
- Updated the existing `quenti_commands.rs` module to reference "Ordo Quiz" instead of "Quenti"
- Updated module exports in `mod.rs` files to include the new modules

## Launch Scripts

- Updated references in launch scripts to use "Ordo Quiz" instead of "Quenti"
- Added compatibility notes to old scripts

## Documentation

- Updated README.md files to reference "Ordo Quiz" instead of "Quenti"
- Updated comments in code to reference "Ordo Quiz" instead of "Quenti"

## Build Scripts

- Updated build scripts to reference "Ordo Quiz" instead of "Quenti"
- Updated database paths in build scripts

## Note on Backward Compatibility

For backward compatibility, we've kept the original files with "quenti" in their names but updated their content to reference "Ordo Quiz". This ensures that existing code that depends on these modules will continue to work while new code should use the new modules with "ordo_quiz" in their names.

## Next Steps

1. Test the application to ensure it launches correctly with the new naming
2. Update any documentation or user-facing text that still references "educonnect" or "Quenti"
3. Consider a more comprehensive renaming in the future that removes the backward compatibility layers
