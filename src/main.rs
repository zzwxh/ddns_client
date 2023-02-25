use reqwest::Response;

#[tokio::main]
async fn main() {
    let mut ip = String::new();
    loop {
        if let Some(ip_new) = get_ip2() {
            if ip != ip_new {
                ip = ip_new;
                println!("changed {}",ip);
                while {
                    let r = send_ip_change(&ip).await;
                    match r {
                        Some(req) => {
                            println!("REQ:{}",req.status());
                            false
                        }
                        None => true
                    }
                } {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    println!("RETRYING");
                }
            }else {
                println!("unchanged {}",ip);
            }
        }else {
            println!("no connection");
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

async fn send_ip_change(ip:&str) -> Option<Response> {
    const ZONE:&'static str = "zzwxh.dynv6.net";
    const TOKEN:&'static str = "VLzA8DWehCy3wf3x7q1BwjNiHFfMMy";
    let uri = format!("https://dynv6.com/api/update?zone={ZONE}&token={TOKEN}&ipv6={ip}");
    match reqwest::get(uri).await {
        Ok(r) => Some(r),
        Err(e) => {
            println!("ERROR:{e}");
            None
        },
    }
}

// fn get_ip() -> Option<String> {
//     const ADAPTER:&'static str = "Realtek PCIe GbE Family Controller";
//     let adapter = ipconfig::get_adapters().unwrap()
//         .into_iter()
//         .find(|adapter|adapter.description() == ADAPTER)
//         .expect("ADAPTER error");
//     let ip = adapter.ip_addresses().into_iter()
//         .next()
//         .expect("IP error")
//         .to_string();
//     if ip.starts_with("2409:8a20:251") {
//         Some(ip)
//     }else {
//         None
//     }
// }

fn get_ip2() -> Option<String> {
    const ADAPTER:&'static str = "Realtek PCIe GbE Family Controller";
    let adapter = ipconfig::get_adapters().unwrap()
        .into_iter()
        .find(|adapter|adapter.description() == ADAPTER)
        .expect("ADAPTER error");
    if let ipconfig::OperStatus::IfOperStatusUp = adapter.oper_status() {
        let ip = adapter.ip_addresses().into_iter()
            .next()
            .expect("IP error")
            .to_string();
        if ip.starts_with("fe80") {
            println!("starts with FE80");
            None
        }else {
            Some(ip)
        }
    }else {
        println!("oper_status not UP");
        None
    }
}
