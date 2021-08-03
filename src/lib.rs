///
/// # 旧暦計算プログラム
///  
use chrono::{Datelike, Local, NaiveDate};
use std::f64::consts::PI;


#[derive(Debug)]
pub struct Qreki {
    pub qy: i32,
    pub qm: u32,
    pub qd: u32,
    pub ql: bool,
    pub qr: &'static str
}

impl Qreki {
    pub fn new() -> Qreki {
        let date = Local::today().naive_local();
        let (qy, qm, qd, ql,qr) = calc_kyureki(date);
        Qreki {
            qy: qy,
            qm: qm,
            qd: qd,
            ql: ql,
            qr:qr
        }
    }
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Qreki {
        let date: NaiveDate = NaiveDate::from_ymd(year, month, day);
        let (qy, qm, qd, ql,qr) = calc_kyureki(date);
        Qreki {
            qy: qy,
            qm: qm,
            qd: qd,
            ql: ql,
            qr: qr
        }
    }

}

const K: f64 = PI / 180.0;
const TZ: f64 = 9.0 / 24.0; //JST

fn calc_kyureki(dt: NaiveDate) -> (i32, u32, u32, bool,&'static str) {
    let tm0 = dt.num_days_from_ce() as f64 + 1721424.0;
    let mut chu: [[f64; 2]; 4] = [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
    chu[0] = calc_tm(tm0, 90.0);
    for i in 1..4 {
        chu[i] = calc_tm(chu[i - 1][0] + 32.0, 30.0);
    }
    let mut saku: [f64; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
    saku[0] = calc_saku(chu[0][0]);
    for i in 1..5 {
        saku[i] = calc_saku(saku[i - 1] + 30.0);
        if (saku[i - 1] as i64 - saku[i] as i64).abs() <= 26 {
            saku[i] = calc_saku(saku[i - 1] + 35.0);
        }
    }
    if saku[1] as i64 <= chu[0][0] as i64 {
        for i in 0..4 {
            saku[i] = saku[i + 1];
        }
        saku[4] = calc_saku(saku[3] + 35.0)
    } else if saku[0] as i64 > chu[0][0] as i64 {
        for i in (1..5).rev() {
            saku[i] = saku[i - 1];
        }
        saku[0] = calc_saku(saku[0] - 27.0)
    }
    let mut lap = saku[4] as i64 <= chu[3][0] as i64;
    let mut m: [[i64; 3]; 5] = [[0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]];
    m[0][0] = (chu[0][1] / 30.0) as i64 + 2;
    m[0][1] = 0;
    m[0][2] = saku[0] as i64;
    for i in 1..5 {
        if lap && i != 1 {
            if ((chu[i - 1][0]) as i64 <= (saku[i - 1] as i64))
                || ((chu[i - 1][0]) as i64 >= (saku[i]) as i64)
            {
                m[i - 1][0] = m[i - 2][0];
                m[i - 1][1] = 1;
                m[i - 1][2] = saku[i - 1] as i64;
                lap = false;
            }
        }
        m[i][0] = m[i - 1][0] + 1;
        if m[i][0] > 12 {
            m[i][0] -= 12;
        }
        m[i][1] = 0;
        m[i][2] = saku[i] as i64;
    }
    let mut state = 0;
    let mut i = 0;
    for j in 0..5 {
        i = j;
        if (tm0 as i64) < (m[i][2]) {
            state = 1;
            break;
        } else if tm0 as i64 == m[i][2] {
            state = 2;
            break;
        }
    }
    if state == 1 {
        if i > 0 {
            i -= 1;
        }
    }
    let qm = m[i][0] as u32;
    let ql = m[i][1] > 0;
    let qd = (tm0 - m[i][2] as f64 + 1.0) as u32;
    let d = NaiveDate::from_num_days_from_ce((tm0 - 1721424.0) as i32);
    let mut qy = d.year();
    if qm > 9 && qm > d.month() {
        qy -= 1;
    }
    let qr = getrokuyou(qm,qd );
    return (qy, qm, qd , ql,qr);
}

fn calc_tm(tm: f64, deg: f64) -> [f64; 2] {
    let (mut tm2, mut tm1) = libm::modf(tm);
    tm2 -= TZ;
    let right = (tm2 + 0.5) / 36525.0;
    let left = (tm1 - 2451545.0) / 36525.0;
    let t = right + left;
    let rm_sun = longitude_of_sun(t);
    let rm_sun0 = rm_sun - rm_sun % deg;
    let mut delta_t1: f64 = 0.0;
    let mut delta_t2: f64 = 1.0;
    while (delta_t1 + delta_t2).abs() > (1.0 / 86400.0) {
        let right = (tm2 + 0.5) / 36525.0;
        let left = (tm1 - 2451545.0) / 36525.0;
        let t = right + left;
        let rm_sun = longitude_of_sun(t);
        let mut delta_rm = rm_sun - rm_sun0;
        if delta_rm > 180.0 {
            delta_rm -= 360.0;
        } else if delta_rm < -180.0 {
            delta_rm += 360.0;
        }
        let (tmp_t2, tmp_t1) = libm::modf(delta_rm * 365.2 / 360.0);
        tm1 -= tmp_t1;
        tm2 -= tmp_t2;
        delta_t1 = tmp_t1;
        delta_t2 = tmp_t2;
        if tm2 < 0.0 {
            tm2 += 1.0;
            tm1 -= 1.0
        }
    }
    return [tm1 + tm2 + TZ, rm_sun0];
}

fn longitude_of_sun(t: f64) -> f64 {
    const C4SUN: [[f64; 3]; 14] = [
        [31557.0, 161.0, 0.0004],
        [29930.0, 48.0, 0.0004],
        [2281.0, 221.0, 0.0005],
        [155.0, 118.0, 0.0005],
        [33718.0, 316.0, 0.0006],
        [9038.0, 64.0, 0.0007],
        [3035.0, 110.0, 0.0007],
        [65929.0, 45.0, 0.0007],
        [22519.0, 352.0, 0.0013],
        [45038.0, 254.0, 0.0015],
        [445267.0, 208.0, 0.0018],
        [19.0, 159.0, 0.0018],
        [32964.0, 158.0, 0.0020],
        [71998.1, 265.1, 0.0200],
    ];
    let mut th: f64 = 0.0;
    for c in C4SUN {
        let ang = (c[0] * t + c[1]) % 360.0;
        th += c[2] * (K * ang).cos();
    }
    let mut ang = (35999.05 * t + 267.52) % 360.0;
    th -= 0.0048 * t * (K * ang).cos();
    th += 1.9147 * (K * ang).cos();
    ang = (36000.7695 * t) % 360.0;
    ang = (ang + 280.4659) % 360.0;
    (th + ang) % 360.0
}
fn longitude_of_moon(t: f64) -> f64 {
    const C4MOON: [[f64; 3]; 61] = [
        [2322131.0, 191.0, 0.0003],
        [4067.0, 70.0, 0.0003],
        [549197.0, 220.0, 0.0003],
        [1808933.0, 58.0, 0.0003],
        [349472.0, 337.0, 0.0003],
        [381404.0, 354.0, 0.0003],
        [958465.0, 340.0, 0.0003],
        [12006.0, 187.0, 0.0004],
        [39871.0, 223.0, 0.0004],
        [509131.0, 242.0, 0.0005],
        [1745069.0, 24.0, 0.0005],
        [1908795.0, 90.0, 0.0005],
        [2258267.0, 156.0, 0.0006],
        [111869.0, 38.0, 0.0006],
        [27864.0, 127.0, 0.0007],
        [485333.0, 186.0, 0.0007],
        [405201.0, 50.0, 0.0007],
        [790672.0, 114.0, 0.0007],
        [1403732.0, 98.0, 0.0008],
        [858602.0, 129.0, 0.0009],
        [1920802.0, 186.0, 0.0011],
        [1267871.0, 249.0, 0.0012],
        [1856938.0, 152.0, 0.0016],
        [401329.0, 274.0, 0.0018],
        [341337.0, 16.0, 0.0021],
        [71998.0, 85.0, 0.0021],
        [990397.0, 357.0, 0.0021],
        [818536.0, 151.0, 0.0022],
        [922466.0, 163.0, 0.0023],
        [99863.0, 122.0, 0.0024],
        [1379739.0, 17.0, 0.0026],
        [918399.0, 182.0, 0.0027],
        [1934.0, 145.0, 0.0028],
        [541062.0, 259.0, 0.0037],
        [1781068.0, 21.0, 0.0038],
        [133.0, 29.0, 0.0040],
        [1844932.0, 56.0, 0.0040],
        [1331734.0, 283.0, 0.0040],
        [481266.0, 205.0, 0.0050],
        [31932.0, 107.0, 0.0052],
        [926533.0, 323.0, 0.0068],
        [449334.0, 188.0, 0.0079],
        [826671.0, 111.0, 0.0085],
        [1431597.0, 315.0, 0.0100],
        [1303870.0, 246.0, 0.0107],
        [489205.0, 142.0, 0.0110],
        [1443603.0, 52.0, 0.0125],
        [75870.0, 41.0, 0.0154],
        [513197.9, 222.5, 0.0304],
        [445267.1, 27.9, 0.0347],
        [441199.8, 47.4, 0.0409],
        [854535.2, 148.2, 0.0458],
        [1367733.1, 280.7, 0.0533],
        [377336.3, 13.2, 0.0571],
        [63863.5, 124.2, 0.0588],
        [966404.0, 276.5, 0.1144],
        [35999.05, 87.53, 0.1851],
        [954397.74, 179.93, 0.2136],
        [890534.22, 145.7, 0.6583],
        [413335.35, 10.74, 1.2740],
        [477198.868, 44.963, 6.2888],
    ];
    let mut th: f64 = 0.0;
    for c in C4MOON {
        let ang = (c[0] * t + c[1]) % 360.0;
        th += c[2] * (K * ang).cos();
    }
    let mut ang = (481267.8809 * t) % 360.0;
    ang = (ang + 218.3162) % 360.0;
    (th + ang) % 360.0
}

fn calc_saku(tm: f64) -> f64 {
    let (mut tm2, mut tm1) = libm::modf(tm);
    tm2 -= TZ;
    for lc in 1..30 {
        let right = (tm2 + 0.5) / 36525.0;
        let left = (tm1 - 2451545.0) / 36525.0;
        let t = right + left;
        let rm_sun = longitude_of_sun(t);
        let rm_moon = longitude_of_moon(t);
        let mut delta_rm = rm_moon - rm_sun;

        if lc == 1 && delta_rm < 0.0 {
            if delta_rm < 0.0 {
                delta_rm += 360.0;
            }
            delta_rm = delta_rm % 360.0;
        } else if rm_sun >= 0.0 && rm_sun <= 20.0 && rm_moon >= 300.0 {
            if delta_rm < 0.0 {
                delta_rm += 360.0;
            }
            delta_rm = delta_rm % 360.0;
            delta_rm = 360.0 - delta_rm;
        } else if delta_rm.abs() > 40.0 {
            if delta_rm < 0.0 {
                delta_rm += 360.0;
            }
            delta_rm = delta_rm % 360.0;
        }
        let (tmp_t2, tmp_t1) = libm::modf(delta_rm * 29.530589 / 360.0);
        tm1 -= tmp_t1;
        tm2 -= tmp_t2;
        let delta_t1 = tmp_t1;
        let delta_t2 = tmp_t2;
        if tm2 < 0.0 {
            tm2 += 1.0;
            tm1 -= 1.0;
        }
        if (delta_t1 + delta_t2).abs() > 1.0 / 86400.0 {
            if lc == 15 {
                tm1 = tm - 26.0;
                tm2 = 0.0;
            }
        } else {
            break;
        }
    }
    tm2 + tm1 + TZ
}
     fn getrokuyou(qm:u32,qd:u32) -> &'static str {
        let roku = ["先勝", "友引", "先負", "仏滅", "大安", "赤口"];
        let x = ((qm + qd - 2) % 6) as usize;
        roku[x]
    }

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use chrono::{Duration, NaiveDate};
    use std::env;
    use std::process::Command;
    #[test]
    fn diffawk() {
        let mut y = 2021;
        let mut m = 1;
        let mut d = 1;
        let mut date = NaiveDate::from_ymd(y, m, d);
        while y < 2022 {
            let path = env::current_dir().unwrap();
            let s = format!("{}/src/qrsamp11/qreki.awk", path.display());
            let output = Command::new("gawk")
                .args(&["-f", &s, &y.to_string(), &m.to_string(), &d.to_string()])
                .output()
                .expect("faild to start awk");
            let awkstr = String::from_utf8_lossy(&output.stdout);
            let qreki = crate::Qreki::from_ymd(y, m, d);
            let mut qdaystr: &str = &format!("{}", qreki.qd);
            if qreki.qd == 1 {
                qdaystr = "朔"
            }
            let mut qmstr: &str = &format!("{}", qreki.qm);
            if qreki.qm == 1 {
                qmstr = "正"
            }
            let qlstr: &str = match qreki.ql {
                true => "閏",
                false => "",
            };

            let ruststr = format!(
                "西暦{}年 {}月 {}日は、旧暦{}年{}{}月 {}日 {}です。",
                y,
                m,
                d,
                qreki.qy,
                qlstr,
                qmstr,
                qdaystr,
                qreki.qr
            );
            assert_eq!(awkstr.replace(" ", "").trim(), ruststr.replace(" ", ""));
            date = date + Duration::days(1);
            y = date.year();
            m = date.month();
            d = date.day();
        }
    }
}
