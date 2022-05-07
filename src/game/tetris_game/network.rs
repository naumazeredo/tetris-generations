// Network
use super::*;
use crate::game::network;

impl TetrisGame {
    pub fn from_network(
        net_tetris_game: network::NetworkedTetrisGame,
        rules: Rules,
        randomizer: Randomizer,
        net_timestamp: u64,
        app: &mut App,
        _persistent: &mut PersistentData
    ) -> Self {
        // lock delay
        let remaining_lock_delay = rules.lock_delay;

        // @TODO do we need this?
        // Fix timestamps
        app.set_game_timestamp(net_timestamp);

        Self {
            timestamp: net_tetris_game.timestamp,

            has_topped_out: net_tetris_game.has_topped_out,

            playfield: net_tetris_game.playfield,
            rules,
            randomizer,

            current_score: net_tetris_game.current_score,
            total_lines_cleared: net_tetris_game.total_lines_cleared,

            current_piece: net_tetris_game.current_piece,
            next_piece_types: net_tetris_game.next_piece_types,

            lock_piece_timestamp: net_tetris_game.lock_piece_timestamp,
            last_locked_piece: net_tetris_game.last_locked_piece,
            soft_drop_steps: 0,
            hard_drop_steps: 0,

            hold_piece: net_tetris_game.hold_piece,
            has_used_hold: false,

            is_locking: false,
            remaining_lock_delay,

            has_moved: false,
            has_rotated: false,
            has_stepped: false,
            last_piece_action: LastPieceAction::Movement,

            movement_last_timestamp_x: net_tetris_game.movement_last_timestamp_x,
            movement_last_timestamp_y: net_tetris_game.movement_last_timestamp_y,

            movement_animation_delta_grid_x: 0.0,
            movement_animation_delta_grid_y: 0.0,
            movement_animation_current_delta_grid: Vec2::new(),

            locking_animation_timestamp: 0,
        }
    }

    pub fn to_network(&self) -> network::NetworkedTetrisGame {
        network::NetworkedTetrisGame {
            timestamp: self.timestamp,

            has_topped_out: self.has_topped_out,
            playfield: self.playfield.clone(),

            current_score: self.current_score,
            total_lines_cleared: self.total_lines_cleared,

            current_piece: self.current_piece,
            next_piece_types: self.next_piece_types.clone(),

            lock_piece_timestamp: self.lock_piece_timestamp,
            last_locked_piece: self.last_locked_piece,

            hold_piece: self.hold_piece,

            movement_last_timestamp_x: self.movement_last_timestamp_x,
            movement_last_timestamp_y: self.movement_last_timestamp_y,
        }
    }

    pub fn update_from_network(
        &mut self,
        net_tetris_game: network::NetworkedTetrisGame,
        net_timestamp: u64,
        app: &mut App,
    ) {
        // @TODO do we need this?
        // Fix timestamps
        app.set_game_timestamp(net_timestamp);

        self.has_topped_out = net_tetris_game.has_topped_out;
        //self.playfield.update_from_network(net_tetris_game.playfield);
        self.playfield = net_tetris_game.playfield;

        self.current_score = net_tetris_game.current_score;
        self.total_lines_cleared = net_tetris_game.total_lines_cleared;

        self.current_piece = net_tetris_game.current_piece;
        self.next_piece_types = net_tetris_game.next_piece_types;

        self.lock_piece_timestamp = net_tetris_game.lock_piece_timestamp;
        self.last_locked_piece = net_tetris_game.last_locked_piece;

        self.hold_piece = net_tetris_game.hold_piece;

        self.movement_last_timestamp_x = net_tetris_game.movement_last_timestamp_x;
        self.movement_last_timestamp_y = net_tetris_game.movement_last_timestamp_y;
    }
}
