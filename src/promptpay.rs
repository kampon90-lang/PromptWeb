
pub fn promptpay_payload(pp_id: &str, amount: Option<f64>) -> Result<String, String> {
    let paycheck = build_promptpay_payload(pp_id, amount)?;
    Ok(paycheck)
}

fn tlv(tag: &str, value: &str) -> String {
    format!("{}{:02}{}", tag, value.len(), value)
}

fn crc16_ccitt(payload: &str) -> String{
    let mut crc = 0xFFFF;
    for byte in payload.bytes() {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    format!("{:04X}", crc)
}

fn normalize_promptpay_id(raw: &str) -> Result<(String, String), String> {
    let s_raw = raw.replace("-", "");
    let s = s_raw.trim().to_string();

    if s.len() == 10 && s.starts_with("0") && s.chars().all(|c| c.is_digit(10)){ // Phone number
        let formatted = format!("0066{}", &s[1..]);
        return Ok(("01".to_string(), formatted));
    }

    if s.len() == 13 && s.chars().all(|c| c.is_digit(10)){
        return Ok(("02".to_string(), s.to_string()));
    }

    if s.len() == 15 && s.chars().all(|c| c.is_digit(10)) {
        return Ok(("03".to_string(), s.to_string()));
    }

    Err("Invalid Promptpay ID Use ID or Phone Number".to_string())
}

fn build_promptpay_payload(pp_id: &str, amount: Option<f64>) -> Result<String, String> {
    let (subtag, id_value) = normalize_promptpay_id(pp_id)?;

    // Merchant acount information (Tag 29)
    // AID A000000677010111
    let mai = format!("{}{}", 
        tlv("00", "A000000677010111"),
        tlv(&subtag, &id_value)
    );

    // Point of initialtion method (Tag 01)
    // 11 = Statuc (no amount), 12 = dynamic (with amount) 
    let poi_method = if amount.is_some() {"12"} else {"11"};

    // Create payload
    let mut payload = String::new();
    payload.push_str(&tlv("00", "01"));  // Payload format
    payload.push_str(&tlv("01", poi_method)); // Point of initiation
    payload.push_str(&tlv("29", &mai)); // Merchant account information
    payload.push_str(&tlv("53", "764")); // Country code and currentcy, THB = 76

    if let Some(amt) = amount {
        let amt_str = format!("{:.2}", amt);
        payload.push_str(&tlv("54", &amt_str)); // Transection amount
    }

    payload.push_str(&tlv("58", "TH")); // Country Code TH

    // CRC (Tag 63) - add 6304 then CRC payload to checksume
    let payload_crc = format!("{}6304", payload);
    let crc = crc16_ccitt(&payload_crc);

    Ok(format!("{}{}", payload_crc, crc)) // payload + 6304 + crc of whole string
}




