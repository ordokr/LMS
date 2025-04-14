use axum::{extract::State, Json};
use std::sync::Arc;
use crate::blockchain::core::Blockchain;
use crate::models::blockchain::Certificate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateRequest {
    pub user_id: String,
    pub course_id: String,
    pub metadata: String,
}

pub async fn create_certificate(
    State(blockchain): State<Arc<Blockchain>>,
    Json(request): Json<CertificateRequest>,
) -> Json<Certificate> {
    let mut blockchain = blockchain.lock().await;
    let certificate = blockchain.add_certificate(&request.user_id, &request.course_id, &request.metadata);
    Json(certificate)
}

pub async fn get_certificate(
    State(blockchain): State<Arc<Blockchain>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Option<Json<Certificate>> {
    let blockchain = blockchain.lock().await;
    blockchain.get_certificate(&id).map(Json)
}