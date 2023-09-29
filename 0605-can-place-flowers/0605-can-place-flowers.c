bool canPlaceFlowers(int* flowerbed, int flowerbedSize, int n){
    // 0 = empty pot
    // 1 = full pot
    // 2 = empty, but is viable to be potted
    int prev = 0;
    int total_pottable = 0;
    for (int i = 0; i < flowerbedSize; i++) {
        int pot = flowerbed[i];
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
}