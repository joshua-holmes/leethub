/**
 * @param {number[]} flowerbed
 * @param {number} n
 * @return {boolean}
 */
var canPlaceFlowers = function(flowerbed, n) {
    // 0 = empty pot
    // 1 = full pot
    // 2 = empty, but is viable to be potted
    let prev = 0;
    let total_pottable = 0;
    for (const [i, pot] of flowerbed.entries()) {
        switch (prev) {
            case 0:
                if (pot == 0) {
                    prev = 2;
                    total_pottable += 1;
                    continue;
                }
                break;
            case 2:
                if (pot == 1) {
                    total_pottable -= 1;
                }
                break;
        }
        prev = pot;
    }

    return total_pottable >= n;
};