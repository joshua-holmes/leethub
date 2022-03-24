/**
 * @param {number[][]} points
 * @param {number[][]} queries
 * @return {number[]}
 */
var countPoints = function(points, queries) {
    // initialization
    const answer = [];
    // loop through circles (i)
    for (let i = 0; i < queries.length; i++) {
        let counter = 0
        // loop through points (j)
        for (let j = 0; j < points.length; j++) {
            // a = subtract point[x] from queries[x] (absolute value)
            const a = Math.abs(queries[i][0] - points[j][0]);
            // b = subtract point[y] from queries[y] (absolute value)
            const b = Math.abs(queries[i][1] - points[j][1]);
            // c = pathagorean theorem (this is distance from center of circle)
            const c = Math.sqrt(a**2 + b**2);
            // if c < r, then add one to answer[0] += 1
            if (c <= queries[i][2]) counter += 1;
        }
        answer.push(counter)
    }
    return answer;
};