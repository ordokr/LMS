use leptos::*;
use futures::future::join_all;
use std::future::Future;

#[derive(Clone, Debug)]
pub struct LoadResult<T, E> {
    pub data: Option<T>,
    pub error: Option<E>,
}

/// Load multiple resources in parallel and return results
pub async fn load_parallel<T, E, Fut, F>(
    loaders: Vec<F>,
) -> Vec<LoadResult<T, E>> 
where
    T: 'static,
    E: 'static,
    Fut: Future<Output = Result<T, E>> + 'static,
    F: FnOnce() -> Fut,
{
    // Wrap each loader in a future that always produces a LoadResult
    let futures = loaders.into_iter().map(|loader| {
        let future = loader();
        async {
            match future.await {
                Ok(data) => LoadResult { data: Some(data), error: None },
                Err(error) => LoadResult { data: None, error: Some(error) },
            }
        }
    }).collect::<Vec<_>>();
    
    // Run all futures in parallel
    join_all(futures).await
}

/// Helper function for use within Leptos components to load multiple resources in parallel
#[hook]
pub fn use_parallel_resources<T, E, Fut, F, D>(
    loaders: Vec<(D, F)>,
) -> Resource<(usize, Vec<D>), Vec<LoadResult<T, E>>>
where
    T: 'static + Clone,
    E: 'static + Clone,
    D: Clone + PartialEq + 'static,
    Fut: Future<Output = Result<T, E>> + 'static,
    F: Fn(D) -> Fut + Clone + 'static,
{
    // Create a resource that reloads when any dependency changes
    let deps: Vec<D> = loaders.iter().map(|(dep, _)| dep.clone()).collect();
    
    create_resource(
        move || (deps.len(), deps.clone()),
        move |(_, deps_clone)| {
            let loader_fns = loaders.clone();
            
            async move {
                let mapped_loaders = loader_fns.into_iter().enumerate().map(|(i, (_, loader))| {
                    let dep = deps_clone[i].clone();
                    move || loader(dep)
                }).collect();
                
                load_parallel(mapped_loaders).await
            }
        }
    )
}