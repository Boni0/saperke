mod grid;

use grid::{GridStruct, GridCellVariant};

fn main() {

    let mut grid = GridStruct::new_rectangle_or_square_grid(10, 10);
    grid.set_mines_to_cells_randomly(15);

    println!("--------------------------------------------------------------");

    for grid_line_vec in grid.cells {
        print!("|");

        for grid_cell in grid_line_vec {
            let print_value = match grid_cell.variant {
                GridCellVariant::WithValue(value) => value.to_string(),
                GridCellVariant::WithBomb => String::from("B"),
                GridCellVariant::NonExist => String::from("X"),
            };

            print!(" {} ", print_value);
        }

        println!("|");
    }

    println!("--------------------------------------------------------------");

}
