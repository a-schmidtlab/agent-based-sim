// UI module - Graphical interface components

pub mod controls;
pub mod visualization;
pub mod layout;
pub mod statistics;

pub use controls::ControlPanel;
pub use visualization::{VisualizationSettings, render_world};
pub use layout::UILayout;
pub use statistics::{StatisticsCollector, render_population_graph, Statistics};

