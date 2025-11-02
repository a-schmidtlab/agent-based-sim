// Statistics collection and visualization

use std::collections::VecDeque;

/// Data point for time series
#[derive(Debug, Clone, Copy)]
pub struct DataPoint {
    pub tick: u64,
    pub predator_count: usize,
    pub prey_count: usize,
    pub predator_energy: f64,
    pub prey_energy: f64,
}

/// Statistics collector with circular buffer
pub struct StatisticsCollector {
    data: VecDeque<DataPoint>,
    max_history: usize,
    current_tick: u64,
}

impl StatisticsCollector {
    /// Create a new statistics collector
    pub fn new(max_history: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(max_history),
            max_history,
            current_tick: 0,
        }
    }

    /// Record a data point
    pub fn record(&mut self, predator_count: usize, prey_count: usize, predator_energy: f64, prey_energy: f64) {
        let point = DataPoint {
            tick: self.current_tick,
            predator_count,
            prey_count,
            predator_energy,
            prey_energy,
        };
        
        if self.data.len() >= self.max_history {
            self.data.pop_front();
        }
        self.data.push_back(point);
        self.current_tick += 1;
    }

    /// Get all data points
    pub fn data(&self) -> &VecDeque<DataPoint> {
        &self.data
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.data.clear();
        self.current_tick = 0;
    }

    /// Get current tick
    pub fn current_tick(&self) -> u64 {
        self.current_tick
    }

    /// Get latest data point
    pub fn latest(&self) -> Option<&DataPoint> {
        self.data.back()
    }

    /// Calculate statistics
    pub fn stats(&self) -> Statistics {
        if self.data.is_empty() {
            return Statistics::default();
        }

        let mut total_predators = 0;
        let mut total_prey = 0;
        let mut max_predators = 0;
        let mut max_prey = 0;
        let mut min_predators = usize::MAX;
        let mut min_prey = usize::MAX;
        let mut sum_predator_energy = 0.0;
        let mut sum_prey_energy = 0.0;

        for point in &self.data {
            total_predators += point.predator_count;
            total_prey += point.prey_count;
            max_predators = max_predators.max(point.predator_count);
            max_prey = max_prey.max(point.prey_count);
            min_predators = min_predators.min(point.predator_count);
            min_prey = min_prey.min(point.prey_count);
            sum_predator_energy += point.predator_energy;
            sum_prey_energy += point.prey_energy;
        }

        let count = self.data.len() as f64;

        Statistics {
            average_predators: total_predators as f64 / count,
            average_prey: total_prey as f64 / count,
            max_predators,
            max_prey,
            min_predators: if min_predators == usize::MAX { 0 } else { min_predators },
            min_prey: if min_prey == usize::MAX { 0 } else { min_prey },
            average_predator_energy: sum_predator_energy / count,
            average_prey_energy: sum_prey_energy / count,
            data_points: self.data.len(),
        }
    }
}

/// Calculated statistics
#[derive(Debug, Default)]
pub struct Statistics {
    pub average_predators: f64,
    pub average_prey: f64,
    pub max_predators: usize,
    pub max_prey: usize,
    pub min_predators: usize,
    pub min_prey: usize,
    pub average_predator_energy: f64,
    pub average_prey_energy: f64,
    pub data_points: usize,
}

/// Render a population graph
pub fn render_population_graph(
    painter: &egui::Painter,
    collector: &StatisticsCollector,
    rect: egui::Rect,
) {
    let data = collector.data();
    if data.is_empty() {
        return;
    }

    // Find data range
    let max_count = data.iter()
        .map(|p| p.predator_count.max(p.prey_count))
        .max()
        .unwrap_or(1)
        .max(1);
    
    let y_scale = if max_count > 0 {
        rect.height() / (max_count as f32 + 10.0)
    } else {
        1.0
    };

    let x_scale = if data.len() > 1 {
        rect.width() / (data.len() - 1) as f32
    } else {
        rect.width()
    };

    // Draw background
    painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(250, 250, 250));

    // Draw grid lines
    let grid_lines = 5;
    for i in 0..=grid_lines {
        let y = rect.top() + (rect.height() / grid_lines as f32) * i as f32;
        painter.line_segment(
            [egui::Pos2::new(rect.left(), y), egui::Pos2::new(rect.right(), y)],
            egui::Stroke::new(0.5, egui::Color32::from_rgb(200, 200, 200)),
        );
        
        // Y-axis labels
        let value = max_count - (max_count as f32 / grid_lines as f32 * i as f32) as usize;
        painter.text(
            egui::Pos2::new(rect.left() - 30.0, y),
            egui::Align2::RIGHT_CENTER,
            value.to_string(),
            egui::FontId::monospace(10.0),
            egui::Color32::from_rgb(100, 100, 100),
        );
    }

    // Draw predator line (red)
    if data.len() > 1 {
        for i in 0..data.len() - 1 {
            let x1 = rect.left() + i as f32 * x_scale;
            let y1 = rect.bottom() - data[i].predator_count as f32 * y_scale;
            let x2 = rect.left() + (i + 1) as f32 * x_scale;
            let y2 = rect.bottom() - data[i + 1].predator_count as f32 * y_scale;
            
            painter.line_segment(
                [egui::Pos2::new(x1, y1), egui::Pos2::new(x2, y2)],
                egui::Stroke::new(2.0, egui::Color32::from_rgb(220, 20, 60)),
            );
        }
    }

    // Draw prey line (green)
    if data.len() > 1 {
        for i in 0..data.len() - 1 {
            let x1 = rect.left() + i as f32 * x_scale;
            let y1 = rect.bottom() - data[i].prey_count as f32 * y_scale;
            let x2 = rect.left() + (i + 1) as f32 * x_scale;
            let y2 = rect.bottom() - data[i + 1].prey_count as f32 * y_scale;
            
            painter.line_segment(
                [egui::Pos2::new(x1, y1), egui::Pos2::new(x2, y2)],
                egui::Stroke::new(2.0, egui::Color32::from_rgb(34, 139, 34)),
            );
        }
    }

    // Draw legend
    let legend_y = rect.top() + 10.0;
    painter.circle_filled(egui::Pos2::new(rect.left() + 10.0, legend_y), 4.0, egui::Color32::from_rgb(220, 20, 60));
    painter.text(
        egui::Pos2::new(rect.left() + 20.0, legend_y),
        egui::Align2::LEFT_CENTER,
        "Predators",
        egui::FontId::monospace(10.0),
        egui::Color32::BLACK,
    );
    
    painter.circle_filled(egui::Pos2::new(rect.left() + 90.0, legend_y), 4.0, egui::Color32::from_rgb(34, 139, 34));
    painter.text(
        egui::Pos2::new(rect.left() + 100.0, legend_y),
        egui::Align2::LEFT_CENTER,
        "Prey",
        egui::FontId::monospace(10.0),
        egui::Color32::BLACK,
    );
}

