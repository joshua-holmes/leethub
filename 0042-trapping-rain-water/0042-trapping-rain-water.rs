use std::cmp;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let peaks = find_peaks(&height);
        let mut total_area = 0;
        for (i, j) in peaks {
            total_area += calculate_area(&height, i, j);
        }
        total_area
    }
}

fn find_peaks(height: &Vec<i32>) -> Vec<(usize, usize)> {
    let mut peaks = Vec::new();
    if height.len() < 3 {
        return peaks;
    }

    let mut queue = vec![(0, height.len() - 1)];
    while let Some((start, end)) = queue.pop() {
        let mut p1 = None;
        let mut p2 = None;
        let mut pre_i = start;
        let mut going_up = true;
        // loop through segment and find 2 tallest peaks
        for i in (start + 1)..=(end + 1) {
            let h = *height.get(i).unwrap_or(&-1);
            let pre_h = height[pre_i];
            // found a peak
            if pre_h > h && going_up {
                let mut new_p1 = None;
                let mut new_p2 = None;
                // check if it's the tallest
                if let Some(old_p1) = p1 {
                    if pre_h > height[old_p1] {
                        if let Some(old_p2) = p2 {
                            if height[old_p1] > height[old_p2] {
                                new_p2 = Some(old_p1);
                            }
                        } else {
                            new_p2 = Some(old_p1);
                        }
                        new_p1 = Some(pre_i);
                    } else {
                        if let Some(old_p2) = p2 {
                            if pre_h > height[old_p2] {
                                new_p2 = Some(pre_i);
                            }
                        } else {
                            new_p2 = Some(pre_i);
                        }
                    }
                } else {
                    new_p1 = Some(pre_i);
                }
                p1 = if new_p1.is_some() { new_p1 } else { p1 };
                p2 = if new_p2.is_some() { new_p2 } else { p2 };
            }
            // set going_up
            if pre_h < h {
                going_up = true;
            } else if pre_h > h {
                going_up = false;
            }
            pre_i = i;
        }
        // check if this segment had 2 peaks
        if let Some(p1) = p1 {
            if let Some(p2) = p2 {
                let mut ps = [p1, p2];
                ps.sort();
                let [p1, p2] = ps;
                peaks.push((p1, p2));
                queue.push((start, p1));
                queue.push((p2, end));
            }
        }
    }
    peaks
}

fn calculate_area(height: &Vec<i32>, peak1: usize, peak2: usize) -> i32 {
    let h1 = height[peak1];
    let h2 = height[peak2];
    let min = cmp::min(h1, h2);
    let mut area = 0;
    println!("P1 {:?} {:?}", peak1, h1);
    println!("P2 {:?} {:?}", peak2, h2);
    for i in (peak1 + 1)..peak2 {
        area += cmp::max(min - height[i], 0);
    }
    area
}