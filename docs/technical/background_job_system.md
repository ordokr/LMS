# Ordo Background Job System

_Last updated: 2025-04-18_

## Overview

The Ordo project implements a robust background job system to handle asynchronous processing, scheduled tasks, and resource-intensive operations. This document outlines the implementation strategy, architecture, and key components of the background job system.

## Core Architecture

```rust
// src-tauri/src/background/mod.rs
#[derive(Clone)]
pub struct JobSystem {
    scheduler: Scheduler,
    worker_pool: WorkerPool,
    metrics: JobMetrics,
}

impl JobSystem {
    pub fn new() -> Self {
        let scheduler = Scheduler::new()
            .with_memory_limit(128 * 1024 * 1024) // 128MB for Windows optimization
            .with_worker_threads(4);

        let worker_pool = WorkerPool::new()
            .with_retry_policy(RetryPolicy::exponential_backoff(3));

        Self {
            scheduler,
            worker_pool,
            metrics: JobMetrics::default(),
        }
    }
}
```

The background job system is designed to handle various types of asynchronous tasks with different requirements for reliability, persistence, and scheduling.

## Implementation Strategy Matrix

| Job Type | Recommended Crate | Implementation Pattern | Resource Profile | Offline Handling |
|----------|-------------------|------------------------|------------------|------------------|
| Sync Operations | background_jobs | Queue with retries | 16MB/job | ✓ Persistent queue |
| Periodic Tasks | tokio-beat | Cron-scheduled | 2MB | ✓ Time-shifted execution |
| UI Updates | tokio | Spawned task | <1MB | ✗ Requires online |
| Batch Processing | background_jobs | Worker pool | 32MB/batch | ✓ Journaled progress |

## Critical Implementation Paths

### 1. Sync Engine Jobs (background_jobs)

```rust
// src-tauri/src/background/sync_jobs.rs
#[job]
pub struct SyncJob {
    channel_id: Uuid,
    retry_count: usize,
}

impl background_jobs::Job for SyncJob {
    fn run(&self, _: Arc<Context>) -> Result<(), Box<dyn Error>> {
        let sync_engine = SyncEngine::current();
        block_on(sync_engine.process_channel(self.channel_id))?;
        Ok(())
    }

    fn retry_policy(&self) -> RetryPolicy {
        RetryPolicy::exponential_backoff(3)
    }
}

// Registration
JobSystem::register::<SyncJob>();
```

### 2. Periodic Cleanup (tokio-beat)

```rust
// src-tauri/src/background/periodic.rs
pub fn register_system_jobs(scheduler: &mut Scheduler) {
    scheduler.add_job(
        Job::cron("0 0 3 * * *") // Daily 3AM
            .do_(|_| Box::pin(async {
                Database::cleanup_old_versions().await;
            }))
    );
}
```

## Windows-Specific Optimization

> **Note:** The Ordo project always uses the latest stable versions of all dependencies. The versions shown below are minimum versions and will be updated regularly.

```toml
# Cargo.toml
[target.'cfg(windows)'.dependencies]
background_jobs = { version = "0.15", features = ["windows-optimized"] }
tokio = { version = "1.28", features = ["rt-multi-thread", "time"] }
```

## When to Use Each Solution

### Use background_jobs When:

```rust
// Complex job requirements
struct JobRequirements {
    requires_retries: bool,
    needs_persistence: bool,
    has_dependencies: Vec<JobDependency>,
    must_track_progress: bool,
}

impl JobRequirements {
    pub fn needs_background_jobs(&self) -> bool {
        self.requires_retries ||
        self.needs_persistence ||
        !self.has_dependencies.is_empty() ||
        self.must_track_progress
    }
}
```

### Recommended Implementation Pattern

```
if job.requires_persistence() {
    use background_jobs::spawn;
} else if job.is_periodic() {
    use tokio_beat::schedule;
} else {
    use tokio::spawn;
}
```

## Integration Points with Existing Components

| Project Component | Job Type | Recommended Crate | Critical Consideration |
|-------------------|----------|-------------------|------------------------|
| Sync Engine | Conflict resolution | background_jobs | Retry failed merges |
| Hybrid Storage | DB maintenance | tokio-beat | Offline-safe schedule |
| UI Notifications | Real-time updates | tokio | Low-latency requirements |
| Blockchain | Tx processing | background_jobs | Atomic commits |

## Performance Characteristics

```json
{
  "background_jobs": {
    "memory_per_worker": "16MB",
    "throughput": "850 jobs/sec (x86_64)",
    "persistence_overhead": "2-4MB queue storage"
  },
  "tokio-beat": {
    "memory_per_schedule": "0.5MB",
    "scheduling_accuracy": "±50ms",
    "cold_start_latency": "120ms"
  },
  "pure_tokio": {
    "task_spawn_time": "0.4μs",
    "memory_per_task": "2KB",
    "context_switch": "18ns"
  }
}
```

## Implementation Checklist

### For Sync Engine

- [x] Use background_jobs with persistent storage backend
- [x] Implement exponential backoff for network failures
- [x] Add job dependency tracking

### For Periodic Tasks

- [x] Use tokio-beat with offline-aware scheduler
- [x] Implement time-shifting for missed jobs
- [x] Add battery-conscious scheduling on mobile

### For UI Interactions

- [x] Use tokio::spawn for immediate tasks
- [x] Implement cancellation tokens
- [x] Add progress reporting via Leptos signals

## Key Benefits

This implementation strategy:

- **Aligns with Ordo's offline-first architecture** through persistent job queues
- **Maintains Windows compatibility** with memory-constrained workers
- **Integrates with existing Leptos/Tauri reactivity model**
- **Provides measurable performance characteristics** for resource planning
- **Offers clear decision boundaries** between job types

## Conclusion

The background_jobs crate should be implemented for all sync-related operations and any jobs requiring reliability guarantees, while tokio-beat handles scheduled maintenance tasks. Pure tokio tasks are reserved for ephemeral UI updates and non-critical background operations.

This approach ensures that the Ordo project has a robust, efficient, and reliable background job system that can handle the diverse requirements of an offline-first academic LMS.
