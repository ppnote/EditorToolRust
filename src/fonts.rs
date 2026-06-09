use eframe::egui;

struct FontCandidate {
    name: &'static str,
    path: String,
    index: u32,
}

pub fn setup_cjk_fonts(ctx: &egui::Context) {
    let candidates = font_candidates();
    let mut fonts = egui::FontDefinitions::default();

    for candidate in candidates {
        let Ok(bytes) = std::fs::read(&candidate.path) else {
            continue;
        };

        fonts.font_data.insert(
            candidate.name.to_string(),
            std::sync::Arc::new(egui::FontData {
                font: std::borrow::Cow::Owned(bytes),
                index: candidate.index,
                tweak: egui::FontTweak::default(),
            }),
        );

        for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
            fonts
                .families
                .entry(family)
                .or_default()
                .insert(0, candidate.name.to_string());
        }

        ctx.set_fonts(fonts);
        return;
    }
}

fn font_candidates() -> Vec<FontCandidate> {
    #[cfg(target_os = "windows")]
    {
        let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());
        let font_dir = format!(r"{windir}\Fonts");
        [
            ("msyh", "msyh.ttc", 0),
            ("simhei", "simhei.ttf", 0),
            ("simsun", "simsun.ttc", 0),
        ]
        .into_iter()
        .map(|(name, file, index)| FontCandidate {
            name,
            path: format!(r"{font_dir}\{file}"),
            index,
        })
        .collect()
    }

    #[cfg(target_os = "macos")]
    {
        [
            ("pingfang", "/System/Library/Fonts/PingFang.ttc", 0),
            ("heiti", "/System/Library/Fonts/STHeiti Medium.ttc", 0),
            (
                "hiragino",
                "/System/Library/Fonts/Hiragino Sans GB.ttc",
                0,
            ),
            (
                "arial_unicode",
                "/Library/Fonts/Arial Unicode.ttf",
                0,
            ),
        ]
        .into_iter()
        .map(|(name, path, index)| FontCandidate {
            name,
            path: path.to_string(),
            index,
        })
        .collect()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        [
            (
                "noto",
                "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
                0,
            ),
            (
                "noto_alt",
                "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
                0,
            ),
        ]
        .into_iter()
        .map(|(name, path, index)| FontCandidate {
            name,
            path: path.to_string(),
            index,
        })
        .collect()
    }
}
