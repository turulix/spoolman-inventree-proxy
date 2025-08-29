// use crate::routes::filament::Filament;
// use crate::routes::ApiResult;
// use actix_web::get;
// use actix_web::web::Json;
// use chrono::NaiveTime;
// use inventree::part::PartListQuery;
// use inventree::InventreeApiClient;
// use utoipa::ToSchema;
//
// #[utoipa::path(
//     tags = ["Filament"],
//     responses(
//         (status = 200, description = "Return a health check.", body = Vec<Filament>)
//     )
// )]
// #[get("/filament")]
// async fn find_filament_route() -> ApiResult<Json<Vec<Filament>>> {
//     let client = InventreeApiClient::new(
//         "https://inv.turulix.de",
//         "inv-53d98420e0de9495ad96414532a7f078ed7235e7-20250828",
//     );
//
//     let res = client
//         .part()
//         .list(&PartListQuery {
//             category: Some(31),
//             parameters: Some(true),
//         })
//         .await
//         .unwrap();
//
//     Ok(Json(
//         res.into_iter()
//             .map(|part| Filament {
//                 id: part.pk,
//                 registered: part.creation_date.and_time(NaiveTime::default()).and_utc(),
//                 name: Some(part.name),
//                 vendor: None,
//                 material: Some("PLA".to_string()),
//                 price: None,
//                 density: select_parameter_numeric(
//                     &part.parameters,
//                     "3DPrint Filament Density",
//                 ).unwrap_or(0.0),
//                 diameter: select_parameter_numeric(
//                     &part.parameters,
//                     "3DPrint Filament Diameter",
//                 ).unwrap_or(0.0),
//                 weight: None,
//                 spool_weight: None,
//                 article_number: None,
//                 comment: None,
//                 settings_extruder_temp: select_parameter_numeric(
//                     &part.parameters,
//                     "3DPrint Extruder Temperature",
//                 ),
//                 settings_bed_temp: select_parameter_numeric(
//                     &part.parameters,
//                     "3DPrint Bed Temperature",
//                 ),
//                 color_hex: None,
//                 multi_color_hexes: None,
//                 multi_color_direction: None,
//                 external_id: part.ipn,
//                 extras: Default::default(),
//             })
//             .collect(),
//     ))
// }
//
// fn select_parameter_numeric(
//     parameters: &Vec<inventree::part::PartParameter>,
//     name: &str,
// ) -> Option<f64> {
//     parameters
//         .iter()
//         .find(|x| x.template_detail.name == name)
//         .and_then(|x| x.data_numeric)
// }
