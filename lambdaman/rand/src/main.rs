use std::{
    collections::VecDeque,
    fs,
    io::{self, BufWriter},
};

fn input() -> (Vec<Vec<u8>>, usize, usize) {
    let mut input = String::new();
    let mut f = Vec::new();
    let mut sr = 0;
    let mut sc = 0;
    while let Ok(b) = io::stdin().read_line(&mut input) {
        if b == 0 || input.trim().len() == 0 {
            break;
        }
        if input.contains("L") {
            sc = input.find('L').unwrap();
            sr = f.len();
        }
        f.push(input.trim().replace('L', ".").into_bytes());
        input.clear();
    }
    (f, sr, sc)
}

struct Env {
    f: Vec<Vec<u8>>,
    all: usize,
    h: usize,
    w: usize,
    sr: usize,
    sc: usize,
}

const DIR: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];
const DIRC: [char; 4] = ['R', 'D', 'L', 'U'];

impl Env {
    fn try_randomwalk(&self, a: usize, b: usize, steps: usize) -> Vec<Vec<bool>> {
        let mut r = self.sr;
        let mut c = self.sc;
        let mut visited = vec![vec![false; self.w]; self.h];
        visited[r][c] = true;

        let mut rng = 1;
        for _ in 0..steps {
            rng = (rng * a + b) % 2usize.pow(32);
            let d = (rng / 1000) % 4;
            let (dr, dc) = DIR[d];
            let nr = r + dr;
            let nc = c + dc;
            if self.f[nr][nc] != b'.' {
                continue;
            }
            r = nr;
            c = nc;
            if !visited[r][c] {
                visited[r][c] = true;
            }
        }
        visited
    }

    fn try_randomwalk2(&self, a: usize, b: usize, steps: usize) -> Vec<Vec<bool>> {
        let mut r = self.sr;
        let mut c = self.sc;
        let mut visited = vec![vec![false; self.w]; self.h];
        visited[r][c] = true;

        let mut rng = 1;
        for _ in 0..steps / 2 {
            rng = (rng * a + b) % 2usize.pow(32);
            for _ in 0..2 {
                let d = (rng / 1000) % 4;
                let (dr, dc) = DIR[d];
                let nr = r + dr;
                let nc = c + dc;
                if self.f[nr][nc] != b'.' {
                    continue;
                }
                r = nr;
                c = nc;
                if !visited[r][c] {
                    visited[r][c] = true;
                }
            }
        }
        visited
    }

    fn try_full_randomwalk(&self, a: usize, b: usize) -> Result<usize, usize> {
        let mut r = self.sr;
        let mut c = self.sc;
        let mut visited = vec![vec![false; self.w]; self.h];
        visited[r][c] = true;
        let mut count = 1;

        let mut rng = 1;
        for step in 1..=999000 {
            if step % 2 == 1 {
                rng = (rng * a + b) % 2usize.pow(32);
            } else {
                // rng = rng + 1000;
            }
            let d = (rng / 1000) % 4;
            let (dr, dc) = DIR[d];
            let nr = r + dr;
            let nc = c + dc;
            if nr >= self.h || nc >= self.w || self.f[nr][nc] != b'.' {
                continue;
            }
            r = nr;
            c = nc;
            if !visited[r][c] {
                visited[r][c] = true;
                count += 1;
            }
            if count == self.all {
                return Ok(step);
            }
        }
        // eprintln!("failed: {} / {}", count, self.all);
        Err(count)
    }

    fn push_fix(&self, vm: &mut Vec<Vec<bool>>, fix: &mut VecDeque<usize>, r: usize, c: usize) {
        for d in 0..4 {
            let (dr, dc) = DIR[d];
            let nr = r + dr;
            let nc = c + dc;
            if nr >= self.h || nc >= self.w || self.f[nr][nc] != b'.' {
                continue;
            }
            if vm[nr][nc] {
                continue;
            }
            vm[nr][nc] = true;
            fix.push_back(d);
            self.push_fix(vm, fix, nr, nc);
            fix.push_back(d ^ 2);
        }
    }

    fn randomwalk_with_fix(&self, vm: &mut Vec<Vec<bool>>, a: usize, b: usize) {
        let mut r = self.sr;
        let mut c = self.sc;
        let mut visited = vec![vec![false; self.w]; self.h];
        let mut count = 1;
        visited[r][c] = true;

        use std::io::Write;
        let file = std::fs::File::create("out.txt").unwrap();
        let info_file = std::fs::File::create("info.txt").unwrap();
        let mut w = BufWriter::new(file);
        let mut iw = BufWriter::new(info_file);

        let mut rng = 1;
        let mut fix = VecDeque::new();
        let mut step = 1000000;
        while step >= 1 {
            if !fix.is_empty() {
                let d: usize = fix.pop_front().unwrap();
                let (dr, dc) = DIR[d];
                (r, c) = (r + dr, c + dc);
                if !visited[r][c] {
                    visited[r][c] = true;
                    count += 1;
                }
                // println!("{} {}", step, DIRC[d]);
                write!(w, "{}", DIRC[d]).unwrap();
                step -= 1;
                continue;
            }
            self.push_fix(vm, &mut fix, r, c);
            if !fix.is_empty() {
                let dirs = fix.iter().map(|&d| DIRC[d]).collect::<String>();
                writeln!(iw, "{} {}", step, dirs).unwrap();
                let d: usize = fix.pop_front().unwrap();
                let (dr, dc) = DIR[d];
                (r, c) = (r + dr, c + dc);
                if !visited[r][c] {
                    visited[r][c] = true;
                    count += 1;
                }
                // println!("{} {}", step, DIRC[d]);
                write!(w, "{}", DIRC[d]).unwrap();
                step -= 1;
                continue;
            }

            rng = (rng * a + b) % 2usize.pow(32);
            for _ in 0..2 {
                let d = (rng / 1000) % 4;
                let (dr, dc) = DIR[d];
                let nr = r + dr;
                let nc = c + dc;
                write!(w, "{}", DIRC[d]).unwrap();
                step -= 1;
                if nr >= self.h || nc >= self.w || self.f[nr][nc] != b'.' {
                    continue;
                }
                r = nr;
                c = nc;
                if !visited[r][c] {
                    visited[r][c] = true;
                    count += 1;
                }
                if count == self.all {
                    writeln!(iw, "{}", step).unwrap();
                    return;
                }
            }
        }
    }

    fn check(&self, route: impl AsRef<str>) {
        let route = route.as_ref();
        let mut r = self.sr;
        let mut c = self.sc;
        let mut count = 1;
        let mut visited = vec![vec![false; self.w]; self.h];
        visited[r][c] = true;
        for d in route.chars() {
            let d = match d {
                'R' => 0,
                'D' => 1,
                'L' => 2,
                'U' => 3,
                _ => unreachable!(),
            };
            let (dr, dc) = DIR[d];
            let nr = r + dr;
            let nc = c + dc;
            if nr >= self.h || nc >= self.w || self.f[nr][nc] != b'.' {
                continue;
            }
            r = nr;
            c = nc;
            if !visited[r][c] {
                visited[r][c] = true;
                count += 1;
            }
        }
        eprintln!("check: {} / {}", count, self.all);
    }
}

fn main() {
    let (f, sr, sc) = input();
    let h = f.len();
    let w = f[0].len();
    let all = f
        .iter()
        .map(|r| r.iter().filter(|&&c| c == b'.').count())
        .sum();
    let env = Env {
        f,
        all,
        h,
        w,
        sr,
        sc,
    };
    eprintln!("{} {}", sr, sc);

    // let route = fs::read_to_string("out.txt").unwrap();
    // env.check(&route);

    // let mut max = 0;
    // let mut maxa = 0;
    // let mut maxb = 0;
    // for a in (10001..20000).step_by(2) {
    //     eprintln!("{}", a);
    //     for b in 1..=100 {
    //         let res = env.try_full_randomwalk(1664524 + a, b);
    //         if let Ok(step) = res {
    //             println!("{} {}", a, b);
    //             println!("{}", step);
    //             return;
    //         } else if let Err(cnt) = res {
    //             if cnt > max {
    //                 eprintln!("{} -> {} / {} ({}, {})", max, cnt, env.all, a, b);
    //                 max = cnt;
    //                 maxa = a;
    //                 maxb = b;
    //                 let mut vm = env.try_randomwalk2(1664524 + maxa, maxb, 999000);
    //                 env.randomwalk_with_fix(&mut vm, 1664524 + maxa, maxb);
    //             }
    //         }
    //     }
    // }
    // eprintln!("max: {} ({}, {})", max, maxa, maxb);

    let maxa = 18541;
    let maxb = 30;

    eprintln!("{:?}", env.try_full_randomwalk(1664524 + maxa, maxb));

    let mut vm = env.try_randomwalk2(1664524 + maxa, maxb, 999000);
    env.randomwalk_with_fix(&mut vm, 1664524 + maxa, maxb);

    // let mut vismap1 = vec![];
    // for a in (1..=100).step_by(2) {
    //     eprintln!("MAP1 {}", a);
    //     for b in 1..=100 {
    //         let vismap = env.try_randomwalk(a, b, 250000);
    //         vismap1.push((a, b, vismap));
    //     }
    // }
    // let mut vismap2 = vec![];
    // for a in (101..=150).step_by(2) {
    //     eprintln!("MAP2 {}", a);
    //     for b in 101..=200 {
    //         let vismap = env.try_randomwalk(a, b, 500000);
    //         vismap2.push((a, b, vismap));
    //     }
    // }

    // for (a1, b1, vm1) in vismap1 {
    //     eprintln!("{} {}", a1, b1);
    //     for (a2, b2, vm2) in &vismap2 {
    //         let mut ok = true;
    //         'outer: for r in 0..h {
    //             for c in 0..w {
    //                 if env.f[r][c] == b'.' && !vm1[r][c] && !vm2[r][c] {
    //                     ok = false;
    //                     break 'outer;
    //                 }
    //             }
    //         }
    //         if ok {
    //             println!("{} {}", a1, b1);
    //             println!("{} {}", a2, b2);
    //             return;
    //         }
    //     }
    // }
}
