
use fltk::{app::*, button::*, window::*, input::*, dialog::*};

fn solvable(grid: &[[i32; 9]; 9]) -> bool {
    let mut items: [i32; 9];

    for row in grid {
        items = [0; 9];
        for &value in row {
            if value > 0 && value < 10 {
                items[(value - 1) as usize] += 1;
            }
        }
        if items.iter().any(|&n| n > 1) {
            return false;
        }
    }

    for i in 0..9 {
        items = [0; 9];
        for row in grid {
            if row[i] > 0 && row[i] < 10 {
                items[(row[i] - 1) as usize] += 1;
            }
        }
        if items.iter().any(|&n| n > 1) {
            return false;
        }
    }

    for &x in [0, 3, 6].iter() {
        for &y in [0, 3, 6].iter() {
            items = [0; 9];
            for i in 0..3 {
                for j in 0..3 {
                    if grid[y + i][x + j] > 0 && grid[y + i][x + j]  < 10 {
                        items[(grid[y + i][x + j] - 1) as usize] += 1;
                    }
                }
            }
            if items.iter().any(|&n| n > 1) {
                return false;
            }
        }
    }
    true
}

fn possible(grid: &[[i32; 9]; 9], y: usize, x: usize, number: i32) -> bool {
    if grid[y].iter().any(|&n| n == number) {
        return false;
    }
    
    if grid.iter().any(|n| n[x] == number) {
        return false;
    }
        
    let x0: usize = (x / 3) * 3;
    let y0: usize = (y / 3) * 3;

    for i in 0..3 {
        for j in 0..3 {
            if grid[y0 + i][x0 + j] == number {
                return false;
            }
        }
    }
    true
}

fn find_next_cell2fill(grid: &[[i32; 9]; 9]) -> (usize, usize) {
    for (x, row) in grid.iter().enumerate() {
        for (y, &val) in row.iter().enumerate() {
            if val == 0 {
                return (x, y);
            }
        }
    }
    (99, 99)
}

fn solve(grid : &mut [[i32; 9]; 9]) -> bool {
    let (i, j) = find_next_cell2fill(grid);
    if i == 99 {
        return true;
    }
    for e in 1..10 {
        if possible(grid, i, j, e) {
            grid[i][j] = e;
            if solve(grid) {
                return true;
            }
            grid[i][j] = 0;
        }
    }
    false
}

fn is_value_legal(val : &str) -> bool {
    matches!(val.trim(), ""  | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
}

#[allow(clippy::redundant_clone)]
fn main() {
    let positions = vec![10, 40, 70, 104, 134, 164, 198, 228, 258];
    let mut input_fields: Vec<Input> = Vec::new();

    let app = App::default().with_scheme(AppScheme::Gtk);
    let mut wind = Window::default()
        .with_size(300, 380)
        .center_screen()
        .with_label("Sudoku solver");
    for &row in positions.iter() {
        for &column in positions.iter() {
            input_fields.push(Input::new(column, row, 30, 30, ""));
        }
    }
    let mut button_solve = Button::new(60, 310, 80, 40, "Solve");
    let mut button_clear = Button::new(150, 310, 80, 40, "Clear");
    wind.end();
    wind.show();
    let mut work_fields = input_fields.clone();
    button_solve.set_callback(Box::new(move || {
            let mut grid = [[0; 9]; 9];
            // Move data from screen to grid
            let mut r = 0;            
            for (idx, field) in work_fields.iter().enumerate() {
                if !is_value_legal(&field.value()) {
                    alert_default(format!("Illegal value: {}", field.value()).as_str());
                    return;    
                }
                let c = idx % 9;
                if idx > 0 && c == 0 { r += 1; }
                grid[r][c] = field.value().trim().parse().unwrap_or(0);
            }
            if solvable(&grid) {
                solve(&mut grid);
                // Move data from grid to screen
                r = 0;
                for (idx, field) in work_fields.iter().enumerate() {
                    let c = idx % 9;
                    if idx > 0 && c == 0 { r += 1; }
                    if grid[r][c] == 0 {
                        alert_default("Not solvable");        
                        break
                    };
                    let b = format!("  {}", grid[r][c]);
                    field.set_value(&b);
                }
            } else {
                alert_default("Not solvable");
            }
        }));
    work_fields = input_fields.clone();
    button_clear.set_callback(Box::new(move || {
        for field in work_fields.iter() {
            field.set_value("");
        }
    }));
    app.run().unwrap();
}