use std::sync::Arc;

use anyhow::{Context, Result};
use rinf::DartSignal;

use database::actions::analysis::{get_analyze_count, if_analyze_exists};
use database::connection::MainDbConnection;

use crate::messages::*;

pub async fn if_analyze_exists_request(
    main_db: Arc<MainDbConnection>,
    dart_signal: DartSignal<IfAnalyzeExistsRequest>,
) -> Result<Option<IfAnalyzeExistsResponse>> {
    let file_id = dart_signal.message.file_id;

    let exists = if_analyze_exists(&main_db, file_id)
        .await
        .with_context(|| format!("Failed to test if analysis exists: {}", file_id))?;
    Ok(Some(IfAnalyzeExistsResponse { file_id, exists }))
}

pub async fn get_analyze_count_request(
    main_db: Arc<MainDbConnection>,
    _dart_signal: DartSignal<GetAnalyzeCountRequest>,
) -> Result<Option<GetAnalyzeCountResponse>> {
    Ok(Some(GetAnalyzeCountResponse {
        count: get_analyze_count(&main_db).await?,
    }))
}
