use base58::{FromBase58, ToBase58};
use uuid::Uuid;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

fn main() -> Result<()> {
    let uuid = Uuid::now_v7();
    println!("{:>12} ({}): {uuid}", "uuid", uuid.to_string().len());

    let b64 = data_encoding::BASE64.encode(uuid.as_bytes());
    println!("{:>12} ({}): {b64}", "b64", b64.len());

    let b64u = data_encoding::BASE64_NOPAD.encode(uuid.as_bytes());
    println!("{:>12} ({}): {b64u}", "b64u", b64u.len());

    let b64m = data_encoding::BASE64_MIME.encode(uuid.as_bytes());
    println!("{:>12} ({}): {b64m}", "b64m", b64m.len());

    let b32 = data_encoding::BASE32.encode(uuid.as_bytes());
    println!("{:>12} ({}): {b32}", "b32", b32.len());

    let b32u = data_encoding::BASE32_NOPAD.encode(uuid.as_bytes());
    println!("{:>12} ({}): {b32u}", "b32u", b32u.len());

    let b32dnssec = data_encoding::BASE32_DNSSEC.encode(uuid.as_bytes());
    println!("{:>12} ({}): {b32dnssec}", "b32dnssec", b32dnssec.len());

    let b32dncurve = data_encoding::BASE32_DNSCURVE.encode(uuid.as_bytes());
    println!("{:>12} ({}): {b32dncurve}", "b32dncurve", b32dncurve.len());

    let b32h = data_encoding::BASE32HEX.encode(uuid.as_bytes());
    println!("{:>12} ({}): {b32h}", "b32h", b32h.len());

    let b32hex = data_encoding::BASE32HEX_NOPAD.encode(uuid.as_bytes());
    println!("{:>12} ({}): {b32hex}", "b32hex", b32hex.len());

    let b58 = uuid.as_bytes().to_base58();
    println!("{:>12} ({}): {b58}", "b58", b58.len()); // if you want some code to be inputted in
                                                      // the real world then you might use base58.
    let b58_encoding = b58.from_base58().unwrap();
    println!(
        "{:<12} ({}) : {b58_encoding:?}",
        "b58 encoding",
        b58_encoding.len()
    );

    Ok(())
}
