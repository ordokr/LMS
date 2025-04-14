-- Migration script for file storage system

-- Create file metadata table
CREATE TABLE IF NOT EXISTS file_metadata (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    path VARCHAR(1024) NOT NULL,
    content_type VARCHAR(255) NOT NULL,
    size BIGINT NOT NULL,
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    user_id VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    md5_hash VARCHAR(32),
    CONSTRAINT unique_file_hash UNIQUE (md5_hash, size)
);

-- Create file attachments table for linking files to entities
CREATE TABLE IF NOT EXISTS file_attachments (
    id VARCHAR(255) PRIMARY KEY,
    file_id VARCHAR(255) NOT NULL,
    entity_type VARCHAR(100) NOT NULL,
    entity_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    CONSTRAINT fk_file_attachment_file FOREIGN KEY (file_id) REFERENCES file_metadata(id) ON DELETE CASCADE,
    CONSTRAINT unique_file_entity UNIQUE (file_id, entity_type, entity_id)
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_file_metadata_user ON file_metadata(user_id);
CREATE INDEX IF NOT EXISTS idx_file_metadata_created ON file_metadata(created_at);
CREATE INDEX IF NOT EXISTS idx_file_metadata_name ON file_metadata(name);
CREATE INDEX IF NOT EXISTS idx_file_attachments_entity ON file_attachments(entity_type, entity_id);
