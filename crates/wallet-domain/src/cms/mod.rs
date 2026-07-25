use crate::AppState;
use uuid::Uuid;
use wallet_db::models::{AppConfig as AppConfigModel, Guide as GuideModel};
use wallet_error::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct AppConfigRow {
    pub platform: String,
    pub min_version: String,
    pub latest_version: String,
    pub force_update_url: Option<String>,
    pub features_json: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct GuideRow {
    pub id: Uuid,
    pub locale: String,
    pub title: String,
    pub body: String,
}

pub struct CmsService<'a> {
    pub state: &'a AppState,
}

impl<'a> CmsService<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    pub async fn get_app_config(&self, platform: &str) -> AppResult<AppConfigRow> {
        let mut db = self.state.db.clone_inner();
        let rows: Vec<AppConfigModel> = AppConfigModel::filter(
            AppConfigModel::fields().platform().eq(platform),
        )
        .exec(&mut db)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;

        match rows.into_iter().next() {
            Some(r) => {
                let features: serde_json::Value =
                    serde_json::from_str(&r.features_json).unwrap_or(serde_json::json!({}));
                Ok(AppConfigRow {
                    platform: r.platform,
                    min_version: r.min_version,
                    latest_version: r.latest_version,
                    force_update_url: r.force_update_url,
                    features_json: features,
                })
            }
            None => Err(AppError::NotFound(format!(
                "app config for platform '{platform}'"
            ))),
        }
    }

    pub async fn list_guides(
        &self,
        locale: &str,
        page: u32,
        page_size: u32,
    ) -> AppResult<(Vec<GuideRow>, i64)> {
        let limit = page_size.max(1) as usize;
        let offset = ((page.max(1) - 1) * page_size) as usize;
        let mut db = self.state.db.clone_inner();
        let rows: Vec<GuideModel> = GuideModel::filter(GuideModel::fields().locale().eq(locale))
            .limit(limit)
            .offset(offset)
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        let total = GuideModel::filter(GuideModel::fields().locale().eq(locale))
            .count()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        let items = rows
            .into_iter()
            .map(|g| GuideRow {
                id: g.id,
                locale: g.locale,
                title: g.title,
                body: g.body,
            })
            .collect();
        Ok((items, total as i64))
    }
}
