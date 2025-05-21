use tinycolors::{hsl, hsv, okhsl, oklab, rgb, srgb};

#[test]
fn rgb_to_srgb() {
    let color = rgb {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    assert_eq!(
        srgb {
            r: 1.0,
            g: 1.0,
            b: 1.0
        },
        srgb::from(color)
    );
}

// #[test]
// fn hsl_to_srgb() {
//     todo!()
// }
//
// #[test]
// fn hsv_to_srgb() {
//     todo!();
// }

#[test]
fn srgb_to_rgb() {
    let color = srgb {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };

    assert_eq!(
        rgb {
            r: 1.0,
            g: 0.0,
            b: 0.0
        },
        rgb::from(color)
    );

    let color = srgb {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };

    assert_eq!(
        rgb {
            r: 0.0,
            g: 1.0,
            b: 0.0
        },
        rgb::from(color)
    );

    let color = srgb {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };

    assert_eq!(
        rgb {
            r: 0.0,
            g: 0.0,
            b: 1.0
        },
        rgb::from(color)
    );

    let color = srgb {
        r: 1.0,
        g: 1.0,
        b: 0.0,
    };

    assert_eq!(
        rgb {
            r: 1.0,
            g: 1.0,
            b: 0.0
        },
        rgb::from(color)
    );

    let color = srgb {
        r: 0.5,
        g: 0.5,
        b: 0.5,
    };

    assert_eq!(
        rgb {
            r: 0.21404114,
            g: 0.21404114,
            b: 0.21404114
        },
        rgb::from(color)
    );
}

// #[test]
// fn oklab_to_rgb() {
//     todo!();
// }

#[test]
fn rgb_to_oklab() {
    let color = rgb {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };

    assert_eq!(
        oklab {
            l: 0.7016732,
            a: 0.27456677,
            b: -0.16915637
        },
        oklab::from(color)
    );
}

#[test]
fn okhsl_to_oklab() {
    let color = okhsl {
        h: 0.0,
        s: 0.8,
        l: 0.8,
    };

    println!("okhsl to srgb: {:?}", srgb::from(color));

    assert_eq!(
        oklab {
            l: 0.8281325,
            a: 0.091876104,
            b: 0.0
        },
        oklab::from(color)
    );
}

// #[test]
// fn okhsv_to_oklab() {
//     todo!();
// }
//
// #[test]
// fn oklab_to_okhsl() {
//     todo!();
// }
//
// #[test]
// fn oklab_to_okhsv() {
//     todo!();
// }

#[test]
fn srgb_to_hsl() {
    let color = srgb {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };

    assert_eq!(
        hsl {
            h: 0.0,
            s: 1.0,
            l: 0.5
        },
        hsl::from(color)
    );

    let color = srgb {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };

    assert_eq!(
        hsl {
            h: (1.0 / 3.0),
            s: 1.0,
            l: 0.5
        },
        hsl::from(color)
    );

    let color = srgb {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };

    assert_eq!(
        hsl {
            h: (2.0 / 3.0),
            s: 1.0,
            l: 0.5
        },
        hsl::from(color)
    );

    let color = srgb {
        r: 1.0,
        g: 1.0,
        b: 0.0,
    };

    assert_eq!(
        hsl {
            h: 1.0 / 6.0,
            s: 1.0,
            l: 0.5
        },
        hsl::from(color)
    );

    let color = srgb {
        r: 0.5,
        g: 0.5,
        b: 0.5,
    };

    assert_eq!(
        hsl {
            h: 0.0,
            s: 0.0,
            l: 0.5
        },
        hsl::from(color)
    );
}

#[test]
fn srgb_to_hsv() {
    let color = srgb {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };

    assert_eq!(
        hsv {
            h: 0.0,
            s: 1.0,
            v: 1.0
        },
        hsv::from(color)
    );

    let color = srgb {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };

    assert_eq!(
        hsv {
            h: (1.0 / 3.0),
            s: 1.0,
            v: 1.0
        },
        hsv::from(color)
    );

    let color = srgb {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };

    assert_eq!(
        hsv {
            h: (2.0 / 3.0),
            s: 1.0,
            v: 1.0
        },
        hsv::from(color)
    );

    let color = srgb {
        r: 1.0,
        g: 1.0,
        b: 0.0,
    };

    assert_eq!(
        hsv {
            h: 1.0 / 6.0,
            s: 1.0,
            v: 1.0
        },
        hsv::from(color)
    );

    let color = srgb {
        r: 0.5,
        g: 0.5,
        b: 0.5,
    };

    assert_eq!(
        hsv {
            h: 0.0,
            s: 0.0,
            v: 0.5
        },
        hsv::from(color)
    );
}
