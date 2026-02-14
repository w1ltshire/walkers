#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("invalid zoom level")]
pub struct InvalidZoom;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Zoom(pub f64);

impl From<f64> for Zoom {
    fn from(value: f64) -> Self {
        Zoom(value)
    }
}

// The reverse shouldn't be implemented, since we already have TryInto<f32>.
#[allow(clippy::from_over_into)]
impl Into<f64> for Zoom {
    fn into(self) -> f64 {
        self.0
    }
}

impl Default for Zoom {
    fn default() -> Self {
        Self(16.)
    }
}

impl Zoom {
    pub fn round(&self) -> u8 {
        self.0.round() as u8
    }

    pub fn zoom_in(&mut self) {
        *self = Self::from(self.0 + 1.);
    }

    pub fn zoom_out(&mut self) {
        *self = Self::from(self.0 - 1.);
    }

    /// Zoom using a relative value.
    pub fn zoom_by(&mut self, value: f64) {
        *self = Self::from(self.0 + value);
    }
}