/**
 * @param {character[][]} board
 * @return {boolean}
 */
var isValidSudoku = function(board) {
    function findQuadrant(x, y) {
        return Math.floor(y / 3) * 3 + Math.floor(x / 3);
    }
    const columns = [[],[],[],[],[],[],[],[],[]];
    const box = [[],[],[],[],[],[],[],[],[]];
    // loop row
    for (let row = 0; row < 9; row++) {
        // loop column
        for (let col = 0; col < 9; col++) {
            if (board[row][col] !== '.') {
                
                // check column for dup
                if (columns[col].includes(board[row][col])) return false;
                
                // add value to translated board
                columns[col].push(board[row][col]);
                
                // check row for dup
                const lastInd = board[row].lastIndexOf(board[row][col])
                if (lastInd !== col) return false;
                
                // set quadrant
                const quadrant = findQuadrant(col, row);
                // check box
                if (box[quadrant].includes(board[row][col])) return false;
                
                // add value to box
                box[quadrant].push(board[row][col]);
            }
        }
    }
    return true;
};
