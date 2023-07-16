pub mod game;
pub mod magic_maze;
mod ordinary_maze;

#[cfg(test)]
mod tests {
    use super::{game, magic_maze::MagicMaze, ordinary_maze::OrdinaryMaze};

    #[test]
    fn test_game() {
        let ordinary_maze = OrdinaryMaze::new();
        game::run(ordinary_maze);

        let magic_maze = MagicMaze::new();
        game::run(magic_maze);
    }
}
