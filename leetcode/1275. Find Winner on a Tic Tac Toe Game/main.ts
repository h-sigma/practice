type PlayerA = 'A';
type PlayerB = 'B';
type Players = PlayerA | PlayerB;
type Elements = Players | ' ';
type Grid = Elements[];

function tictactoe(moves: number[][]): string {
	let grid: Grid = Array(9).fill(' ' satisfies Elements);
	let player: Players = 'A';

	for (const move of moves) {
		let [row, col] = move as [number, number];

		const ROW_SIZE = 3;
		grid[row * ROW_SIZE + col] = player;
		player = player === 'A' ? 'B' : 'A';
	}

	if (checkWin(grid, 'A')) return 'A';
	if (checkWin(grid, 'B')) return 'B';
	if (checkDraw(grid)) return 'Draw';
	return 'Pending';
}

function checkWin(grid: Grid, player: Players) {
	const WIN_INDICES: [number, number, number][] = [
		[0, 1, 2],
		[3, 4, 5],
		[6, 7, 8],
		[0, 3, 6],
		[1, 4, 7],
		[2, 5, 8],
		[0, 4, 8],
		[2, 4, 6],
	];
	for (const wi of WIN_INDICES) {
		if (
			grid[wi[0]] === player &&
			grid[wi[1]] === player &&
			grid[wi[2]] === player
		) {
			return true;
		}
	}
	return false;
}

function checkDraw(grid: Grid) {
	return !grid.some((ch) => ch === ' ');
}
