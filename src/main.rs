struct Puzzle
{
    mask : u64,
    puzzle : Vec<u32>,
    targets : Vec<u32>
}

impl Puzzle {
    pub fn cell (self : &Puzzle, idx : usize) -> u32
    {
        if (1u64 << idx) & self.mask == 0 { 0 } else { self.puzzle[idx] }
    }

    pub fn line ( self : &Puzzle, idxs : Vec<usize>) -> u32
    {
        idxs.iter().map(|i| self.cell(*i)).sum::<u32>()
    }

    pub fn calc_lines( self : &Puzzle ) -> Vec<u32>
    {
        let mut lvs :Vec<u32> = Vec::new();
        lvs.push(self.line(vec![0, 1, 2, 3]));
        lvs.push(self.line(vec![4, 5, 6, 7]));
        lvs.push(self.line(vec![8, 9, 10, 11]));
        lvs.push(self.line(vec![12, 13, 14, 15]));
        lvs.push(self.line(vec![0, 4, 5, 6]));
        lvs.push(self.line(vec![1, 5, 9, 13]));
        lvs.push(self.line(vec![2, 6, 10, 14]));
        lvs.push(self.line(vec![3, 7, 11, 15]));

        lvs
    }

    pub fn display( self : &Puzzle )
    {
        let d = |c : u32| if c == 0 { String::from(" ") } else { c.to_string() };
        let lvs = self.calc_lines();

        println!("     | {0:<4} | {1:<4} | {2:<4} | {3:<4}", self.targets[0], self.targets[1], self.targets[2], self.targets[3]);
        println!("{0:<4} | {1:<4} | {2:<4} | {3:<4}| {4:<4}| {5:<4}", self.targets[4], self.cell(0), self.cell(1), self.cell(2), self.cell(3), lvs[0]);
        println!("{0:<4} | {1:<4} | {2:<4} | {3:<4}| {4:<4}| {5:<4}", self.targets[5], self.cell(4), self.cell(5), self.cell(6), self.cell(7), lvs[1]);
        println!("{0:<4} | {1:<4} | {2:<4} | {3:<4}| {4:<4}| {5:<4}", self.targets[6], self.cell(8), self.cell(9), self.cell(10), self.cell(11), lvs[2]);
        println!("{0:<4} | {1:<4} | {2:<4} | {3:<4}| {4:<4}| {5:<4}", self.targets[7], self.cell(12), self.cell(13), self.cell(14), self.cell(15), lvs[3]);
        println!("     | {0:<4} | {1:<4} | {2:<4} | {3:<4}", lvs[4], lvs[5], lvs[6], lvs[7]);
    }
}

fn main() {
    let puz = Puzzle {
        mask : 0b0110111111110110,
        puzzle : vec![0,1,2,0,3,4,5,6,7,8,9,1,0,2,3,0],
        targets : vec![20, 6, 7, 24, 8, 14, 23, 12]
    };
    
    puz.display();
}