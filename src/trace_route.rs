use serde::Serialize;
use crate::{costing, DateTime};
use crate::route::{Location};


#[derive(Serialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
/// Type of the directions
pub enum ShapeMatchType {
    /// indicating an edge walking algorithm can be used.
    #[serde(rename = "edge_walk")]
    EdgeWalk,

    /// indicating hat a map-matching algorithm should be used because the input shape might not
    /// closely match Valhalla edges.
    #[serde(rename = "map_snap")]
    MapSnap,

    /// indicating that maneuvers with instructions should be returned (this is the default if not
    /// specified).
    #[default]
    #[serde(rename = "walk_or_snap")]
    WalkOrSnap,
}

#[derive(Serialize, Default, Debug)]
/// Trace options
pub struct TraceOptions {
    search_radius: Option<f64>,
    gps_accuracy: Option<f64>,
    breakage_distance: Option<f64>,
    interpolation_distance: Option<f64>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Default, Debug)]
/// Trace route request
pub struct Manifest {
    shape_match: Option<ShapeMatchType>,
    #[serde(flatten)]
    costing: Option<costing::Costing>,
    begin_time: Option<DateTime>,
    durations: Option<Vec<i64>>,
    use_timestamps: Option<bool>,
    trace_options: Option<TraceOptions>,
    linear_references: Option<bool>,
    shape: Vec<Location>,
}
