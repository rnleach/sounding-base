extern crate sounding_base;

use sounding_base::{Sounding, MissingData};

#[test]
fn test_validate() {

    let snd = create_valid_test_sounding();
    assert!(snd.validate().is_ok());

    let snd = create_invalid_test_sounding();
    assert!(snd.validate().is_err());
}

fn create_valid_test_sounding() -> Sounding {
    Sounding {
        num: 1.into(),
        valid_time: None,
        lead_time: 0.into(),
        lat: 45.0.into(),
        lon: (-115.0).into(),
        elevation: 1023.0.into(),

        show: (-2.0).into(),
        li: (-2.0).into(),
        swet: 35.0.into(),
        kinx: 45.0.into(),
        lclp: 850.0.into(),
        pwat: 2.0.into(),
        totl: 55.0.into(),
        cape: 852.0.into(),
        lclt: 12.0.into(),
        cins: (-200.0).into(),
        eqlv: 222.0.into(),
        lfc: 800.0.into(),
        brch: 1.2.into(),
        hain: 6.into(),

        pressure: vec![
            840.0.into(),
            800.0.into(),
            700.0.into(),
            500.0.into(),
            300.0.into(),
            250.0.into(),
            200.0.into(),
            100.0.into(),
        ],
        temperature: vec![
            20.0.into(),
            15.0.into(),
            2.0.into(),
            (-10.0).into(),
            (-20.0).into(),
            (-30.0).into(),
            (-50.0).into(),
            (-45.0).into(),
        ],
        wet_bulb: vec![
            20.0.into(),
            14.0.into(),
            1.0.into(),
            (-11.0).into(),
            (-25.0).into(),
            (-39.0).into(),
            (-58.0).into(),
            (-60.0).into(),
        ],
        dew_point: vec![
            20.0.into(),
            13.0.into(),
            0.0.into(),
            (-12.0).into(),
            (-27.0).into(),
            (-45.0).into(),
            (-62.0).into(),
            (-80.0).into(),
        ],
        theta_e: vec![],
        direction: vec![
            0.0.into(),
            40.0.into(),
            80.0.into(),
            120.0.into(),
            160.0.into(),
            200.0.into(),
            240.0.into(),
            280.0.into(),
        ],
        speed: vec![
            5.0.into(),
            10.0.into(),
            15.0.into(),
            12.0.into(),
            27.0.into(),
            45.0.into(),
            62.0.into(),
            80.0.into(),
        ],
        omega: vec![],
        height: vec![
            100.0.into(),
            200.0.into(),
            300.0.into(),
            400.0.into(),
            500.0.into(),
            650.0.into(),
            700.0.into(),
            800.0.into(),
        ],
        cloud_fraction: vec![
            100.0.into(),
            85.0.into(),
            70.0.into(),
            50.0.into(),
            30.0.into(),
            25.0.into(),
            20.0.into(),
            10.0.into(),
        ],

        mslp: 1014.0.into(),
        station_pres: 847.0.into(),
        low_cloud: f32::MISSING.into(),
        mid_cloud: f32::MISSING.into(),
        hi_cloud: f32::MISSING.into(),
        uwind: 0.0.into(),
        vwind: 0.0.into(),
    }
}

fn create_invalid_test_sounding() -> Sounding {
    Sounding {
        hain: 1.into(),
        ..create_valid_test_sounding()
    }
}
