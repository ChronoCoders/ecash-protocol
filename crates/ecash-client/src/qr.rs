use crate::error::{ClientError, Result};
use ecash_core::Token;
use image::{ImageBuffer, Luma};
use qrcode::QrCode;

pub struct QrCodeGenerator;

impl QrCodeGenerator {
    pub fn generate_token_qr(token: &Token) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>> {
        let token_json = serde_json::to_string(token)
            .map_err(|e| ClientError::QrCode(format!("Serialization failed: {}", e)))?;
        
        let code = QrCode::new(token_json.as_bytes())
            .map_err(|e| ClientError::QrCode(format!("QR generation failed: {}", e)))?;
        
        let image = code.render::<Luma<u8>>()
            .min_dimensions(200, 200)
            .build();
        
        Ok(image)
    }

    pub fn generate_payment_request_qr(amount: u64, recipient: &str) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>> {
        let payment_data = serde_json::json!({
            "type": "payment_request",
            "amount": amount,
            "recipient": recipient,
        });
        
        let payment_json = serde_json::to_string(&payment_data)
            .map_err(|e| ClientError::QrCode(format!("Serialization failed: {}", e)))?;
        
        let code = QrCode::new(payment_json.as_bytes())
            .map_err(|e| ClientError::QrCode(format!("QR generation failed: {}", e)))?;
        
        let image = code.render::<Luma<u8>>()
            .min_dimensions(200, 200)
            .build();
        
        Ok(image)
    }

    pub fn save_qr_png(image: &ImageBuffer<Luma<u8>, Vec<u8>>, path: &str) -> Result<()> {
        image.save(path)
            .map_err(|e| ClientError::QrCode(format!("Failed to save image: {}", e)))?;
        Ok(())
    }
}
