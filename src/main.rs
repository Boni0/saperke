mod grid;

use grid::{GridStruct, GridCellVariant};

fn main() {

    let mut grid = GridStruct::new_rectangle_or_square_grid(10, 10);
    grid.set_mines_to_cells_randomly(15);

    grid.set_cell_visible(4, 3);

    println!("--------------------------------------------------------------");

    for grid_line_vec in grid.cells {
        print!("|");

        for grid_cell in grid_line_vec {
            let print_value = match grid_cell.state {
                grid::GridCellState::Visible => {
                    match grid_cell.variant {
                        GridCellVariant::WithValue(value) => value.to_string(),
                        GridCellVariant::WithBomb => String::from("B"),
                        GridCellVariant::NonExist => String::from("X"),
                    }
                },
                _ => String::from("?")
            };

            print!(" {} ", print_value);
        }

        println!("|");
    }

    println!("--------------------------------------------------------------");

}
