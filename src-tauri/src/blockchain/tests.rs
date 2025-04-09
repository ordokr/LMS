#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use crate::blockchain::domain::{UserId, CourseId, AchievementRecord, AchievementType};
    use crate::blockchain::HybridChain;
    
    // Unit test for basic blockchain functionality
    #[tokio::test]
    async fn test_create_block() {
        let mut chain = HybridChain::new(None).await.unwrap();
        let timestamp = chain.create_block().await.unwrap();
        assert!(timestamp > 0);
    }
    
    // Property-based test for blockchain invariants
    proptest! {
        #[test]
        fn test_blockchain_ordering(
            transactions in prop::collection::vec(any::<AchievementRecord>(), 0..100)
        ) {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let mut chain = HybridChain::new(None).await.unwrap();
                
                let mut timestamps = Vec::new();
                
                for _ in &transactions {
                    let timestamp = chain.create_block().await.unwrap();
                    timestamps.push(timestamp);
                }
                
                // Check that timestamps are strictly increasing
                for i in 1..timestamps.len() {
                    assert!(timestamps[i] > timestamps[i-1]);
                }
                
                // Other invariants could be checked here
            });
        }
    }
    
    // Generate arbitrary achievement records for property testing
    impl Arbitrary for AchievementRecord {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;
        
        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (
                any::<[u8; 16]>(),       // For UserId
                any::<[u8; 16]>(),       // For CourseId
                any::<[u8; 32]>(),       // For tx_hash
                any::<i64>(),            // For timestamp
                prop::sample::select(vec![
                    AchievementType::CourseCompletion,
                    AchievementType::BadgeEarned,
                    AchievementType::CertificateIssued,
                ]),
            ).prop_map(|(user_bytes, course_bytes, tx_hash, timestamp, achievement_type)| {
                let user = UserId(Uuid::from_bytes(user_bytes));
                let course = CourseId(Uuid::from_bytes(course_bytes));
                let datetime = chrono::DateTime::from_timestamp(
                    timestamp.abs(), 0
                ).unwrap_or_else(|| chrono::Utc::now());
                
                AchievementRecord {
                    user,
                    course,
                    tx_hash,
                    timestamp: datetime,
                    achievement_type,
                }
            })
            .boxed()
        }
    }
    
    // Benchmark using Criterion
    #[cfg(feature = "bench")]
    mod benchmarks {
        use super::*;
        use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
        
        fn bench_block_creation(c: &mut Criterion) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            
            let mut group = c.benchmark_group("blockchain");
            
            for size in [10, 100, 1000].iter() {
                group.bench_with_input(BenchmarkId::new("block_creation", size), size, |b, &size| {
                    b.iter(|| {
                        rt.block_on(async {
                            let mut chain = HybridChain::new(None).await.unwrap();
                            
                            // Create multiple blocks to test batch performance
                            for _ in 0..size {
                                chain.create_block().await.unwrap();
                            }
                        });
                    });
                });
            }
            
            group.finish();
        }
        
        criterion_group!(benches, bench_block_creation);
        criterion_main!(benches);
    }
}