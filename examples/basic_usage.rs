use epcgen::{Builder, CharacterSet, Identification, Version};
use image::Luma;
use qrcode::QrCode;

fn main() {
    let builder = Builder::new();
    let builder = builder.version(Version::V1);
    let builder = builder.character_set(CharacterSet::UTF8);
    let builder = builder.identification(Identification::Sct);
    let builder = builder.bic("GENODEF1SLR".to_string());
    let builder = builder.beneficiary("Codeberg e.V.".to_string());
    let builder = builder.iban("DE90 8306 5408 0004 1042 42".to_string());
    let builder = builder.amount(10.00);
    let builder = builder.remittance("for the good cause".to_string());
    let epc = builder.build();
    let epc = epc.expect("example should work");
    let qrcode_payload = epc.to_string();

    let code = QrCode::new(qrcode_payload).unwrap();

    // Render the data into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("./examples_basic_usage_qrcode.png").unwrap();
}
