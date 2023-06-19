use crate::match3::grid::*;
#[test]
fn find_shapes_returns_empty_if_no_shapes_found() {
    let grid = vec![
        vec![Some(TileType::Pentagon), Some(TileType::Circle), Some(TileType::Circle)],
        vec![Some(TileType::Circle), Some(TileType::Pentagon), Some(TileType::Circle)],
        vec![Some(TileType::Circle), Some(TileType::Circle), Some(TileType::Pentagon)],
    ];
    let shapes = find_shapes(&grid);
    assert_eq!(shapes.len(), 0);
}
