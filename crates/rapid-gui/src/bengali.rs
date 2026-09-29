use std::sync::OnceLock;

static FACE: OnceLock<rustybuzz::Face<'static>> = OnceLock::new();

pub fn get_bengali_face() -> &'static rustybuzz::Face<'static> {
    FACE.get_or_init(|| {
        let font_bytes = include_bytes!("../../../assets/fonts/kalpurush-pua.ttf");
        rustybuzz::Face::from_slice(font_bytes, 0).expect("Failed to load kalpurush font")
    })
}

pub fn has_bengali(text: &str) -> bool {
    text.chars().any(|c| ('\u{0980}'..='\u{09FF}').contains(&c))
}

pub fn shape_bengali(text: &str) -> String {
    if !has_bengali(text) {
        return text.to_string();
    }

    let face = get_bengali_face();
    let mut buffer = rustybuzz::UnicodeBuffer::new();
    buffer.push_str(text);
    buffer.set_script(rustybuzz::script::BENGALI);
    buffer.set_direction(rustybuzz::Direction::LeftToRight);

    let output = rustybuzz::shape(face, &[], buffer);
    let glyph_infos = output.glyph_infos();

    let mut result = String::with_capacity(glyph_infos.len() * 3);
    for info in glyph_infos {
        if info.glyph_id > 0 {
            if let Some(ch) = char::from_u32(0xE000 + info.glyph_id) {
                result.push(ch);
                continue;
            }
        }
        let cluster = info.cluster as usize;
        if let Some(ch) = text.get(cluster..).and_then(|s| s.chars().next()) {
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bengali_sentences() {
        let test_cases = [
            "পুরাতন মোটরসাইকেল ক্রয় করবেন_ জেনে নিন বিস্তারিত.mp4",
            "বাংলা বর্ণমালা ও যুক্তবর্ণ",
            "আন্তর্জাতিক মাতৃভাষা দিবস",
            "বিজ্ঞান ও প্রযুক্তি",
            "শ্রেষ্ঠ শিক্ষক ও ছাত্র",
            "শান্তি ও শৃঙ্খলা",
            "ঢাকা বিশ্ববিদ্যালয়",
            "Test 123 - পুরাতন মোটরসাইকেল.mkv",
        ];

        for case in test_cases {
            let shaped = shape_bengali(case);
            assert!(!shaped.is_empty());
            println!("Original: {} -> Shaped len: {} chars", case, shaped.chars().count());
        }
    }
}

#[cfg(test)]
#[cfg(test)]
#[cfg(test)]
mod egui_test {
    use super::*;
    use eframe::egui;

    #[test]
    fn test_egui_font_load() {
        let ctx = egui::Context::default();
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "noto_bengali".to_owned(),
            egui::FontData::from_static(include_bytes!("../../../assets/fonts/kalpurush-pua.ttf")),
        );
        fonts.families.entry(egui::FontFamily::Proportional).or_default().push("noto_bengali".to_owned());
        ctx.set_fonts(fonts);
        
        let _ = ctx.run(Default::default(), |ctx| {
            let text = shape_bengali("পুরাতন মোটরসাইকেল ক্রয় করবেন_ জেনে নিন বিস্তারিত.mp4");
            let galley = ctx.fonts(|f| {
                f.layout_no_wrap(text, egui::FontId::proportional(14.0), egui::Color32::WHITE)
            });
            println!("Layout success! Galley size: {:?}", galley.size());
            assert!(galley.size().x > 100.0);
        });
    }
}
