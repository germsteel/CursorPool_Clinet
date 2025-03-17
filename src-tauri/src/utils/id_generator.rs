use rand::RngCore;
use sha2::{Digest, Sha256, Sha512};
use std::collections::HashMap;
use uuid::Uuid;

pub fn generate_new_ids() -> HashMap<String, String> {
    let mut rng = rand::thread_rng();
    let mut random_bytes_32 = [0u8; 32];
    let mut random_bytes_64 = [0u8; 64];
    rng.fill_bytes(&mut random_bytes_32);
    rng.fill_bytes(&mut random_bytes_64);

    let mut ids = HashMap::new();

    // 生成设备ID
    ids.insert(
        "telemetry.devDeviceId".to_string(),
        Uuid::new_v4().to_string(),
    );

    // 生成MAC机器ID
    let mac_id = Sha512::digest(random_bytes_64);
    ids.insert("telemetry.macMachineId".to_string(), hex::encode(mac_id));

    // 生成机器ID
    let machine_id = Sha256::digest(random_bytes_32);
    ids.insert("telemetry.machineId".to_string(), hex::encode(machine_id));

    // 生成SQM ID
    ids.insert(
        "telemetry.sqmId".to_string(),
        format!("{{{}}}", Uuid::new_v4().to_string().to_uppercase()),
    );

    ids
}
