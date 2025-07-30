use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug)]
pub struct FontPayload {
    pub name: String,
    pub data: Vec<u8>,
    pub uuid: String,
}

#[derive(Debug)]
pub struct MobileConfig {
    pub payload_identifier: String,
    pub payload_display_name: String,
    pub payload_uuid: String,
    pub payload_version: i32,
    pub fonts: Vec<FontPayload>,
}

impl MobileConfig {
    pub fn new(display_name: String, identifier: String) -> Self {
        Self {
            payload_identifier: identifier,
            payload_display_name: display_name,
            payload_uuid: Uuid::new_v4().to_string(),
            payload_version: 1,
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

        let font_payload = FontPayload {
            name,
            data,
            uuid: Uuid::new_v4().to_string(),
        };

        self.fonts.push(font_payload);
        Ok(())
    }

    pub fn generate_xml(&self) -> String {
        let mut xml = String::new();

        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n");
        xml.push_str("<plist version=\"1.0\">\n");
        xml.push_str("<dict>\n");
        xml.push_str("  <key>PayloadContent</key>\n");
        xml.push_str("  <array>\n");

        for font in &self.fonts {
            let base64_data = general_purpose::STANDARD.encode(&font.data);
            xml.push_str("    <dict>\n");
            xml.push_str("      <key>PayloadContent</key>\n");
            xml.push_str("      <data>");

            // Format base64 data as multi-line with 64 characters per line
            for (i, chunk) in base64_data.as_bytes().chunks(64).enumerate() {
                if i > 0 {
                    xml.push_str("\n        ");
                }
                xml.push_str(std::str::from_utf8(chunk).unwrap());
            }

            xml.push_str("</data>\n");
            xml.push_str("      <key>PayloadIdentifier</key>\n");
            xml.push_str(&format!(
                "      <string>{}.{}.fontpayload</string>\n",
                self.payload_identifier, font.uuid
            ));
            xml.push_str("      <key>PayloadType</key>\n");
            xml.push_str("      <string>com.apple.font</string>\n");
            xml.push_str("      <key>PayloadUUID</key>\n");
            xml.push_str(&format!("      <string>{}</string>\n", font.uuid));
            xml.push_str("      <key>PayloadVersion</key>\n");
            xml.push_str("      <integer>1</integer>\n");
            xml.push_str("      <key>Name</key>\n");
            xml.push_str(&format!("      <string>{}</string>\n", font.name));
            xml.push_str("    </dict>\n");
        }

        xml.push_str("  </array>\n");
        xml.push_str("  <key>PayloadDisplayName</key>\n");
        xml.push_str(&format!(
            "  <string>{}</string>\n",
            self.payload_display_name
        ));
        xml.push_str("  <key>PayloadIdentifier</key>\n");
        xml.push_str(&format!("  <string>{}</string>\n", self.payload_identifier));
        xml.push_str("  <key>PayloadType</key>\n");
        xml.push_str("  <string>Configuration</string>\n");
        xml.push_str("  <key>PayloadUUID</key>\n");
        xml.push_str(&format!("  <string>{}</string>\n", self.payload_uuid));
        xml.push_str("  <key>PayloadVersion</key>\n");
        xml.push_str(&format!("  <integer>{}</integer>\n", self.payload_version));
        xml.push_str("</dict>\n");
        xml.push_str("</plist>");

        xml
    }

    pub fn save_to_file(&self, output_path: &Path) -> Result<()> {
        let xml = self.generate_xml();
        fs::write(output_path, xml)
            .map_err(|e| anyhow!("Failed to write mobileconfig file: {}", e))?;
        Ok(())
    }
}
