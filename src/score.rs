use crate::board_controller::BoardController;
use crate::figure::{Figure, W_INFINITY};
use crate::movement::Move;
use crate::point::Point;

pub fn evaluate_score<T: Fn(Point, Figure) -> i32>(controller: &BoardController, eval_fn: T) -> i32 {
    let friend_score: i32 = controller.friend_list.iter()
        .map(|p| eval_fn(p, *controller.board.point(p)))
        .sum();
    // println!("friend_score: {}", friend_score);

    let enemy_score: i32 = controller.enemy_list.iter()
        .map(|p| eval_fn(p, *controller.board.point(p)))
        .sum();
    // println!("enemy_score: {}", enemy_score);

    friend_score - enemy_score
}

pub fn material_fn(_: Point, f: Figure) -> i32 {
    f.weight()
}

pub fn simple_positional_fn(p: Point, f: Figure) -> i32 {
    (unsafe { std::mem::transmute::<_, u8>(f.color()) } as i32 - 64) + p.y() as i32 * 8 + (8 - p.x() as i32)
}

#[derive(Default)]
pub struct MinMaxSimpleSearch {}

#[derive(Default)]
pub struct AlphaBetaSearch {}

pub trait MoveSearch {
    fn find_best_move(&self, controller: &mut BoardController, depth: i32) -> (i32, Option<Move>);
}

impl MoveSearch for MinMaxSimpleSearch {
    fn find_best_move(&self, controller: &mut BoardController, depth: i32) -> (i32, Option<Move>) {
        min_max_simple(controller, depth)
    }
}

impl MoveSearch for AlphaBetaSearch {
    fn find_best_move(&self, controller: &mut BoardController, depth: i32) -> (i32, Option<Move>) {
        alpha_betta(controller, depth, - W_INFINITY, W_INFINITY)
    }
}

///
/// Implements a simplified Minimax algorithm for a board game AI. This function recursively evaluates board states 
/// to determine the optimal move and score up to a given search depth. It uses a combination of material and positional
/// evaluation functions to determine the static score of a specific board state when the search depth reaches zero.
///
/// # Arguments
///
/// * `controller` - A mutable reference to a `BoardController` object, which manages the state of the game. 
///   It provides methods for generating possible moves, making/unmaking moves, and transitioning between players.
/// * `depth` - An integer value that specifies the recursion depth of the algorithm. A depth of zero causes 
///   the algorithm to stop and evaluate the board state.
///
/// # Returns
///
/// A tuple consisting of:
/// * `i32` - The best score achievable from the current position at the provided depth.
/// * `Option<Move>` - The best move to achieve the score, or `None` if the depth is zero or there are no valid moves.
///
/// # Algorithm Details
///
/// * The function attempts to recursively maximize the "score" of the current board state from the perspective of the
///   AI/player making the move. It does so by simulating possible moves, flipping perspectives (minimizing the opponent's score),
///   and evaluating the best outcome for the current depth.
/// * When `depth <= 0`, the function halts recursion and evaluates the static board state using the sum of material
///   and positional scores.
/// * A list of possible moves is generated using the `controller.friend_movies()` function, and the algorithm iterates 
///   over these moves to simulate each one.
/// * The moves are made and unmade using `controller.make_move()` and `controller.unmake_move()` respectively, ensuring
///   no persistent state changes during recursion.
/// * Move transitions between players are handled by `controller.pass_move_to_enemy()`.
///
/// # Notes
///
/// * The function assumes `W_INFINITY` is a predefined constant representing a very large score, 
///   which serves as a placeholder for the best possible/worst possible evaluation.
/// * `evaluate_score` is used to calculate the value of the board using a scoring function 
///   that combines material and positional considerations.
/// * Ties are broken by selecting the first move encountered with the same score.
/// 
pub fn min_max_simple(controller: &mut BoardController, depth: i32) -> (i32, Option<Move>) {
    if depth <= 0 {
        controller.position_counter += 1;
        return (evaluate_score(controller, |p, f| {
            material_fn(p, f) + simple_positional_fn(p, f)
        }), None);
    }

    // unsafe { println!("{:?}", (*friend_list.first).point); }
    let move_list = controller.friend_movies();

    // if let Some(king_eat_move) = self.find_king_eat_move(&move_list) {
    //     return (W_INFINITY, Some(*king_eat_move));
    // }
    // MoveList::default();
    // unsafe { println!("{:?}", (*friend_list.first).point); }

    let mut best_score = - W_INFINITY;
    let mut best_move: Option<Move> = move_list.iter().next().copied();
    for movement in move_list.iter() {
        let move_info = controller.make_move(movement);
        controller.pass_move_to_enemy();

        // println!("{}", self.board);
        // println!();

        let cur_score = - min_max_simple(controller, depth - 1).0;

        controller.pass_move_to_enemy();
        controller.unmake_move(move_info);

        if cur_score > best_score {
            best_score = cur_score;
            best_move = Some(*movement);
        }

    }

    (best_score, best_move)
}


/// The `alpha_betta` function is an implementation of the alpha-beta pruning algorithm
/// used in game trees to determine the best move in a turn-based strategy game.
/// It operates by recursively exploring possible moves and pruning suboptimal paths 
/// to reduce the number of evaluated positions in the search tree. This function 
/// returns the best move and its associated score, where a higher score is considered better.
///
/// # Parameters
///
/// - `controller: &mut BoardController`
///   - A mutable reference to the `BoardController` instance used for managing the game state, 
///     querying possible moves, making simulated moves, and evaluating board positions.
///
/// - `depth: i32`
///   - The remaining depth of the search tree. When the depth is `0`, the function evaluates the current
///     board state without further recursion.
///
/// - `alpha: i32`
///   - The alpha value in alpha-beta pruning, representing the best score achievable by the maximizing player
///     along the current path. It is initially set to a very low value (e.g., `-W_INFINITY`).
///
/// - `betta: i32`
///   - The beta value in alpha-beta pruning, representing the best score achievable by the minimizing player
///     along the current path. It is initially set to a very high value (e.g., `W_INFINITY`).
///
/// # Returns
///
/// - `(i32, Option<Move>)`
///   - A tuple containing:
///     - The best score (`i32`) for the current player at the given position.
///     - An `Option<Move>`, which is the corresponding best move. This could be `None` if no moves are possible.
///
/// # Algorithm Description
///
/// 1. **Base Case**:
///    - If `depth <= 0`, the maximum search depth is reached. The function evaluates the current 
///      board position using the `evaluate_score` function with a material and positional evaluation heuristic. 
///      It increments the `position_counter` on the `controller` to track the number of evaluated positions.
///      Returns the evaluation score and `None` for the move at this point.
///
/// 2. **Move Generation**:
///    - A list of possible moves for the current player is obtained via `controller.friend_movies()`.
///    - Moves are sorted using a simple positional evaluation heuristic defined by `simple_positional_fn`.
///
/// 3. **Recursive Exploration**:
///    - Iterates through the sorted move list, performing the following for each move:
///      - Makes the move, updates the game state, and recurses into the enemy's perspective.
///      - Uses the "null-window" approach to perform a preliminary alpha-beta search between `-(alpha + 1)`
///        and `-alpha`.
///      - If the preliminary result suggests a better score might exist (within the range of `alpha` to `betta`),
///        a deeper full alpha-beta search is performed between `-betta` and `-alpha`.
///      - After the recursive call, the game state is reverted to preserve the state before the move.
///
/// 4. **Alpha-Beta Updates**:
///    - Tracks the best score and corresponding move based on the recursive results.
///    - Updates the alpha value, indicating the best score seen so far. If `alpha` becomes greater than
///      or equal to `betta`, the function performs a "beta cutoff," terminating further exploration of this branch.
///
/// 5. **Return**:
///    - The function returns the best score and associated move for the current position.
///
/// # Complexity
///
/// - Best Case: When pruning is highly effective, the algorithm approaches `O(b^(d/2))`, where `b` is the branching factor
///   (number of valid moves at each position) and `d` is the depth of the search.
/// - Worst Case: Without pruning, the complexity is closer to `O(b^d)`.
///
pub fn alpha_betta(controller: &mut BoardController, depth: i32, mut alpha: i32, betta: i32) -> (i32, Option<Move>) {
    if depth <= 0 {
        controller.position_counter += 1;
        return (evaluate_score(controller, |p, f| {
            material_fn(p, f) + simple_positional_fn(p, f)
        }), None);
    }

    let mut move_list = controller.friend_movies();
    move_list.sort_by(controller.board, simple_positional_fn);

    // if let Some(first_move) = move_list.iter().next() {
    //     let f = self.board.point(first_move.to);
    //     if f.rank() == KING && f.color() == self.enemy_color && first_move.m_type == SIMPLE {
    //         return (W_INFINITY, Some(*first_move));
    //     }
    // }

    let mut best_score = - W_INFINITY;
    let mut best_move: Option<Move> = move_list.iter().next().copied();
    for movement in move_list.iter() {
        let move_info = controller.make_move(movement);
        controller.pass_move_to_enemy();

        let mut cur_score = - alpha_betta(controller, depth - 1, - (alpha + 1), - alpha).0;
        if cur_score > alpha && cur_score < betta {
            cur_score = - alpha_betta(controller, depth - 1, - betta, - alpha).0;
        }

        controller.pass_move_to_enemy();
        controller.unmake_move(move_info);

        if cur_score > best_score {
            best_score = cur_score;
            best_move = Some(*movement);
        }

        if best_score > alpha { alpha = best_score }
        if alpha >= betta { return (alpha, best_move) }
    }

    (best_score, best_move)
}
