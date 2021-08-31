const GEO_DATA: &[u8; 61914533] = include_bytes!("../geo_ip.dat");
fn with_iso_code<'a>(db: Arc<Reader<&[u8; 61914533]>>) -> impl Filter<Extract = (Option<String>,), Error = Rejection> + Clone + '_ {
    let ret = filters::header::optional::<IpAddr>("x-forwarded-for")
    .map(move |ip| {
        match ip {
            Some(val) => match db.lookup::<City>(val) {
                Ok(City {country: Some(country), ..}) => match country.iso_code { Some(iso) => Some(String::from(iso)), _ => None},
                _ => None
            },
            _ => None
        }
    });
    ret
}
