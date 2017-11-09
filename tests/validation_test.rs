extern crate sounding_base;

use sounding_base::{Sounding, OptionVal};

#[test]
fn test_validate() {

    let snd = create_valid_test_sounding();
    assert!(snd.validate().is_ok());

    let snd = create_invalid_test_sounding();
    assert!(snd.validate().is_err());
}

fn create_valid_test_sounding() -> Sounding {
    use sounding_base::Profile::*;
    use sounding_base::Index::*;
    use sounding_base::Surface::*;

    Sounding::new()
        .set_station_num(1)
        .set_valid_time(None)
        .set_lead_time(0)
        .set_location(45.0, -115.0, 1023.0)
        .set_index(Showalter, -2.0)
        .set_index(LI, -2.0)
        .set_index(SWeT, 35.0)
        .set_index(K, 45.0)
        .set_index(LCL, 850.0)
        .set_index(PWAT, 2.0)
        .set_index(TotalTotals, 55.0)
        .set_index(CAPE, 852.0)
        .set_index(LCLTemperature, 290.0)
        .set_index(CIN, -200.0)
        .set_index(EquilibrimLevel, 222.0)
        .set_index(LFC, 800.0)
        .set_index(BulkRichardsonNumber, 1.2)
        .set_profile(
            Pressure,
            vec![
                OptionVal::from(840.0),
                OptionVal::from(800.0),
                OptionVal::from(700.0),
                OptionVal::from(500.0),
                OptionVal::from(300.0),
                OptionVal::from(250.0),
                OptionVal::from(200.0),
                OptionVal::from(100.0),
            ],
        )
        .set_profile(
            Temperature,
            vec![
                OptionVal::from(20.0),
                OptionVal::from(15.0),
                OptionVal::from(2.0),
                OptionVal::from((-10.0)),
                OptionVal::from((-20.0)),
                OptionVal::from((-30.0)),
                OptionVal::from((-50.0)),
                OptionVal::from((-45.0)),
            ],
        )
        .set_profile(
            WetBulb,
            vec![
                OptionVal::from(20.0),
                OptionVal::from(14.0),
                OptionVal::from(1.0),
                OptionVal::from((-11.0)),
                OptionVal::from((-25.0)),
                OptionVal::from((-39.0)),
                OptionVal::from((-58.0)),
                OptionVal::from((-60.0)),
            ],
        )
        .set_profile(
            DewPoint,
            vec![
                OptionVal::from(20.0),
                OptionVal::from(13.0),
                OptionVal::from(0.0),
                OptionVal::from((-12.0)),
                OptionVal::from((-27.0)),
                OptionVal::from((-45.0)),
                OptionVal::from((-62.0)),
                OptionVal::from((-80.0)),
            ],
        )
        .set_profile(
            WindDirection,
            vec![
                OptionVal::from(0.0),
                OptionVal::from(40.0),
                OptionVal::from(80.0),
                OptionVal::from(120.0),
                OptionVal::from(160.0),
                OptionVal::from(200.0),
                OptionVal::from(240.0),
                OptionVal::from(280.0),
            ],
        )
        .set_profile(
            WindSpeed,
            vec![
                OptionVal::from(5.0),
                OptionVal::from(10.0),
                OptionVal::from(15.0),
                OptionVal::from(12.0),
                OptionVal::from(27.0),
                OptionVal::from(45.0),
                OptionVal::from(62.0),
                OptionVal::from(80.0),
            ],
        )
        .set_profile(
            GeopotentialHeight,
            vec![
                OptionVal::from(100.0),
                OptionVal::from(200.0),
                OptionVal::from(300.0),
                OptionVal::from(400.0),
                OptionVal::from(500.0),
                OptionVal::from(650.0),
                OptionVal::from(700.0),
                OptionVal::from(800.0),
            ],
        )
        .set_profile(
            CloudFraction,
            vec![
                OptionVal::from(100.0),
                OptionVal::from(85.0),
                OptionVal::from(70.0),
                OptionVal::from(50.0),
                OptionVal::from(30.0),
                OptionVal::from(25.0),
                OptionVal::from(20.0),
                OptionVal::from(10.0),
            ],
        )
        .set_surface_value(MSLP, 1014.0)
        .set_surface_value(StationPressure, 847.0)
        .set_surface_value(UWind, 0.0)
        .set_surface_value(VWind, 0.0)
}

fn create_invalid_test_sounding() -> Sounding {
    create_valid_test_sounding().set_index(sounding_base::Index::PWAT, -5.1)
}
