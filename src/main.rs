
fn display_puzzle(mask : u64, puzzle : &Vec<u32>, targets : &Vec<u32>) -> Vec<u32>
{
    let cv = |idx : usize| if (1u64 << idx) & mask == 0 { 0 } else { puzzle[idx] };
    let lv = |idxs:Vec<usize>| idxs.iter().map(|i|cv(*i)).sum::<u32>();
    let cell = |idx : usize| if (1u64 << idx) & mask == 0 { String::from(" ") } else { puzzle[idx].to_string() };

    let mut lvs :Vec<u32> = Vec::new();
    lvs.push(lv(vec![0, 1, 2, 3]));
    lvs.push(lv(vec![4, 5, 6, 7]));
    lvs.push(lv(vec![8, 9, 10, 11]));
    lvs.push(lv(vec![12, 13, 14, 15]));
    lvs.push(lv(vec![0, 4, 5, 6]));
    lvs.push(lv(vec![1, 5, 9, 13]));
    lvs.push(lv(vec![2, 6, 10, 14]));
    lvs.push(lv(vec![3, 7, 11, 15]));

    println!("     | {0:<4} | {1:<4} | {2:<4} | {3:<4}", targets[0], targets[1], targets[2], targets[3]);
    println!("{0:<4} | {1:<4} | {2:<4} | {3:<4}| {4:<4}| {5:<4}", targets[4], cell(0), cell(1), cell(2), cell(3), lvs[0]);
    println!("{0:<4} | {1:<4} | {2:<4} | {3:<4}| {4:<4}| {5:<4}", targets[5], cell(4), cell(5), cell(6), cell(7), lvs[1]);
    println!("{0:<4} | {1:<4} | {2:<4} | {3:<4}| {4:<4}| {5:<4}", targets[6], cell(8), cell(9), cell(10), cell(11), lvs[2]);
    println!("{0:<4} | {1:<4} | {2:<4} | {3:<4}| {4:<4}| {5:<4}", targets[7], cell(12), cell(13), cell(14), cell(15), lvs[3]);
    println!("     | {0:<4} | {1:<4} | {2:<4} | {3:<4}", lvs[4], lvs[5], lvs[6], lvs[7]);

   lvs 
}

fn main() {
    let puzzle_mask : u64 = 0b0110111111110110;
    let puzzle : Vec<u32> = vec![0,1,2,0,3,4,5,6,7,8,9,1,0,2,3,0];
    let targets : Vec<u32> = vec![20, 6, 7, 24, 8, 14, 23, 12];

    display_puzzle(puzzle_mask, &puzzle, &targets);
}