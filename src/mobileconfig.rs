use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn generate_uuid() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let mut hasher = DefaultHasher::new();
    timestamp.hash(&mut hasher);
    let hash = hasher.finish();

    format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        (hash >> 32) as u32,
        (hash >> 16) as u16,
        (hash >> 8) as u16,
        (hash & 0xFF) as u16,
        hash
    )
}

#[derive(Debug)]
pub struct FontPayload {
    pub name: String,
    pub data: Vec<u8>,
    pub uuid: String,
    pub identifier: String,
}

#[derive(Debug)]
pub struct MobileConfig {
    pub payload_identifier: String,
    pub payload_display_name: String,
    pub payload_uuid: String,
    pub consent_text: String,
    pub fonts: Vec<FontPayload>,
}

impl MobileConfig {
    pub fn new(display_name: String, identifier: String) -> Self {
        Self {
            payload_identifier: identifier,
            payload_display_name: display_name,
            payload_uuid: generate_uuid(),
            consent_text: "This profile will install custom fonts on your iOS device.".to_string(),
            fonts: Vec::new(),
        }
    }

    pub fn add_font(&mut self, font_path: &Path, font_name: Option<String>) -> Result<()> {
        let name = font_name.unwrap_or_else(|| {
            font_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string()
        });

        let data = fs::read(font_path)
            .map_err(|e| anyhow!("Failed to read font file {}: {}", font_path.display(), e))?;

        let font_uuid = generate_uuid();
        let font_identifier = format!("{}.{}.fontpayload", self.payload_identifier, font_uuid);

        let font_payload = FontPayload {
            name,
            data,
            uuid: font_uuid,
            identifier: font_identifier,
        };

        self.fonts.push(font_payload);
        Ok(())
    }

    pub fn generate_xml(&self) -> String {
        let mut xml = String::new();

        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n");
        xml.push_str("<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\"\n");
        xml.push_str("    \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n");
        xml.push_str("<plist version=\"1.0\">\n");
        xml.push_str("<dict>\n");

        // ConsentText
        xml.push_str("  <key>ConsentText</key>\n");
        xml.push_str("  <dict>\n");
        xml.push_str("    <key>default</key>\n");
        xml.push_str(&format!("    <string>{}</string>\n", self.consent_text));
        xml.push_str("  </dict>\n");

        // PayloadContent
        xml.push_str("  <key>PayloadContent</key>\n");
        xml.push_str("  <array>\n");

        for font in &self.fonts {
            let base64_data = general_purpose::STANDARD.encode(&font.data);
            xml.push_str("      <dict>\n");
            xml.push_str("        <key>Font</key>\n");
            xml.push_str("        <data>");

            // Format base64 data as multi-line with proper indentation
            for (i, chunk) in base64_data.as_bytes().chunks(64).enumerate() {
                if i > 0 {
                    xml.push_str("\n        ");
                }
                xml.push_str(std::str::from_utf8(chunk).unwrap());
            }

            xml.push_str("</data>\n");
            xml.push_str("        <key>Name</key>\n");
            xml.push_str(&format!("        <string>{}</string>\n", font.name));
            xml.push_str("        <key>PayloadDescription</key>\n");
            xml.push_str("        <string>Configures Font settings</string>\n");
            xml.push_str("        <key>PayloadDisplayName</key>\n");
            xml.push_str("        <string>Fonts</string>\n");
            xml.push_str("        <key>PayloadIdentifier</key>\n");
            xml.push_str(&format!("        <string>{}</string>\n", font.identifier));
            xml.push_str("        <key>PayloadType</key>\n");
            xml.push_str("        <string>com.apple.font</string>\n");
            xml.push_str("        <key>PayloadUUID</key>\n");
            xml.push_str(&format!("        <string>{}</string>\n", font.uuid));
            xml.push_str("        <key>PayloadVersion</key>\n");
            xml.push_str("        <integer>1</integer>\n");
            xml.push_str("      </dict>\n");
        }

        xml.push_str("  </array>\n");

        // PayloadDisplayName
        xml.push_str("  <key>PayloadDisplayName</key>\n");
        xml.push_str(&format!(
            "  <string>{}</string>\n",
            self.payload_display_name
        ));

        // PayloadIdentifier
        xml.push_str("  <key>PayloadIdentifier</key>\n");
        xml.push_str(&format!("  <string>{}</string>\n", self.payload_identifier));

        // PayloadRemovalDisallowed
        xml.push_str("  <key>PayloadRemovalDisallowed</key>\n");
        xml.push_str("  <false />\n");

        // PayloadType
        xml.push_str("  <key>PayloadType</key>\n");
        xml.push_str("  <string>Configuration</string>\n");

        // PayloadUUID
        xml.push_str("  <key>PayloadUUID</key>\n");
        xml.push_str(&format!("  <string>{}</string>\n", self.payload_uuid));

        // PayloadVersion
        xml.push_str("  <key>PayloadVersion</key>\n");
        xml.push_str("  <integer>1</integer>\n");

        xml.push_str("</dict>\n");
        xml.push_str("</plist>\n");

        xml
    }

    pub fn save_to_file(&self, output_path: &Path) -> Result<()> {
        if self.fonts.is_empty() {
            return Err(anyhow!("No font data has been added to the configuration"));
        }

        let xml = self.generate_xml();
        fs::write(output_path, xml)
            .map_err(|e| anyhow!("Failed to write mobileconfig file: {}", e))?;
        Ok(())
    }
}
