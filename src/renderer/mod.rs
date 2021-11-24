mod controllers;

pub mod GridRenderer {
    use druid::{Env, Lens, LensExt, Widget, WidgetExt, WidgetId, lens};
    use druid::widget::{SizedBox, List, Flex, Svg, SvgData, Painter, Label, LensWrap, LensScopeTransfer, ViewSwitcher};

    use crate::assets::SvgAssets;
    use crate::game::{Game};
    use crate::{AppStruct};
    use crate::grid::{GridCell, GridStruct, GridCellState, GridCellVariant};

    pub fn create() -> impl Widget<AppStruct> {
        ViewSwitcher::new(
            |data: &AppStruct, _| Clone::clone(&data.assets),
            |svg_assets, data, _| {
                let mut flex = Flex::column();

                for row in  data.game.grid.cells.iter() {
                    let mut flex_row = Flex::row();

                    for cell_iter in row {
                        let cell: &GridCell = cell_iter;
                        flex_row.add_child(self::create_cell(cell, svg_assets));
                    }

                    flex.add_child(flex_row);
                } 

                flex.center().boxed()
            }
        )
    }

    fn create_cell(cell: &GridCell, svg_assets: &SvgAssets) -> impl Widget<AppStruct> {
        let svg = match cell.variant {
            GridCellVariant::WithValue(value) => {
                if value == 0 {
                    SvgData::default()
                } else {
                    svg_assets.get(value.to_string().as_str())
                }
            },
            GridCellVariant::WithBomb => svg_assets.get("bomb"),
            GridCellVariant::NonExist => SvgData::default(),
        };

        let mut bg = Svg::new(svg_assets.get("opened"));
    
        let box_painter = Painter::new(move |ctx, _data, env| {
            bg.paint(ctx, _data, env);
        }); 
    
        Svg::new(svg)
            .fix_size(23.0, 23.0)
            .background(box_painter)
            // .controller(ClickController)
            // .on_click(|_, _, _| ()) // has to be to update into hot or active
    }
}