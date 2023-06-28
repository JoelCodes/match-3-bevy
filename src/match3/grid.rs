use bevy::{prelude::*};
use bevy_inspector_egui::{InspectorOptions, prelude::ReflectInspectorOptions};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, InspectorOptions, FromReflect)]
#[reflect(Hash, InspectorOptions)]
pub enum TileType {
  Pentagon,
  Triangle,
  Square,
  Circle,
  Diamond,
  Star
}

pub fn rand_tile() -> TileType {
  let mut rng = rand::thread_rng();
  match rng.gen_range(0..6) {
    0 => TileType::Pentagon,
    1 => TileType::Triangle,
    2 => TileType::Square,
    3 => TileType::Circle,
    4 => TileType::Diamond,
    _ => TileType::Star,
  }
}

pub fn create_grid(rows: usize, columns:usize) -> Vec<Vec<Option<TileType>>> {
  let mut grid = vec![vec![None; rows.into()]; columns.into()];
  for column in 0..columns {
    for row in 0..rows {
      grid[column][row] = Some(rand_tile());
    }
  }
  loop {
    let shapes = find_shapes(&grid);
    if shapes.len() == 0 {
      break;
    }
    for shape in shapes {
      for (column, row) in shape {
        grid[column][row] = Some(rand_tile());
      }
    }
  }
  grid
}

// Given a grid, find all the shapes in it
// A shape is a group of 3 or more tiles of the same type in a row, vertically or horizontally
// Once a shape is found, push the coordinates of the tiles in the shape to the shapes vector
// A horizontal shape and vertical shape can share a tile, 
// but no two vertical shapes should share the same tile,
// and no two horizontal shapes should share the same tile.
pub fn find_shapes(grid: &Vec<Vec<Option<TileType>>>) -> Vec<Vec<(usize, usize)>> {
  let mut shapes = vec![];
  // find horizontal shapes.
  // for each row in the grid, start at the first tile.
  // check to see if that tile is not None.  If so, move to the next.
  // if it is Some, check to see if the next tile is Some and the same type.
  // Find out how long the shape is, and push the coordinates of the tiles in the shape to the shapes vector.
  // Move to the next tile after the shape.
  // Repeat until the end of the row.
  // Repeat for each row.
  for row in 0..grid.len() {
    let mut column = 0;
    while column < grid[row].len() {
      if let Some(tile_type) = grid[row][column] {
        let mut shape = vec![(row, column)];
        let mut next_column = column + 1;
        while next_column < grid[row].len() && grid[row][next_column] == Some(tile_type) {
          shape.push((row, next_column));
          next_column += 1;
        }
        if shape.len() >= 3 {
          shapes.push(shape);
        }
        column = next_column;
      } else {
        column += 1;
      }
    }
  }
  // find vertical shapes the same way.
  for column in 0..grid[0].len() {
    let mut row = 0;
    while row < grid.len() {
      if let Some(tile_type) = grid[row][column] {
        let mut shape = vec![(row, column)];
        let mut next_row = row + 1;
        while next_row < grid.len() && grid[next_row][column] == Some(tile_type) {
          shape.push((next_row, column));
          next_row += 1;
        }
        if shape.len() >= 3 {
          shapes.push(shape);
        }
        row = next_row;
      } else {
        row += 1;
      }
    }
  }
  shapes
}

#[cfg(test)]
mod tests {
  use super::*;
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
  #[test]
  fn find_shapes_finds_horizontal_shapes() {
    let grid = vec![
      vec![Some(TileType::Pentagon), Some(TileType::Pentagon), Some(TileType::Pentagon)],
      vec![Some(TileType::Circle), Some(TileType::Pentagon), Some(TileType::Circle)],
      vec![Some(TileType::Circle), Some(TileType::Circle), Some(TileType::Pentagon)],
    ];
    let shapes = find_shapes(&grid);
    assert_eq!(shapes.len(), 1);
    assert_eq!(shapes[0], vec![(0, 0), (0, 1), (0, 2)]);
  }
  #[test]
  fn find_shapes_find_vertical_shapes() {
    let grid = vec![
      vec![Some(TileType::Pentagon), Some(TileType::Circle), Some(TileType::Circle)],
      vec![Some(TileType::Pentagon), Some(TileType::Pentagon), Some(TileType::Circle)],
      vec![Some(TileType::Pentagon), Some(TileType::Circle), Some(TileType::Pentagon)],
    ];
    let shapes = find_shapes(&grid);
    assert_eq!(shapes.len(), 1);
    assert_eq!(shapes[0], vec![(0, 0), (1, 0), (2, 0)]);
  }
  #[test]
  fn find_shapes_correctly_identifies_long_shapes() {
    // make a grid of 4x4 with all Some(TileType::Circle) 
    // except for one row of Some(TileType::Pentagon)
    // and one column of Some(TileType::Pentagon)
    let mut grid = vec![vec![Some(TileType::Circle); 4]; 4];
    for column in 0..4 {
      grid[2][column] = Some(TileType::Pentagon);
    }
    for row in 0..4 {
      grid[row][2] = Some(TileType::Pentagon);
    }
    let shapes = find_shapes(&grid);
    assert_eq!(shapes.len(), 2);
    assert_eq!(shapes[0], vec![(2, 0), (2, 1), (2, 2), (2, 3)]);
    assert_eq!(shapes[1], vec![(0, 2), (1, 2), (2, 2), (3, 2)]);

  }
}

