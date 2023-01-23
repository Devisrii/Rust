use actix_multipart::Multipart;
use actix_web::{post, App, HttpServer, Responder};
use futures_util::stream::StreamExt as _;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessControllerEvent {
    device_name: String,
    major_event_type: u32,
    sub_event_type: u32,
    name: String,
    card_reader_kind: u32,
    card_reader_no: u32,
    verify_no: u32,
    employee_no_string: String,
    serial_no: u32,
    user_type: String,
    current_verify_mode: String,
    front_serial_no: u32,
    attendance_status: String,
    label: String,
    status_value: u32,
    mask: String,
    pure_pwd_verify_enable: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventInfo {
    ip_address: String,
    port_no: u32,
    protocol: String,
    mac_address: String,
    #[serde(rename = "channelID")]
    channel_id: u32,
    date_time: String,
    active_post_count: u32,
    event_type: String,
    event_state: String,
    event_description: String,
    #[serde(rename = "AccessControllerEvent")]
    access_controller_event: AccessControllerEvent
}

#[post("/index")]
async fn index(mut payload: Multipart) -> impl Responder {
    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let bytes = chunk.unwrap();
            // println!("-- CHUNK: \n{:?}", std::str::from_utf8(&chunk.unwrap()));
            // let data = "{\n\t\"ipAddress\":\t\"192.168.1.200\",\n\t\"portNo\":\t8080,\n\t\"protocol\":\t\"HTTP\",\n\t\"macAddress\":\t\"bc:9b:5e:26:0f:55\",\n\t\"channelID\":\t1,\n\t\"dateTime\":\t\"2023-01-21T12:53:09+05:30\",\n\t\"activePostCount\":\t1,\n\t\"eventType\":\t\"AccessControllerEvent\",\n\t\"eventState\":\t\"active\",\n\t\"eventDescription\":\t\"Access Controller Event\",\n\t\"AccessControllerEvent\":\t{\n\t\t\"deviceName\":\t\"Access Controller\",\n\t\t\"majorEventType\":\t5,\n\t\t\"subEventType\":\t38,\n\t\t\"name\":\t\"DEVISRI M\",\n\t\t\"cardReaderKind\":\t1,\n\t\t\"cardReaderNo\":\t1,\n\t\t\"verifyNo\":\t146,\n\t\t\"employeeNoString\":\t\"4\",\n\t\t\"serialNo\":\t513,\n\t\t\"userType\":\t\"normal\",\n\t\t\"currentVerifyMode\":\t\"cardOrFaceOrFp\",\n\t\t\"frontSerialNo\":\t512,\n\t\t\"attendanceStatus\":\t\"undefined\",\n\t\t\"label\":\t\"\",\n\t\t\"statusValue\":\t0,\n\t\t\"mask\":\t\"unknown\",\n\t\t\"purePwdVerifyEnable\":\ttrue\n\t}\n}";
            let data = std::str::from_utf8(&bytes).unwrap();
            // let json:Option<EventInfo> = serde_json::from_str(data).ok();
            if let Ok(event_info) = serde_json::from_str::<EventInfo>(data) {
                println!("event_info = {:?}", &event_info)
            }
            // println!("json = {:?}", &json);
        }
    }
    format!("success")
    // Ok(HttpResponse::Ok().into())
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new().service(index)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await

}
