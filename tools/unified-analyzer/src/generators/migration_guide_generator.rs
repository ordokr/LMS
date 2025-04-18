use std::fs;
use std::path::Path;
use chrono::Local;

use crate::analyzers::unified_analyzer::AnalysisResult;

/// Generate migration guide
pub fn generate_migration_guide(result: &AnalysisResult, base_dir: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Generating migration guide...");

    // Ensure docs directory exists
    let docs_dir = Path::new("C:\\Users\\Tim\\Desktop\\LMS\\docs");
    if !docs_dir.exists() {
        fs::create_dir_all(&docs_dir)?;
    }

    // Create the migration guide path
    let guide_path = docs_dir.join("migration_guide.md");

    // Generate the content
    let mut content = String::new();

    // Header
    content.push_str("# Canvas and Discourse to Ordo Migration Guide\n\n");
    content.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));

    content.push_str("This guide outlines the process for migrating from Canvas LMS and Discourse forum systems to the Ordo platform, including data migration strategies and compatibility considerations.\n\n");

    // Migration Overview
    content.push_str("## Migration Overview\n\n");
    content.push_str("The migration process follows these high-level steps:\n\n");
    content.push_str("1. **Assessment**: Evaluate existing Canvas and Discourse installations\n");
    content.push_str("2. **Preparation**: Set up Ordo environment and migration tools\n");
    content.push_str("3. **Data Migration**: Transfer data from source systems to Ordo\n");
    content.push_str("4. **Validation**: Verify data integrity and functionality\n");
    content.push_str("5. **Cutover**: Switch from legacy systems to Ordo\n");
    content.push_str("6. **Monitoring**: Track system performance and address issues\n\n");

    // System Requirements
    content.push_str("## System Requirements\n\n");
    content.push_str("| Requirement | Specification |\n");
    content.push_str("|-------------|---------------|\n");
    content.push_str("| **Ordo Server** | 4+ CPU cores, 16GB+ RAM, 100GB+ SSD |\n");
    content.push_str("| **Database** | SQLite (embedded) |\n");
    content.push_str("| **Operating System** | Linux, macOS, or Windows |\n");
    content.push_str("| **Network** | 100Mbps+ for initial migration |\n\n");

    // Data Migration
    content.push_str("## Data Migration\n\n");
    content.push_str("### Canvas LMS Migration\n\n");
    content.push_str("#### Data Mapping\n\n");
    content.push_str("| Canvas Entity | Ordo Entity | Migration Complexity |\n");
    content.push_str("|---------------|-------------|----------------------|\n");
    content.push_str("| Course | Course | Low |\n");
    content.push_str("| Module | Module | Low |\n");
    content.push_str("| Assignment | Assignment | Medium |\n");
    content.push_str("| Quiz | Assessment | Medium |\n");
    content.push_str("| Discussion | Discussion | Medium |\n");
    content.push_str("| User | User | Low |\n");
    content.push_str("| Enrollment | Enrollment | Low |\n");
    content.push_str("| Submission | Submission | Medium |\n");
    content.push_str("| File | Attachment | Medium |\n");
    content.push_str("| Page | Page | Low |\n");
    content.push_str("| Announcement | Announcement | Low |\n");
    content.push_str("| Calendar Event | Event | Low |\n");
    content.push_str("| Rubric | Rubric | High |\n");
    content.push_str("| Outcome | LearningOutcome | High |\n\n");

    content.push_str("#### Migration Process\n\n");
    content.push_str("```mermaid\n");
    content.push_str("graph TD\n");
    content.push_str("    A[Extract Canvas Data] --> B[Transform to Ordo Format]\n");
    content.push_str("    B --> C[Load into Ordo Database]\n");
    content.push_str("    C --> D[Validate Data Integrity]\n");
    content.push_str("    D --> E[Migrate Files/Attachments]\n");
    content.push_str("    E --> F[Update References]\n");
    content.push_str("```\n\n");

    content.push_str("#### Canvas API Migration\n\n");
    content.push_str("The migration tool uses Canvas API to extract data:\n\n");
    content.push_str("```rust\n");
    content.push_str("pub async fn extract_canvas_courses(\n");
    content.push_str("    canvas_url: &str,\n");
    content.push_str("    api_token: &str,\n");
    content.push_str(") -> Result<Vec<CanvasCourse>, MigrationError> {\n");
    content.push_str("    let client = reqwest::Client::new();\n");
    content.push_str("    \n");
    content.push_str("    // Fetch courses from Canvas API\n");
    content.push_str("    let response = client\n");
    content.push_str("        .get(&format!(\"{}/api/v1/courses\", canvas_url))\n");
    content.push_str("        .header(\"Authorization\", format!(\"Bearer {}\", api_token))\n");
    content.push_str("        .send()\n");
    content.push_str("        .await?;\n");
    content.push_str("    \n");
    content.push_str("    if !response.status().is_success() {\n");
    content.push_str("        return Err(MigrationError::ApiError(format!(\n");
    content.push_str("            \"Failed to fetch courses: {}\",\n");
    content.push_str("            response.status()\n");
    content.push_str("        )));\n");
    content.push_str("    }\n");
    content.push_str("    \n");
    content.push_str("    let courses: Vec<CanvasCourse> = response.json().await?;\n");
    content.push_str("    Ok(courses)\n");
    content.push_str("}\n");
    content.push_str("```\n\n");

    content.push_str("### Discourse Migration\n\n");
    content.push_str("#### Data Mapping\n\n");
    content.push_str("| Discourse Entity | Ordo Entity | Migration Complexity |\n");
    content.push_str("|------------------|-------------|----------------------|\n");
    content.push_str("| Category | ForumCategory | Low |\n");
    content.push_str("| Topic | ForumTopic | Medium |\n");
    content.push_str("| Post | ForumPost | Medium |\n");
    content.push_str("| User | User | Low |\n");
    content.push_str("| Tag | Tag | Low |\n");
    content.push_str("| Group | Group | Medium |\n");
    content.push_str("| Private Message | Message | High |\n");
    content.push_str("| Notification | Notification | Medium |\n");
    content.push_str("| Upload | Attachment | Medium |\n");
    content.push_str("| Badge | Badge | Medium |\n\n");

    content.push_str("#### Migration Process\n\n");
    content.push_str("```mermaid\n");
    content.push_str("graph TD\n");
    content.push_str("    A[Extract Discourse Data] --> B[Transform to Ordo Format]\n");
    content.push_str("    B --> C[Load into Ordo Database]\n");
    content.push_str("    C --> D[Validate Data Integrity]\n");
    content.push_str("    D --> E[Migrate Uploads]\n");
    content.push_str("    E --> F[Update References]\n");
    content.push_str("```\n\n");

    content.push_str("#### Discourse API Migration\n\n");
    content.push_str("The migration tool uses Discourse API to extract data:\n\n");
    content.push_str("```rust\n");
    content.push_str("pub async fn extract_discourse_topics(\n");
    content.push_str("    discourse_url: &str,\n");
    content.push_str("    api_key: &str,\n");
    content.push_str("    api_username: &str,\n");
    content.push_str(") -> Result<Vec<DiscourseTopic>, MigrationError> {\n");
    content.push_str("    let client = reqwest::Client::new();\n");
    content.push_str("    \n");
    content.push_str("    // Fetch topics from Discourse API\n");
    content.push_str("    let response = client\n");
    content.push_str("        .get(&format!(\"{}/latest.json\", discourse_url))\n");
    content.push_str("        .query(&[\n");
    content.push_str("            (\"api_key\", api_key),\n");
    content.push_str("            (\"api_username\", api_username),\n");
    content.push_str("        ])\n");
    content.push_str("        .send()\n");
    content.push_str("        .await?;\n");
    content.push_str("    \n");
    content.push_str("    if !response.status().is_success() {\n");
    content.push_str("        return Err(MigrationError::ApiError(format!(\n");
    content.push_str("            \"Failed to fetch topics: {}\",\n");
    content.push_str("            response.status()\n");
    content.push_str("        )));\n");
    content.push_str("    }\n");
    content.push_str("    \n");
    content.push_str("    let data: DiscourseTopicList = response.json().await?;\n");
    content.push_str("    Ok(data.topic_list.topics)\n");
    content.push_str("}\n");
    content.push_str("```\n\n");

    content.push_str("## User Migration\n\n");
    content.push_str("User accounts require special handling to ensure security and continuity:\n\n");
    content.push_str("1. **Account Mapping**: Match users between Canvas and Discourse\n");
    content.push_str("2. **Password Handling**: Reset passwords or use temporary tokens\n");
    content.push_str("3. **Role Migration**: Map roles and permissions to Ordo equivalents\n");
    content.push_str("4. **Profile Data**: Consolidate profile information from both systems\n\n");

    content.push_str("### User Migration Code Example\n\n");
    content.push_str("```rust\n");
    content.push_str("pub async fn migrate_users(\n");
    content.push_str("    canvas_users: Vec<CanvasUser>,\n");
    content.push_str("    discourse_users: Vec<DiscourseUser>,\n");
    content.push_str(") -> Result<Vec<OrdoUser>, MigrationError> {\n");
    content.push_str("    let mut ordo_users = Vec::new();\n");
    content.push_str("    \n");
    content.push_str("    // Create a map of email to Discourse user for quick lookup\n");
    content.push_str("    let discourse_user_map: HashMap<String, DiscourseUser> = discourse_users\n");
    content.push_str("        .into_iter()\n");
    content.push_str("        .map(|u| (u.email.clone(), u))\n");
    content.push_str("        .collect();\n");
    content.push_str("    \n");
    content.push_str("    // Process each Canvas user\n");
    content.push_str("    for canvas_user in canvas_users {\n");
    content.push_str("        // Look for matching Discourse user\n");
    content.push_str("        let discourse_user = discourse_user_map.get(&canvas_user.email);\n");
    content.push_str("        \n");
    content.push_str("        // Create Ordo user with combined data\n");
    content.push_str("        let ordo_user = OrdoUser {\n");
    content.push_str("            id: Uuid::new_v4().to_string(),\n");
    content.push_str("            email: canvas_user.email.clone(),\n");
    content.push_str("            name: canvas_user.name.clone(),\n");
    content.push_str("            username: discourse_user\n");
    content.push_str("                .map(|u| u.username.clone())\n");
    content.push_str("                .unwrap_or_else(|| generate_username(&canvas_user.name)),\n");
    content.push_str("            avatar_url: discourse_user\n");
    content.push_str("                .map(|u| u.avatar_template.clone())\n");
    content.push_str("                .unwrap_or_default(),\n");
    content.push_str("            bio: discourse_user\n");
    content.push_str("                .map(|u| u.bio_raw.clone())\n");
    content.push_str("                .unwrap_or_default(),\n");
    content.push_str("            created_at: Utc::now(),\n");
    content.push_str("            // Generate temporary password and require reset\n");
    content.push_str("            password_hash: generate_temporary_password_hash(),\n");
    content.push_str("            password_reset_required: true,\n");
    content.push_str("        };\n");
    content.push_str("        \n");
    content.push_str("        ordo_users.push(ordo_user);\n");
    content.push_str("    }\n");
    content.push_str("    \n");
    content.push_str("    Ok(ordo_users)\n");
    content.push_str("}\n");
    content.push_str("```\n\n");

    content.push_str("## Content Migration\n\n");
    content.push_str("### File Attachments\n\n");
    content.push_str("Files and attachments are migrated using this process:\n\n");
    content.push_str("1. Download file from source system\n");
    content.push_str("2. Hash file contents for integrity verification\n");
    content.push_str("3. Upload to Ordo storage system\n");
    content.push_str("4. Update references in migrated content\n\n");

    content.push_str("### Rich Content\n\n");
    content.push_str("Rich content (HTML, embedded media) requires special handling:\n\n");
    content.push_str("1. Parse HTML content\n");
    content.push_str("2. Update internal links and references\n");
    content.push_str("3. Rewrite embedded content for Ordo format\n");
    content.push_str("4. Validate content rendering\n\n");

    content.push_str("## Integration Considerations\n\n");
    content.push_str("### Authentication Integration\n\n");
    content.push_str("If maintaining external authentication systems:\n\n");
    content.push_str("```rust\n");
    content.push_str("pub async fn configure_sso(\n");
    content.push_str("    config: &mut OrdoConfig,\n");
    content.push_str("    canvas_oauth_config: Option<CanvasOAuthConfig>,\n");
    content.push_str("    discourse_sso_config: Option<DiscourseSsoConfig>,\n");
    content.push_str(") -> Result<(), ConfigError> {\n");
    content.push_str("    // Configure Canvas OAuth if available\n");
    content.push_str("    if let Some(canvas_config) = canvas_oauth_config {\n");
    content.push_str("        config.oauth_providers.push(OAuthProvider {\n");
    content.push_str("            name: \"canvas\".to_string(),\n");
    content.push_str("            client_id: canvas_config.client_id,\n");
    content.push_str("            client_secret: canvas_config.client_secret,\n");
    content.push_str("            authorize_url: format!(\"{}/login/oauth2/auth\", canvas_config.base_url),\n");
    content.push_str("            token_url: format!(\"{}/login/oauth2/token\", canvas_config.base_url),\n");
    content.push_str("            user_info_url: format!(\"{}/api/v1/users/self\", canvas_config.base_url),\n");
    content.push_str("            scope: \"read\".to_string(),\n");
    content.push_str("        });\n");
    content.push_str("    }\n");
    content.push_str("    \n");
    content.push_str("    // Configure Discourse SSO if available\n");
    content.push_str("    if let Some(discourse_config) = discourse_sso_config {\n");
    content.push_str("        config.sso_providers.push(SsoProvider {\n");
    content.push_str("            name: \"discourse\".to_string(),\n");
    content.push_str("            sso_secret: discourse_config.sso_secret,\n");
    content.push_str("            sso_url: format!(\"{}/session/sso_provider\", discourse_config.base_url),\n");
    content.push_str("        });\n");
    content.push_str("    }\n");
    content.push_str("    \n");
    content.push_str("    Ok(())\n");
    content.push_str("}\n");
    content.push_str("```\n\n");

    content.push_str("### API Compatibility\n\n");
    content.push_str("For systems that integrate with Canvas or Discourse APIs:\n\n");
    content.push_str("1. Ordo provides Canvas API compatibility layer\n");
    content.push_str("2. Discourse API compatibility is partial\n");
    content.push_str("3. Update API endpoints in integrating systems\n\n");

    content.push_str("## Post-Migration Tasks\n\n");
    content.push_str("1. **Verify User Access**: Ensure all users can access their content\n");
    content.push_str("2. **Check Permissions**: Verify roles and permissions are correctly applied\n");
    content.push_str("3. **Validate Content**: Sample content to ensure proper migration\n");
    content.push_str("4. **Test Integrations**: Verify all integrated systems work properly\n");
    content.push_str("5. **Performance Testing**: Ensure system performs well under load\n\n");

    content.push_str("## Rollback Plan\n\n");
    content.push_str("In case of migration issues:\n\n");
    content.push_str("1. Keep source systems running during migration\n");
    content.push_str("2. Maintain database backups of source systems\n");
    content.push_str("3. Document rollback procedures for each migration step\n");
    content.push_str("4. Test rollback procedures before migration\n\n");

    content.push_str("## Migration Timeline\n\n");
    content.push_str("| Phase | Duration | Description |\n");
    content.push_str("|-------|----------|-------------|\n");
    content.push_str("| Planning | 2-4 weeks | Assessment and preparation |\n");
    content.push_str("| Development | 4-8 weeks | Migration tool development |\n");
    content.push_str("| Testing | 2-4 weeks | Testing in staging environment |\n");
    content.push_str("| Migration | 1-2 days | Actual data migration |\n");
    content.push_str("| Validation | 1 week | Post-migration validation |\n");
    content.push_str("| Support | 4 weeks | Post-migration support |\n\n");

    content.push_str("## Conclusion\n\n");
    content.push_str("Migrating from Canvas and Discourse to Ordo requires careful planning and execution, but provides significant benefits in terms of offline capabilities, integrated functionality, and improved performance. This guide provides a framework for successful migration, but each implementation may require customization based on specific needs and configurations.");

    // Write to file
    fs::write(&guide_path, content)?;

    println!("Migration guide generated at: {:?}", guide_path);

    Ok(())
}
