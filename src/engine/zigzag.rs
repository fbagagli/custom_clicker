use super::ScanParams;
use std::cmp;

pub fn generate_cycle_points(params: &ScanParams) -> Vec<(i32, i32)> {
    let min_x = cmp::min(params.start_x, params.end_x);
    let max_x = cmp::max(params.start_x, params.end_x);
    let min_y = cmp::min(params.start_y, params.end_y);
    let max_y = cmp::max(params.start_y, params.end_y);

    let mut x_points = Vec::new();
    let mut current_x = min_x;
    let x_shift = if params.x_shift_pixels <= 0 {
        1
    } else {
        params.x_shift_pixels
    };
    while current_x <= max_x {
        x_points.push(current_x);
        current_x += x_shift;
    }
    // If the last column wasn't exactly max_x, add it.
    if x_points.last() != Some(&max_x) {
        x_points.push(max_x);
    }

    let mut y_points_down = Vec::new();
    let mut current_y = min_y;
    let y_step = if params.y_step_pixels <= 0 {
        1
    } else {
        params.y_step_pixels
    };
    while current_y <= max_y {
        y_points_down.push(current_y);
        current_y += y_step;
    }
    // If the last row wasn't exactly max_y, add it.
    if y_points_down.last() != Some(&max_y) {
        y_points_down.push(max_y);
    }

    let mut y_points_up = y_points_down.clone();
    y_points_up.reverse();

    let mut points = Vec::new();
    for (i, &x) in x_points.iter().enumerate() {
        if i % 2 == 0 {
            // Even column: go down
            for &y in &y_points_down {
                points.push((x, y));
            }
        } else {
            // Odd column: go up
            for &y in &y_points_up {
                points.push((x, y));
            }
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::ScanParams;

    #[test]
    fn test_generate_cycle_points() {
        let params = ScanParams {
            start_x: 0,
            start_y: 0,
            end_x: 10,
            end_y: 10,
            x_shift_pixels: 5,
            y_step_pixels: 5,
            total_time_seconds: 1.0,
        };

        let points = generate_cycle_points(&params);
        assert_eq!(
            points,
            vec![
                (0, 0),
                (0, 5),
                (0, 10), // col 0: down
                (5, 10),
                (5, 5),
                (5, 0), // col 1: up
                (10, 0),
                (10, 5),
                (10, 10), // col 2: down
            ]
        );
    }

    #[test]
    fn test_generate_cycle_points_not_exact_multiple() {
        let params = ScanParams {
            start_x: 0,
            start_y: 0,
            end_x: 12,
            end_y: 12,
            x_shift_pixels: 5,
            y_step_pixels: 5,
            total_time_seconds: 1.0,
        };

        let points = generate_cycle_points(&params);
        let x_cols: Vec<_> = points
            .iter()
            .map(|(x, _)| *x)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        let mut x_cols = x_cols;
        x_cols.sort();
        assert_eq!(x_cols, vec![0, 5, 10, 12]);

        let y_rows: Vec<_> = points
            .iter()
            .map(|(_, y)| *y)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        let mut y_rows = y_rows;
        y_rows.sort();
        assert_eq!(y_rows, vec![0, 5, 10, 12]);
    }
}
