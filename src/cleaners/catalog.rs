use super::app_caches::AppCachesCleaner;
use super::browser::brave::BraveCleaner;
use super::browser::chrome::ChromeCleaner;
use super::browser::edge::EdgeCleaner;
use super::browser::firefox::FirefoxCleaner;
use super::cleaner::Cleaner;
use super::cleaner::CleanerInfo;
use super::crash_dumps::CrashDumpsCleaner;
use super::defender_history::DefenderHistoryCleaner;
use super::delivery_optimization::DeliveryOptimizationCleaner;
use super::error_reports::ErrorReportsCleaner;
use super::extension_sweep::ExtensionSweepCleaner;
use super::font_cache::FontCacheCleaner;
use super::icon_cache::IconCacheCleaner;
use super::internet_cache::InternetCacheCleaner;
use super::office_cache::OfficeCacheCleaner;
use super::prefetch::PrefetchCleaner;
use super::recycle_bin::RecycleBinCleaner;
use super::shader_cache::ShaderCacheCleaner;
use super::system_temp::SystemTempCleaner;
use super::thumbnail_cache::ThumbnailCacheCleaner;
use super::update_cache::UpdateCacheCleaner;
use super::user_temp::UserTempCleaner;
use super::windows_logs::WindowsLogsCleaner;
use super::windows_old::WindowsOldCleaner;
use crate::core::risk::Risk;

pub fn all_cleaners(include_aggressive: bool) -> Vec<Box<dyn Cleaner>> {
    let mut cleaners: Vec<Box<dyn Cleaner>> = vec![
        Box::new(UserTempCleaner),
        Box::new(SystemTempCleaner),
        Box::new(UpdateCacheCleaner),
        Box::new(DeliveryOptimizationCleaner),
        Box::new(RecycleBinCleaner),
        Box::new(ThumbnailCacheCleaner),
        Box::new(IconCacheCleaner),
        Box::new(InternetCacheCleaner),
        Box::new(ErrorReportsCleaner),
        Box::new(CrashDumpsCleaner),
        Box::new(DefenderHistoryCleaner),
        Box::new(WindowsLogsCleaner),
        Box::new(ShaderCacheCleaner),
        Box::new(OfficeCacheCleaner),
        Box::new(AppCachesCleaner),
        Box::new(EdgeCleaner),
        Box::new(ChromeCleaner),
        Box::new(FirefoxCleaner),
        Box::new(BraveCleaner),
        Box::new(PrefetchCleaner),
        Box::new(FontCacheCleaner),
        Box::new(WindowsOldCleaner),
        Box::new(ExtensionSweepCleaner),
    ];

    cleaners.retain(|cleaner| include_aggressive || cleaner.risk() == Risk::Safe);
    cleaners.sort_by(|left, right| left.id().cmp(right.id()));
    cleaners
}

pub fn describe_all() -> Vec<CleanerInfo> {
    let cleaners = all_cleaners(true);
    let mut infos: Vec<CleanerInfo> = cleaners
        .iter()
        .map(|cleaner| CleanerInfo {
            id: cleaner.id().to_string(),
            display_name: cleaner.display_name().to_string(),
            risk: cleaner.risk(),
            needs_admin: cleaner.needs_admin(),
        })
        .collect();
    infos.sort_by(|left, right| left.id.cmp(&right.id));
    infos
}
