use rayon::{prelude::*, ThreadPoolBuilder};
use std::{
    collections::VecDeque,
    fmt::{Display, Write},
    io::{self, BufWriter},
    sync::{Arc, RwLock},
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

trait Rng: Display {
    fn next4(&mut self) -> usize;
}

struct LCGRng {
    x: usize,
    a: usize,
    b: usize,
    m: usize,
}

impl LCGRng {
    fn new(x: usize, a: usize, b: usize, m: usize) -> Self {
        Self { x, a, b, m }
    }
}

impl Rng for LCGRng {
    fn next4(&mut self) -> usize {
        self.x = (self.x * self.a + self.b) % self.m;
        self.x / 1000 % 4
    }
}

impl Display for LCGRng {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LCGRng({}, {}, {}, {})", self.x, self.a, self.b, self.m)
    }
}

struct Rng2Step<R> {
    rng: R,
    step: usize,
    prev: usize,
}

impl<R: Rng> Rng2Step<R> {
    fn new(mut rng: R) -> Self {
        let init = rng.next4();
        Self {
            rng,
            step: 0,
            prev: init,
        }
    }
}

impl<R: Rng> Rng for Rng2Step<R> {
    fn next4(&mut self) -> usize {
        if self.step % 2 == 1 {
            self.prev = self.rng.next4();
        } else {
            self.step += 1;
        }
        self.prev
    }
}

impl<R: Rng> Display for Rng2Step<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rng2Step({}, {})", self.rng, self.step)
    }
}

impl Env {
    fn try_randomwalk(&self, mut rng: impl Rng, steps: usize) -> Vec<Vec<bool>> {
        let mut r = self.sr;
        let mut c = self.sc;
        let mut visited = vec![vec![false; self.w]; self.h];
        visited[r][c] = true;

        for _ in 0..steps {
            let d = rng.next4();
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
            }
        }
        visited
    }

    fn try_randomwalk2(&self, mut rng: impl Rng, steps: usize) -> Vec<Vec<bool>> {
        let mut r = self.sr;
        let mut c = self.sc;
        let mut visited = vec![vec![false; self.w]; self.h];
        visited[r][c] = true;

        for _ in 0..steps {
            let d = rng.next4();
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
            }
        }
        visited
    }

    fn try_full_randomwalk(&self, mut rng: impl Rng) -> Result<usize, usize> {
        let mut r = self.sr;
        let mut c = self.sc;
        let mut visited = vec![vec![false; self.w]; self.h];
        visited[r][c] = true;
        let mut count = 1;

        for step in 1..=999000 {
            let d = rng.next4();
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

    fn randomwalk_with_fix(&self, vm: &mut Vec<Vec<bool>>, mut rng: impl Rng) {
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

        writeln!(iw, "{}", rng).unwrap();

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

            let d = rng.next4();
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

    ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();

    // let route = fs::read_to_string("out.txt").unwrap();
    // env.check(&route);

    struct MaxInfo {
        max: usize,
        maxa: usize,
        maxb: usize,
    }

    let mi = RwLock::new(MaxInfo {
        max: 0,
        maxa: 0,
        maxb: 0,
    });

    const M: usize = 2usize.pow(32);
    let a: Vec<usize> = (1..=999999).step_by(2).collect();
    a.into_par_iter().for_each(|a| {
        eprintln!("{}", a);
        for b in 1..=100 {
            let res = env.try_full_randomwalk(LCGRng::new(1, 1664524 + a, b, M));
            if let Ok(step) = res {
                println!("{} {}", a, b);
                println!("{}", step);
                return;
            } else if let Err(cnt) = res {
                let mut mi = mi.write().unwrap();
                if cnt > mi.max {
                    eprintln!("{} -> {} / {} ({}, {})", mi.max, cnt, env.all, a, b);
                    *mi = MaxInfo {
                        max: cnt,
                        maxa: a,
                        maxb: b,
                    };
                    let mut vm = env.try_randomwalk2(LCGRng::new(1, 1664524 + a, b, M), 999000);
                    env.randomwalk_with_fix(&mut vm, LCGRng::new(1, 1664524 + a, b, M));
                }
            }
        }
    });

    let mi = mi.read().unwrap();
    eprintln!("max: {} ({}, {})", mi.max, mi.maxa, mi.maxb);
}
