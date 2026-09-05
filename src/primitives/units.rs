// <comment>
// 2026-09-01
// Kaleb Troyer

// ========================================
// Time and Duration
// ========================================
// 

/// 
#[derive(Debug, Deserialize, Serialize)]
pub enum Time {
    Seconds,
    Minutes,
    Hours,
    Rounds,
    Turns,
}

/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Duration {
    count: u16,
    units: Time,
}

// ========================================
// Time and Duration
// ========================================
// 

/// 
#[derive(Debug, Deserialize, Serialize)]
pub enum Length {
    Feet,
    Miles,
}

/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct Distance {
    min: u16,
    mid: u16,
    max: u16,
    units: Option<Length>,
}

// EOF
