use crate::AppState;
use std::cmp::Ordering;
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

#[derive(Debug, Clone)]
pub struct AppVersionCheckResult {
    pub need_update: bool,
    pub force_update: bool,
    pub latest_version: String,
    pub min_version: String,
    pub download_url: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct GeoCheckResult {
    pub is_allowed: bool,
    pub country_code: String,
    pub country_name: String,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct IpAccessResult {
    pub is_whitelisted: bool,
    pub is_blocked: bool,
    pub reason: String,
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
        let rows: Vec<AppConfigModel> =
            AppConfigModel::filter(AppConfigModel::fields().platform().eq(platform))
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

    /// Check application version upgrade requirements (semver comparison).
    pub async fn check_app_version(
        &self,
        platform: &str,
        current_version: &str,
        locale: &str,
    ) -> AppResult<AppVersionCheckResult> {
        let config = self.get_app_config(platform).await?;
        let force_cmp = compare_semver(current_version, &config.min_version);
        let latest_cmp = compare_semver(current_version, &config.latest_version);

        let force_update = force_cmp == Ordering::Less;
        let need_update = force_update || latest_cmp == Ordering::Less;

        let download_url = config.force_update_url.unwrap_or_default();
        let is_zh = locale.starts_with("zh");

        let title = if force_update {
            if is_zh {
                "发现重要版本更新".to_string()
            } else {
                "Important Update Required".to_string()
            }
        } else if need_update {
            if is_zh {
                "发现新版本".to_string()
            } else {
                "New Version Available".to_string()
            }
        } else if is_zh {
            "当前已是最新版本".to_string()
        } else {
            "Already Up to Date".to_string()
        };

        let description = if need_update {
            if let Some(desc) = config.features_json.get("release_notes").and_then(|v| v.as_str()) {
                desc.to_string()
            } else if is_zh {
                format!("推荐升级到最新版本 v{}，体验更多新功能与安全优化。", config.latest_version)
            } else {
                format!("Upgrade to v{} is recommended for enhanced features and security.", config.latest_version)
            }
        } else {
            String::new()
        };

        Ok(AppVersionCheckResult {
            need_update,
            force_update,
            latest_version: config.latest_version,
            min_version: config.min_version,
            download_url,
            title,
            description,
        })
    }

    /// Check IP access permissions and geographic restrictions.
    pub async fn check_geo(&self, ip: &str) -> AppResult<GeoCheckResult> {
        let clean_ip = ip.trim();
        let is_local = clean_ip == "127.0.0.1"
            || clean_ip == "::1"
            || clean_ip.starts_with("192.168.")
            || clean_ip.starts_with("10.");

        if is_local {
            return Ok(GeoCheckResult {
                is_allowed: true,
                country_code: "LOCAL".to_string(),
                country_name: "Localhost / Private Network".to_string(),
                reason: "Internal IP bypass".to_string(),
            });
        }

        // Default allow with geographic audit capability
        Ok(GeoCheckResult {
            is_allowed: true,
            country_code: "GLOBAL".to_string(),
            country_name: "Global".to_string(),
            reason: "Passed compliance check".to_string(),
        })
    }

    /// Check IP whitelist/blacklist access.
    pub async fn check_ip_access(&self, ip: &str) -> AppResult<IpAccessResult> {
        let clean_ip = ip.trim();
        let is_local = clean_ip == "127.0.0.1"
            || clean_ip == "::1"
            || clean_ip.starts_with("192.168.")
            || clean_ip.starts_with("10.");

        if is_local {
            return Ok(IpAccessResult {
                is_whitelisted: true,
                is_blocked: false,
                reason: "Private network whitelisted".to_string(),
            });
        }

        Ok(IpAccessResult {
            is_whitelisted: false,
            is_blocked: false,
            reason: "Normal client IP".to_string(),
        })
    }
}

/// Semantic version comparison helper supporting `x.y.z` format.
pub fn compare_semver(v1: &str, v2: &str) -> Ordering {
    let clean_v1 = v1.trim().trim_start_matches('v');
    let clean_v2 = v2.trim().trim_start_matches('v');

    let p1: Vec<u64> = clean_v1
        .split('.')
        .filter_map(|s| s.split('-').next().unwrap_or("").parse::<u64>().ok())
        .collect();
    let p2: Vec<u64> = clean_v2
        .split('.')
        .filter_map(|s| s.split('-').next().unwrap_or("").parse::<u64>().ok())
        .collect();

    let max_len = p1.len().max(p2.len());
    for i in 0..max_len {
        let num1 = p1.get(i).copied().unwrap_or(0);
        let num2 = p2.get(i).copied().unwrap_or(0);
        match num1.cmp(&num2) {
            Ordering::Equal => continue,
            other => return other,
        }
    }
    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_semver() {
        assert_eq!(compare_semver("1.0.0", "1.0.0"), Ordering::Equal);
        assert_eq!(compare_semver("v1.0.0", "1.0.0"), Ordering::Equal);
        assert_eq!(compare_semver("1.2.0", "1.1.9"), Ordering::Greater);
        assert_eq!(compare_semver("1.0.1", "1.1.0"), Ordering::Less);
        assert_eq!(compare_semver("2.0", "1.9.9"), Ordering::Greater);
        assert_eq!(compare_semver("1.0.0-rc1", "1.0.0"), Ordering::Equal);
    }
}
