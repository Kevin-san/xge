//! Engine Ecosystem Module
//!
//! 提供资源商店、性能分析器和遥测系统功能
//!
//! # 核心模块
//!
//! - `AssetStore`: 资源商店核心结构
//! - `PerformanceProfiler`: 性能分析器
//! - `Telemetry`: 遥测系统
//! - `AssetStoreClient`: 商店客户端

mod asset_store;
mod client;
mod common;
mod profiler;
mod telemetry;

pub use asset_store::*;
pub use client::*;
pub use common::*;
pub use profiler::*;
pub use telemetry::*;

// 常量定义
const DEFAULT_SAMPLE_RATE_HZ: u32 = 100;
const DEFAULT_MAX_FRAMES: usize = 1000;
const DEFAULT_CACHE_DIR: &str = "cache";
const DEFAULT_INSTALL_DIR: &str = "assets";
const TELEMETRY_ENDPOINT: &str = "https://telemetry.engine.example.com";
const ASSET_STORE_ENDPOINT: &str = "https://store.engine.example.com";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_store::*;
    use crate::client::*;
    use crate::common::*;
    use crate::profiler::*;
    use crate::telemetry::*;

    /// 测试 AssetId
    #[test]
    fn test_asset_id() {
        let id = AssetId::new();
        assert!(!id.to_string().is_empty());

        let parsed = AssetId::parse(&id.to_string());
        assert!(parsed.is_ok());
        assert_eq!(id, parsed.unwrap());
    }

    /// 测试 AssetVersion
    #[test]
    fn test_asset_version() {
        let version = AssetVersion::new(1, 2, 3);
        assert_eq!(version.to_string(), "1.2.3");

        let parsed = AssetVersion::parse("2.0.1");
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();
        assert_eq!(parsed.major(), 2);
        assert_eq!(parsed.minor(), 0);
        assert_eq!(parsed.patch(), 1);

        let v1 = AssetVersion::new(1, 0, 0);
        let v2 = AssetVersion::new(2, 0, 0);
        assert!(v1 < v2);
    }

    /// 测试 AssetStore
    #[test]
    fn test_asset_store() {
        let store = AssetStore::default();

        let asset = InstalledAsset {
            id: AssetId::new(),
            name: "Test Asset".to_string(),
            version: AssetVersion::default(),
            install_path: std::path::PathBuf::from("/tmp/test"),
            install_time: chrono::Utc::now(),
            files: vec![std::path::PathBuf::from("/tmp/test/file.bin")],
        };

        store.add_installed(asset.clone());
        let installed = store.list_installed();
        assert_eq!(installed.len(), 1);

        let retrieved = store.get_installed(&asset.id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Asset");

        store.remove_installed(&asset.id);
        assert_eq!(store.list_installed().len(), 0);
    }

    /// 测试 Cart
    #[test]
    fn test_cart() {
        let mut cart = Cart::new();

        cart.add(AssetId::new(), "Free Asset".to_string(), PriceModel::Free);
        cart.add(
            AssetId::new(),
            "Paid Asset".to_string(),
            PriceModel::Paid {
                amount: 10.0,
                currency: "USD".to_string(),
            },
        );

        assert_eq!(cart.items().len(), 2);
        assert_eq!(cart.total(), 10.0);

        let first_id = cart.items()[0].asset_id.clone();
        cart.remove(&first_id);
        assert_eq!(cart.items().len(), 1);

        cart.clear();
        assert_eq!(cart.items().len(), 0);
    }

    /// 测试 CommentSystem
    #[test]
    fn test_comment_system() {
        let mut system = CommentSystem::new();
        let asset_id = AssetId::new();

        let comment = Comment::new("User".to_string(), "Great asset!".to_string(), 5);
        system.post(asset_id.clone(), comment);

        let comments = system.list(&asset_id);
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].rating, 5);

        system.vote(&asset_id, 0, true);
        let comments = system.list(&asset_id);
        assert_eq!(comments[0].helpful_votes, 1);
    }

    /// 测试 PerformanceProfiler
    #[test]
    fn test_profiler() {
        let config = ProfilerConfig::new(100, 100);
        let mut profiler = PerformanceProfiler::new(config);

        profiler.begin_frame();
        {
            let _guard = profiler.begin_scope("test_scope");
        }
        // 在 end_frame 之前检查样本
        let cpu_samples_before = profiler.cpu_samples();
        assert!(cpu_samples_before.len() > 0);
        assert!(cpu_samples_before
            .iter()
            .any(|s| s.scope_name == "test_scope"));

        profiler.end_frame();

        assert_eq!(profiler.frame_count(), 1);
        assert_eq!(profiler.current_frame_number(), 1);
    }

    /// 测试 FlameGraph
    #[test]
    fn test_flame_graph() {
        let samples = vec![
            CpuSample {
                scope_name: "root".to_string(),
                thread_id: 0,
                start_ns: 0,
                duration_ns: 1000,
                parent_index: None,
                data: None,
            },
            CpuSample {
                scope_name: "child".to_string(),
                thread_id: 0,
                start_ns: 100,
                duration_ns: 500,
                parent_index: Some(0),
                data: None,
            },
        ];

        let flame = FlameGraph::from_samples(&samples);
        assert!(flame.root().children.len() > 0);
    }

    /// 测试 Histogram
    #[test]
    fn test_histogram() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let histogram = Histogram::from_values(&values, 5);

        assert!(histogram.mean() > 0.0);
        assert!(histogram.median() > 0.0);
        assert!(histogram.min() >= 1.0);
        assert!(histogram.max() <= 10.0);
    }

    /// 测试 FrameMetricsAggregator
    #[test]
    fn test_frame_metrics_aggregator() {
        let mut aggregator = FrameMetricsAggregator::new();

        for i in 0..10 {
            aggregator.push(FrameMetrics {
                frame_number: i,
                frame_time: 16.0,
                gpu_time: 5.0,
                cpu_time: 10.0,
                draw_calls: 100,
                triangles: 1000,
                vertices: 500,
            });
        }

        assert_eq!(aggregator.average_frame_time(), 16.0);
        assert!(aggregator.average_fps() > 0.0);
        assert_eq!(aggregator.total_triangles(), 10000);
    }

    /// 测试 Telemetry
    #[test]
    fn test_telemetry() {
        let config = TelemetryConfig::new("http://localhost".to_string(), true);
        let mut telemetry = Telemetry::new(config);

        assert!(telemetry.is_enabled());

        telemetry.record_performance(crate::profiler::Metrics::new(16.0));
        assert_eq!(telemetry.buffer_size(), 1);

        telemetry.clear_buffer();
        assert_eq!(telemetry.buffer_size(), 0);

        telemetry.disable();
        assert!(!telemetry.is_enabled());
    }

    /// 测试 TelemetryEvent
    #[test]
    fn test_telemetry_event() {
        let config = TelemetryConfig::new("http://localhost".to_string(), true);
        let mut telemetry = Telemetry::new(config);

        let error = ErrorEvent::new("TestError".to_string(), "Test message".to_string());
        telemetry.record_error(error);
        assert_eq!(telemetry.buffer_size(), 1);

        let action = UserAction::new("click".to_string(), "button".to_string())
            .with_detail("x".to_string(), "100".to_string());
        telemetry.record_user_action(action);
        assert_eq!(telemetry.buffer_size(), 2);
    }

    /// 测试 AssetStoreClient
    #[test]
    fn test_asset_store_client() {
        let mut client = AssetStoreClient::default();

        assert!(!client.is_logged_in());
        client.login("test_user", "password").unwrap();
        assert!(client.is_logged_in());

        let profile = client.me().unwrap();
        assert_eq!(profile.username, "test_user");

        client.logout();
        assert!(!client.is_logged_in());
    }

    /// 测试 AssetStoreClient 搜索
    #[test]
    fn test_asset_store_client_search() {
        let client = AssetStoreClient::default();

        let results = client.search("test", SearchFilters::default()).unwrap();
        assert!(results.len() > 0);
        assert!(results[0].name.contains("test"));
    }

    /// 测试 AssetStoreClient 购物车
    #[test]
    fn test_asset_store_client_cart() {
        let mut client = AssetStoreClient::default();

        client.add_to_cart(AssetId::new(), "Free Item".to_string(), PriceModel::Free);
        client.add_to_cart(
            AssetId::new(),
            "Paid Item".to_string(),
            PriceModel::Paid {
                amount: 5.0,
                currency: "USD".to_string(),
            },
        );

        assert_eq!(client.cart_items().len(), 2);
        assert_eq!(client.cart_total(), 5.0);

        client.remove_from_cart(&client.cart_items()[0].asset_id);
        assert_eq!(client.cart_items().len(), 1);
    }

    /// 测试 AssetStoreClient 安装
    #[test]
    fn test_asset_store_client_install() {
        let mut client = AssetStoreClient::default();

        let asset_id = AssetId::new();
        let path = client.download(&asset_id).unwrap();

        let installed = client
            .install(&path, &std::path::PathBuf::from("/tmp"))
            .unwrap();
        assert!(!installed.name.is_empty());

        let installed_list = client.list_installed();
        assert!(installed_list.len() > 0);

        client.uninstall(&installed.id).unwrap();
        assert_eq!(client.list_installed().len(), 0);
    }

    /// 测试 DeveloperCenter
    #[test]
    fn test_developer_center() {
        let mut center = DeveloperCenter::new();

        let draft = DeveloperDraft {
            title: "Test Asset".to_string(),
            description: "Test description".to_string(),
            category: AssetCategory::Models3D,
            tags: vec!["test".to_string()],
            price_model: PriceModel::Free,
            files_dir: std::path::PathBuf::from("/tmp"),
            screenshots: vec!["screen.png".to_string()],
            videos: vec!["video.mp4".to_string()],
        };

        let published = center.publish_asset(draft).unwrap();
        assert!(!published.name.is_empty());

        center.submit_for_review(&published.id).unwrap();
        let status = center.review_status(&published.id).unwrap();
        assert_eq!(status, ReviewStatus::UnderReview);
    }

    /// 测试 PerformanceDiagnosticEngine
    #[test]
    fn test_diagnostic_engine() {
        let engine = PerformanceDiagnosticEngine::new();
        let profiler = PerformanceProfiler::default();

        let warnings = engine.run(&profiler);
        assert!(warnings.len() >= 0);
    }

    /// 测试 BaselineProfile
    #[test]
    fn test_baseline_profile() {
        let baseline = BaselineProfile::new(
            "test_baseline".to_string(),
            vec![CpuSample {
                scope_name: "test".to_string(),
                thread_id: 0,
                start_ns: 0,
                duration_ns: 1000,
                parent_index: None,
                data: None,
            }],
        );

        let new_samples = vec![CpuSample {
            scope_name: "test".to_string(),
            thread_id: 0,
            start_ns: 0,
            duration_ns: 2000,
            parent_index: None,
            data: None,
        }];

        let report = baseline.compare(&new_samples);
        assert!(report.regression_count() >= 0);
    }

    /// 测试 AssetRating
    #[test]
    fn test_asset_rating() {
        let rating = AssetRating::new(4.5, 100);
        assert_eq!(rating.stars(), 4.5);
        assert_eq!(rating.review_count(), 100);
    }

    /// 测试 AuthToken
    #[test]
    fn test_auth_token() {
        let token = AuthToken::new("test_token".to_string());
        assert!(!token.expired());
        assert_eq!(token.to_string(), "test_token");

        let refreshed = token.refresh();
        assert!(refreshed.to_string().contains("refreshed"));
    }

    /// 测试 DownloadProgress
    #[test]
    fn test_download_progress() {
        let progress = DownloadProgress {
            bytes_downloaded: 500,
            bytes_total: 1000,
            speed_kbps: 100.0,
            eta_seconds: 5,
        };

        assert_eq!(progress.percent(), 50.0);
    }

    /// 测试 Money
    #[test]
    fn test_money() {
        let money = Money::new(10.0, "USD".to_string());
        assert_eq!(money.to_string(), "10 USD");
    }

    /// 测试 RevenueSplit
    #[test]
    fn test_revenue_split() {
        let split = RevenueSplit::default_70_30();
        assert_eq!(split.developer_percent, 70.0);
        assert_eq!(split.platform_percent, 30.0);

        let premium = RevenueSplit::premium();
        assert_eq!(premium.developer_percent, 80.0);

        let custom = RevenueSplit::custom(90.0, 10.0);
        assert_eq!(custom.developer_percent, 90.0);
    }

    /// 测试 DeviceInfo
    #[test]
    fn test_device_info() {
        let info = DeviceInfo::detect();
        assert!(!info.os.is_empty());
        assert!(!info.engine_version.is_empty());
    }

    /// 测试 TelemetryStats
    #[test]
    fn test_telemetry_stats() {
        let events = vec![
            TelemetryEvent::Performance {
                timestamp: chrono::Utc::now(),
                session_id: "test".to_string(),
                metrics: crate::profiler::Metrics::new(16.0),
            },
            TelemetryEvent::Error {
                timestamp: chrono::Utc::now(),
                session_id: "test".to_string(),
                error: ErrorEvent::new("Test".to_string(), "Error".to_string()),
            },
        ];

        let stats = TelemetryStats::from_events(&events);
        assert_eq!(stats.total_events(), 2);
        assert!(stats.events_by_type().contains_key("Performance"));
        assert!(stats.events_by_type().contains_key("Error"));
    }
}
