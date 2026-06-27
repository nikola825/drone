#[repr(u8)]
pub enum OSDSymbol {
    Blank = b' ',
    Vols = b'\x1f',
    Battery = b'\x63',
    LinkQuality = b'\x02',
    Rssi = b'\x01',
    Percent = b'\x25',
    Dbm = b'\x13',
    ThrottlePercentage = b'\x95',
    SateliteLeft = b'\x08',
    SateliteRight = b'\x09',
    SpeedKmh = b'\x90',
    AltitudeMeters = b'\x76',
    Heading = b'\x0c',
    Home = b'\x0a',
    DistanceMeters = b'\x7a',
    ArrowLeft = b'\x1d',
    ArrowRight = b'\x19',
}

impl From<OSDSymbol> for u8 {
    fn from(value: OSDSymbol) -> Self {
        value as u8
    }
}
