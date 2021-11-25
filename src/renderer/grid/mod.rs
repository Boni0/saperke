mod controllers;

use druid::{Widget, WidgetExt};
use druid::widget::{Flex, Svg, SvgData, Painter, ViewSwitcher};

use crate::{AppStruct};
use crate::grid::{GridCell, GridCellState, GridCellVariant};

use controllers::{CellClickController};

pub struct GridRenderer;

impl GridRenderer {
    pub fn render() -> impl Widget<AppStruct> {
        ViewSwitcher::new(
            |app: &AppStruct, _| app.clone(),
            |_, app, _| {
                let mut flex = Flex::column();

                for (y_cord, row) in app.game.grid.cells.iter().enumerate() {
                    let mut flex_row = Flex::row();

                    for (x_cord, cell) in row.iter().enumerate() {
                        flex_row.add_child(
                            GridRenderer::create_cell(cell, y_cord, x_cord, &app)
                        );
                    }

                    flex.add_child(flex_row);
                } 

                flex
                    .center()
                    // .controller(CellClickController { x_cord: 0, y_cord: 0 })
                    .boxed()
            }
        )
    }

    fn create_cell(cell: &GridCell, y_cord: usize, x_cord: usize, app: &AppStruct) -> impl Widget<AppStruct> {
        let value_svg_data = if cell.state == GridCellState::Visible {
            match cell.variant {
                GridCellVariant::WithValue(value) => {
                    if value == 0 {
                        SvgData::default()
                    } else {
                        app.assets.get(value.to_string().as_str())
                    }
                },
                GridCellVariant::WithBomb => app.assets.get("bomb"),
                GridCellVariant::NonExist => SvgData::default(),
            }
        } else {
            SvgData::default()
        };

        let mut bg_svg = Svg::new(
            app.assets.get(
                match cell.state {
                    GridCellState::Visible | GridCellState::Clicked => "opened",
                    _ => "unopened",
                }
            )
        );
    
        let box_painter = Painter::new(move |ctx, app: &AppStruct, env| {
            bg_svg.paint(ctx, app, env);
        }); 
    
        Svg::new(value_svg_data)
            .fix_size(23.0, 23.0)
            .background(box_painter)
            .controller(CellClickController::init(y_cord, x_cord))
            // .on_click(move |_, app_data: &mut AppStruct, _| {
            //     app_data.game.grid.set_cell_visible(y_cord, x_cord);          
            // }) // has to be to update into hot or active
    }
}